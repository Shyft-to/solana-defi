export function rl_formatter(parsedInstruction,txn){
  let output = {};
  const instructions = parsedInstruction.instructions.find((x)=> x.name === "migrate_to_amm" || x.name === "migrate_to_cpswap");
   if(!instructions) return;
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