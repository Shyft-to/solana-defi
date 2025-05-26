export function parseSwapTransactionOutput(parsedInstruction,txn) {
    let price;
    const SOL_MINT = 'So11111111111111111111111111111111111111112';
    const decimal = txn.meta?.preTokenBalances.find(
        (instruction) => instruction.mint != SOL_MINT     
    ).uiTokenAmount?.decimals;
    const swapInstruction = parsedInstruction.instructions.pumpFunIxs.find(
        (instruction) => instruction.name === 'buy' || instruction.name === 'sell'
    );

    if (!swapInstruction) {
        return;
    }
    const baseMintPubkey = swapInstruction.accounts.find((account) => account.name === 'base_mint')?.pubkey;
    const quoteMintPubkey = swapInstruction.accounts.find((account) => account.name === 'quote_mint')?.pubkey;

    const parsedEvent = parsedInstruction.instructions.events[0]?.data;
    const pool_base_token_reserves = parsedEvent.pool_base_token_reserves;
    const pool_quote_token_reserves = parsedEvent.pool_quote_token_reserves;
    if(baseMintPubkey === SOL_MINT){
        price = calculatePumpAmmPrice(
            pool_base_token_reserves,
            pool_quote_token_reserves,
            decimal
        );
    }else {
        price = calculatePumpAmmPrice(
            pool_quote_token_reserves,
            pool_base_token_reserves,
            decimal
        );
    }
    const formattedPrice = price.toFixed(20).replace(/0+$/, ''); 
    const output = {
        base_mint : baseMintPubkey,
        quote_mint: quoteMintPubkey,
        pool_base_token_reserver: pool_base_token_reserves,
        pool_quote_token_reserver: pool_quote_token_reserves,
        price: formattedPrice + " SOL",
    }
    return output;

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