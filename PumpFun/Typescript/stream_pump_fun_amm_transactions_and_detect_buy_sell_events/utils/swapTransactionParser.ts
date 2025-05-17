export function parseSwapTransactionOutput(parsedInstruction, transaction) {
    const SOL_MINT = 'So11111111111111111111111111111111111111112';
    let output = {};

    const swapInstruction = parsedInstruction.instructions.pumpFunIxs.find(
        (instruction) => instruction.name === 'buy' || instruction.name === 'sell'
    );

    if (!swapInstruction) {
        return;
    }

    const signerPubkey = swapInstruction.accounts.find((account) => account.name === 'user')?.pubkey;
    const user_quote_token_account = swapInstruction.accounts.find((account) => account.name === "user_quote_token_account")?.pubkey;
    const pool_base_token_account = swapInstruction.accounts.find((account) => account.name === "pool_base_token_account")?.pubkey;

    const swapAmount = swapInstruction.name === 'sell'
        ? swapInstruction.args?.base_amount_in
        : swapInstruction.args?.base_amount_out;

    const quoteAmount = swapInstruction.name === 'sell'
        ? swapInstruction.args?.min_quote_amount_out
        : swapInstruction.args?.max_quote_amount_in;

    const determineOutAmount = () => {
        if (!transaction.meta.innerInstructions) {
            console.error("No inner instructions found in transaction");
            return null;
        }
         const transferChecked = parsedInstruction.inner_ixs.find(
         (instruction) =>
         instruction.name === 'transferChecked' && instruction.args?.amount !== swapAmount).args?.amount;
          return transferChecked;
    };
    const determineBuySellEvent = () => {
        const baseMintPubkey = swapInstruction.accounts.find((account) => account.name === 'base_mint')?.pubkey;
        const quoteMintPubkey = swapInstruction.accounts.find((account) => account.name === 'quote_mint')?.pubkey;

        if (!baseMintPubkey || !quoteMintPubkey) {
            console.error("Base or quote mint not found in swap accounts");
            return { type: "Unknown", mint: null };
        }

        const mint = baseMintPubkey === SOL_MINT ? quoteMintPubkey : baseMintPubkey;
        const eventType = swapInstruction.name === 'buy' ? "Buy" : "Sell";

        return { type: eventType, mint };
    };

    const buySellEvent = determineBuySellEvent();
  const base_amount_in = swapInstruction.name === 'sell'
        ? swapInstruction.args?.base_amount_in
        : swapInstruction.args?.base_amount_out;
     
    const amountIn = swapInstruction.name === 'buy'
        ? determineOutAmount()
        : base_amount_in;

 const amountOut = swapInstruction.name === 'sell'
        ? determineOutAmount()
        : base_amount_in;
    const transactionEvent = {
        type: buySellEvent.type,
        user: signerPubkey,
        mint: buySellEvent.mint,
        out_amount: amountOut,
        in_amount: amountIn, 
    };


    output = {
        ...transaction,
        meta: {
            ...transaction.meta,
            innerInstructions: parsedInstruction.inner_ixs,
        },
        transaction: {
            ...transaction.transaction,
            message: {
                ...transaction.transaction.message,
                compiledInstructions: parsedInstruction.instructions,
            },
        }
    };

    return { output, transactionEvent };
}