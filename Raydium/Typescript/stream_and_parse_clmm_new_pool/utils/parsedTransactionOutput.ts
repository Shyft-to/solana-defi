export function parsedTransactionOutput(parsedInstruction,txn){
  let output = {};
  const name = parsedInstruction.instructions.find(
    (instruction) => instruction.name === 'createPool'
  );
   if(!name) return;
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
  console.log("parsedInstruction", name);

  return output;
}