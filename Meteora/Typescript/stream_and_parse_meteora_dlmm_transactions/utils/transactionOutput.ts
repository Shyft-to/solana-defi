export function transactionOutput(parsedInstruction,txn){
  const instructions = parsedInstruction.instructions;
  const slot = txn.slot;
  const version = txn.version;
  const blockTime = txn.blockTime;
  const fee = txn.meta.fee;
  const err = txn.meta.err;
  const preBalances = JSON.stringify(txn.meta.preBalances || []);  // Stringify array
  const postBalances = JSON.stringify(txn.meta.postBalances || []);
  const preTokenBalances = JSON.stringify(txn.meta.preTokenBalances || []);
  const postTokenBalances = JSON.stringify(txn.meta.postTokenBalances || []);
  const logMessages = JSON.stringify(txn.meta.logMessages || []);
  const loadedAddresses = JSON.stringify(txn.meta.loadedAddresses || {});
  //const innerInstructions = JSON.stringify(txn.meta.innerInstructions || []);
  // Extracting data from transaction object
  const signatures = JSON.stringify(txn.transaction.signatures || []);
  const message = JSON.stringify(txn.transaction.message || {});
  let innerInstructions;

  if (instructions.length > 1) {
    innerInstructions = JSON.stringify({
      nameA: instructions[0].name,
      nameB: instructions[1].name,
      accountsA: instructions[0].accounts,
      accountsB: instructions[1].accounts,
      argsA: instructions[0].args,
      argsB: instructions[1].args
    });
  } else {
    innerInstructions = JSON.stringify({
      name: instructions[0].name,
      accounts: instructions[0].accounts,
      args: instructions[0].args
    });
  }
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
    innerInstructions, // Now contains formatted instruction details
    signatures,
    message
  };

  return output;
 
}
