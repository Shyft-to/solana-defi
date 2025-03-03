export function transactionOutput(txn){
 const instructions =  txn.instructions[0];//console.log(txn);
 const events = txn.events[0];
 const name = instructions.name;
 const accounts = instructions.accounts;
 const args = instructions.args;
 const data = events.data;
return {
    name,
    accounts,
    args,
    data
}
}