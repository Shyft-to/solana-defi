export function parseSwapTransactionOutput(parsedInstruction) {
  if (!parsedInstruction) return;
  
  const instructions = parsedInstruction.inner_ixs;

  const swapInstruction = instructions.find(ix =>
    ix?.name === 'buy' ||
    ix?.name === 'sell' ||
    ix?.name === 'buy_exact_sol_in'
  );
  

  if (!swapInstruction) return;

  const { name: type, accounts = [], args = {} } = swapInstruction;

  const user = accounts.find(a => a.name === 'user')?.pubkey;
  const mint = accounts.find(a => a.name === 'mint')?.pubkey;
  const bondingCurve = accounts.find(a => a.name === 'bonding_curve')?.pubkey;
  const inAmount = args.amount ?? args.spendable_sol_in;
  
  return {
    type,
    user,
    mint,
    bonding_curve: bondingCurve,
    in_amount: inAmount,
  };
}
