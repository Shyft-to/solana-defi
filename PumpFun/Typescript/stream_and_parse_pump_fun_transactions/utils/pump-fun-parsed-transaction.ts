export function pumpFunParsedTransaction(parsedInstruction,txn){
  let output = {};
  let inner_ixs = parsedInstruction.inner_ixs;
  let events = parsedInstruction.events;
    output = {
      ...txn,
      meta: {
        ...txn.meta,
        innerInstructions: {inner_ixs, events},
      },
      transaction: {
        ...txn.transaction,
        message: {
          ...txn.transaction.message,
          instructions : parsedInstruction.instructions,
          compiledInstructions: parsedInstruction.instructions,
        },
      }
    }
  

  return output;
}