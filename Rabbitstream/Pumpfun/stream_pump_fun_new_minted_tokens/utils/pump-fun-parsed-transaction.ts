export function pumpFunParsedTransaction(parsedInstruction,txn){
  const filterMintTransactions = parsedInstruction.inner_ixs.find((x)=> x.name === "create")
  if(!filterMintTransactions) return;
  const args = filterMintTransactions.args;
  const name = args.name;
  const symbols = args.symbol;
  const url = args.uri;
  const creator = args.creator
  const mint = filterMintTransactions.accounts.find((ix)=> ix.name == "mint").pubkey;
  const bondingCurve = filterMintTransactions.accounts.find((ix) => ix.name == "bonding_curve").pubkey;
  let output = {};
    output = {
     Name : name,
     Symbol : symbols,
     Uri : url,
     Mint: mint,
     Bonding_Curve: bondingCurve,
     Creator: creator,
    }

  return output;
}