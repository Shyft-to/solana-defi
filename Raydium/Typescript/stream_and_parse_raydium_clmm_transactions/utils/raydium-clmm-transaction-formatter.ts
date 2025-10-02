export function raydiumClmmFormatter(parsedInstruction, txn){
  let output = {};

  const instructions = parsedInstruction.instructions || parsedInstruction.raydiumClmmIxs;
  const innerInstructions = parsedInstruction.inner_ixs || parsedInstruction.innerIx || parsedInstruction.innerInstructions;
  const events = parsedInstruction.events;
  if(txn.version === 0){
    output = {
      ...txn,
      meta: {
        ...txn.meta,
        innerInstructions: innerInstructions,
      },
      transaction: {
        ...txn.transaction,
        message: {
          ...txn.transaction.message,
          compiledInstructions: {instructions, events},
        },
      }
    }
  }
  else {
    output = {
      ...txn,
      meta: {
        ...txn.meta,
        innerInstructions: innerInstructions,
      },
      transaction: {
        ...txn.transaction,
        message: {
          ...txn.transaction.message,
          instructions: instructions,
        },
      }
    }
  }

  return output;
}