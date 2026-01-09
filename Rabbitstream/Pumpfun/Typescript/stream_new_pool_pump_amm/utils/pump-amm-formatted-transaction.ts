export function pumpSwapParsedTransaction(parsedInstruction,txn){
    let output = {};
    const instruction_name = parsedInstruction?.inner_ixs.find((x)=> x.name === 'create_pool')
    if(!instruction_name)return;
     output = {
      ...txn,
      meta: {
        ...txn.meta,
      },
      transaction: {
        ...txn.transaction,
        message: {
          ...txn.transaction.message,
          instructions : parsedInstruction?.instructions,
          compiledInstructions: parsedInstruction?.inner_ixs,
        },
      }
    }


  return output;
}