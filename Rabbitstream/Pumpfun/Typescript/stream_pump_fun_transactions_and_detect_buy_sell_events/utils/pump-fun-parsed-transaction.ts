export function pumpFunParsedTransaction(parsedInstruction,txn){
  const swapInstruction = parsedInstruction.inner_ixs.find((x)=> x.name === "buy" || x.name === "sell")
  if(!swapInstruction) return;
   const { name: type, accounts = [], args = {} } = swapInstruction;
  const baseAmountIn = args?.amount;

  const bondingCurve = accounts.find(a => a.name === 'bonding_curve')?.pubkey;
  const userPubkey = accounts.find(a => a.name === 'user')?.pubkey;
  const mint = accounts.find(a => a.name === 'mint')?.pubkey;

  return {
    type,
    user: userPubkey,
    mint,
    bonding_curve: bondingCurve,
    in_amount: baseAmountIn,
  };
}