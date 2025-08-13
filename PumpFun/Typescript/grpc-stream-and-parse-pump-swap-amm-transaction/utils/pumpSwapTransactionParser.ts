export function parseSwapTransactionOutput(parsedInstruction, transaction) {
    let output = {};
  //console.log("TXN", parsedInstruction);
    output = {
        ...transaction,
        meta: {
            ...transaction.meta,
            innerInstructions: parsedInstruction.inner_instructions,
        },
        transaction: {
            ...transaction.transaction,
            message: {
                ...transaction.transaction.message,
                compiledInstructions: parsedInstruction.instructions,
            },
        }
    };

    return { output };
}