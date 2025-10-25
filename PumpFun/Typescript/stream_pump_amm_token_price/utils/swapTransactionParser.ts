export function parseSwapTransactionOutput(parsedInstruction, txn) {
    const SOL_MINT = 'So11111111111111111111111111111111111111112';

      let swapInstruction = 
        parsedInstruction?.instructions?.pumpAmmIxs?.find(
            instruction => instruction.name === 'buy' || instruction.name === 'sell'
        ) ||
        parsedInstruction?.inner_ixs?.find(
            instruction => instruction.name === 'buy' || instruction.name === 'sell'
        ) ||
        parsedInstruction?.inner_ixs?.pump_amm_inner_ixs?.find(
            instruction => instruction.name === 'buy' || instruction.name === 'sell'
        );
    if (!swapInstruction) return;

    let baseMintPubkey = swapInstruction?.accounts?.find((account) => account.name === 'base_mint')?.pubkey;
    let quoteMintPubkey = swapInstruction?.accounts?.find((account) => account.name === 'quote_mint')?.pubkey;
    [baseMintPubkey, quoteMintPubkey] = getValidMints(
        baseMintPubkey,
        quoteMintPubkey,
        txn?.meta?.preTokenBalances || [],
    );
    const parsedEvent = parsedInstruction?.events?.find(e => e.name === 'BuyEvent' || e.name === 'SellEvent').data;
    const pool_base_token_reserves = parsedEvent?.pool_base_token_reserves;
    const pool_quote_token_reserves = parsedEvent?.pool_quote_token_reserves;

    const decimal = txn.meta?.preTokenBalances?.find(
        (b) => b.mint === quoteMintPubkey || b.mint === baseMintPubkey
    )?.uiTokenAmount?.decimals || 9;

    let price;
    if (baseMintPubkey === SOL_MINT) {
        price = calculatePumpAmmPrice(pool_base_token_reserves, pool_quote_token_reserves, decimal);
    } else {
        price = calculatePumpAmmPrice(pool_quote_token_reserves, pool_base_token_reserves, decimal);
    }

    const formattedPrice = price.toFixed(20).replace(/0+$/, '');
    return {
        base_mint: baseMintPubkey,
        quote_mint: quoteMintPubkey,
        pool_base_token_reserver: pool_base_token_reserves,
        pool_quote_token_reserver: pool_quote_token_reserves,
        price: formattedPrice + " SOL",
    };
}


function calculatePumpAmmPrice(
    pool_base_reserve: number,
    pool_quote_reserve: number,
    decimal : number
): number {

    const base = pool_base_reserve/ 1_000_000_000;;
    const quote = pool_quote_reserve/ Math.pow(10, decimal);
    return base / quote;
}
function getValidMints(baseMint, quoteMint, preTokenBalances = []) {
    const invalidPattern = /=|[^A-Za-z0-9]/g;
    const SOL_MINT = 'So11111111111111111111111111111111111111112';

    const isValid = (mint) => mint && !invalidPattern.test(mint);

    if (isValid(baseMint) && isValid(quoteMint)) {
        return [baseMint, quoteMint];
    }

    const uniqueMints = [...new Set(preTokenBalances.map((b) => b.mint))].filter(isValid);

    if (!isValid(baseMint)) {
        baseMint = uniqueMints.find((m) => m !== quoteMint) || SOL_MINT;
    }

    if (!isValid(quoteMint)) {
        quoteMint = uniqueMints.find((m) => m !== baseMint) || SOL_MINT;
    }

    return [baseMint, quoteMint];
}
