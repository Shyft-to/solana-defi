export function meteoradammTransactionOutput(parsedInstruction,txn){
  let output = {};
  const instruction_name = parsedInstruction.inner_ixs.meteora_damm_inner_ixs.find((x)=> x.name === 'initialize_pool');
  if(!instruction_name)return;
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