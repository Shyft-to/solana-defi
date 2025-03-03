export function transactionOutput(txn){
  const instructions = txn.instructions;
  if(instructions.length > 1){
    const nameA = instructions[0].name;
    const nameB = instructions[1].name;
    const accountsA = instructions[0].accounts;
    const accountsB = instructions[1].accounts;
    const argsA = instructions[0].args;
    const argsB = instructions[1].args;
    return{
        nameA,
        nameB,
        accountsA,
        accountsB,
        argsA,
        argsB
    }
  }else{
    const name = instructions[0].name;
    const account = instructions[0].accounts;
    const args = instructions[0].args;
    return{
        name,
        account,
        args
    }
  }
}