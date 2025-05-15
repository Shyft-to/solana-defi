export function pumpFunParsedTransaction(parsedInstruction,txn){
  const instructions = parsedInstruction.instructions.find((x)=> x.name === "create");
   if(!instructions) return;
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
          instructions : parsedInstruction.instructions,
          compiledInstructions: parsedInstruction.instructions,
        },
      }
    }
  

  return output;
}