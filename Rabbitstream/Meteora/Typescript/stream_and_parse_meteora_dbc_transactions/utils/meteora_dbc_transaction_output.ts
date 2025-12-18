export function meteoradbcTransactionOutput(parsedInstruction,txn){
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