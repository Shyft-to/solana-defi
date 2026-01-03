export function meteoradammTransactionOutput(parsedInstruction,txn){
  let output = {};

  if(txn.version === 0){
    output = {
      ...txn,
      meta: {
        ...txn.meta,
      },
      transaction: {
        ...txn.transaction,
        message: {
          ...txn.transaction.message,
          compiledInstructions: parsedInstruction.inner_ixs,
        },
      }
    }
  }
  else {
    output = {
      ...txn,
      meta: {
        ...txn.meta,
      },
      transaction: {
        ...txn.transaction,
        message: {
          ...txn.transaction.message,
          instructions: parsedInstruction.inner_ixs,
        },
      }
    }
  }

  return output;
}