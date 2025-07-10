export function meteoradbcTransactionOutput(parsedInstruction,txn){
  const instructions = parsedInstruction.instructions.find((x)=>
  x.name === "initialize_virtual_pool_with_spl_token" 
  || x.name == "initialize_virtual_pool_with_token2022" 
  || x.name === "create_config");
   if(!instructions) return;  
   let output = {};

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