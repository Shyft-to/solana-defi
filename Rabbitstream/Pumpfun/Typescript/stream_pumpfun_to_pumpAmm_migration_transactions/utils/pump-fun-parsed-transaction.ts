export function pumpFunParsedTransaction(parsedInstruction,txn){
  const filterMintTransactions = parsedInstruction.inner_ixs.find((x)=> x.name === "migrate")

  return filterMintTransactions;
}