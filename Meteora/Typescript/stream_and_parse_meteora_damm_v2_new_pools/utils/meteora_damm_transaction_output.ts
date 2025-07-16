export function meteoradammTransactionOutput(parsedInstruction,txn){
  let output = {};
  const instruction_name = parsedInstruction.instructions.find((x)=> x.name === 'initialize_pool');
  if(!instruction_name)return;
  if(txn.version === 0){
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
  }
  else {
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
          instructions: parsedInstruction.instructions,
        },
      }
    }
  }

  return output;
}