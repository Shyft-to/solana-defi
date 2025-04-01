export function transactionOutput(parsedInstruction,txn){
  let output = {};

    output = {
      ...txn,
      meta: {
        ...txn.meta,
        innerInstructions: parsedInstruction.inner_ixs,
      },
      transaction: {
        ...txn.transaction,
        message: {
          ...txn.transaction.message,
          compiledInstructions: parsedInstruction.instructions,
        },
      }
    }
  

  return output;
}