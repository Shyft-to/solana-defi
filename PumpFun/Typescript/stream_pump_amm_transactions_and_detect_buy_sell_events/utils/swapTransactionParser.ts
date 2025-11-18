export function parseSwapTransactionOutput(parsedInstruction, transaction) {
    const SOL_MINT = 'So11111111111111111111111111111111111111112';

    if (!parsedInstruction || !transaction?.meta) {
        return;
    }

    const swapInstruction =
        parsedInstruction?.instructions?.pumpAmmIxs?.find(ix => ix.name === 'buy' || ix.name === 'sell') ||
        parsedInstruction?.inner_ixs?.find(ix => ix.name === 'buy' || ix.name === 'sell') ||
        parsedInstruction?.inner_ixs?.pump_amm_inner_ixs?.find(ix => ix.name === 'buy' || ix.name === 'sell');

    if (!swapInstruction) return;

    const isSell = swapInstruction.name === 'sell';
    const isBuy = swapInstruction.name === 'buy';

    const evt = parsedInstruction?.instructions?.events?.[0]?.data ?? {};
    const {
        base_amount_in,
        quote_amount_out,
        user_base_token_reserves,
        user_quote_token_reserves,
        pool_base_token_reserves,
        pool_quote_token_reserves,
        coin_creator,
    } = evt;


    const swapAmount = isSell
        ? swapInstruction.args?.base_amount_in
        : swapInstruction.args?.base_amount_out;

    const quoteAmount = isSell
        ? swapInstruction.args?.min_quote_amount_out
        : swapInstruction.args?.max_quote_amount_in;


    function determineOutAmount() {
        if (!transaction?.meta?.innerInstructions) return null;

        const transferCheckedIx = parsedInstruction.inner_ixs?.find(ix =>
            ix.name === 'transferChecked' && ix.args?.amount !== swapAmount
        );

        return transferCheckedIx?.args?.amount ?? null;
    }

    function determineBuySellEvent() {
        const baseMint = swapInstruction.accounts?.find(a => a.name === 'base_mint')?.pubkey;
        const quoteMint = swapInstruction.accounts?.find(a => a.name === 'quote_mint')?.pubkey;

        if (!baseMint || !quoteMint) {
            return { type: "Unknown", mint: null };
        }

        const mint = baseMint === SOL_MINT ? quoteMint : baseMint;
        return { type: isBuy ? "Buy" : "Sell", mint };
    }

    const buySellEvent = determineBuySellEvent();


    const computedOut = determineOutAmount();

    const amountIn = isBuy ? computedOut : swapInstruction.args?.base_amount_in;
    const amountOut = isSell ? computedOut : swapInstruction.args?.base_amount_out;


    const alternativeMint = transaction.meta.preTokenBalances?.find(x => x.mint !== SOL_MINT)?.mint;

    const signerPubkey =
        swapInstruction?.accounts?.find(a => a.name === 'user')?.pubkey ?? coin_creator;


    return {
        type: swapInstruction.name,
        user: signerPubkey,
        mint: buySellEvent.mint ?? alternativeMint,
        out_amount: amountOut ?? quote_amount_out,
        in_amount: amountIn ?? base_amount_in,
        user_base_token_reserves,
        user_quote_token_reserves,
        pool_base_token_reserves,
        pool_quote_token_reserves,
    };
}
