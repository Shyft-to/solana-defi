export function pumpFunParsedTransaction(parsedInstruction,txn){
  let output = {};
    output = {
      ...txn,
      meta: {
        ...txn.meta,
      },
      transaction: {
        ...txn.transaction,
        message: {
          ...txn.transaction.message,
          instructions : parsedInstruction.instructions,
          compiledInstructions: parsedInstruction.inner_ixs,
        },
      }
    }
  

  return output;
}