export function transactionOutput(parsedInstruction,txn){
    const slot = txn.slot;
    const version = txn.version;
    const blockTime = txn.blockTime;
    const fee = txn.meta.fee;
    const err = txn.meta.err;
    const preBalances = JSON.stringify(txn.meta.preBalances || []);  
    const postBalances = JSON.stringify(txn.meta.postBalances || []);
    const preTokenBalances = JSON.stringify(txn.meta.preTokenBalances || []);
    const postTokenBalances = JSON.stringify(txn.meta.postTokenBalances || []);
    const logMessages = JSON.stringify(txn.meta.logMessages || []);
    const loadedAddresses = JSON.stringify(txn.meta.loadedAddresses || {});
    const signatures = JSON.stringify(txn.transaction.signatures || []);
    const message = JSON.stringify(txn.transaction.message || {});
    const instructions =  parsedInstruction.instructions[0];
    const events = parsedInstruction.events[0];
    const name = instructions.name;
    const accounts = instructions.accounts;
    const args = instructions.args;
    const data = events.data;
    let innerInstructions = JSON.stringify({
        name: name,
        accounts: accounts,
        args: args,
        events :data
      });
    
    const output = {
      slot,
      version,
      blockTime,
      fee,
      err,
      preBalances,
      postBalances,
      preTokenBalances,
      postTokenBalances,
      logMessages,
      loadedAddresses,
      innerInstructions,
      signatures,
      message
    };
  
    return output;
}