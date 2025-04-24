export function parseSwapTransactionOutput(parsedInstruction, transaction) {
    const SOL_MINT = 'So11111111111111111111111111111111111111112';
    let output = {};

    const swapInstruction = parsedInstruction.instructions.find(
        (instruction) => instruction.name === 'buy' || instruction.name === 'sell'
    );

    if (!swapInstruction) {
        console.error("Instruction with name 'buy' or 'sell' not found");
        return transaction;
    }

    const signerPubkey = swapInstruction.accounts.find((account) => account.name === 'user')?.pubkey;

    // Determine the amount based on the instruction type
    const swapAmount = swapInstruction.name === 'sell'
        ? swapInstruction.args?.base_amount_in
        : swapInstruction.args?.base_amount_out;

    const quoteAmount = swapInstruction.name === 'sell'
        ? swapInstruction.args?.min_quote_amount_out
        : swapInstruction.args?.max_quote_amount_in;

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

    const transactionEvent = {
        type: buySellEvent.type,
        user: signerPubkey,
        mint: buySellEvent.mint,
        amount: swapAmount,
        quoteAmount: quoteAmount,
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
