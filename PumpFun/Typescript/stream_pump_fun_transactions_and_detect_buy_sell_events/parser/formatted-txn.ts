export function parseSwapTransactionOutput(parsedInstruction) {
  const innerInstructions = parsedInstruction.inner_ixs ?? [];

  const swapInstruction = innerInstructions.find(
    (ix) => ix.name === 'buy' || ix.name === 'sell'
  );
  
  if (!swapInstruction) return;
  const { name: type, accounts = [], args = {} } = swapInstruction;
  const baseAmountIn = args?.amount;

  const bondingCurve = accounts.find(a => a.name === 'bondingCurve')?.pubkey;
  const userPubkey = accounts.find(a => a.name === 'user')?.pubkey;
  const mint = accounts.find(a => a.name === 'mint')?.pubkey;

  const alternativeAmountOut = innerInstructions.find(
    ix =>
      ix.name === 'transfer' &&
      ix.args.amount !== baseAmountIn &&
      ix.accounts.some(acct => acct.pubkey === bondingCurve)
  )?.args?.lamports;
  const tradeEvent = parsedInstruction?.event?.find(e => e.name === 'TradeEvent');
  const solEventAmount = tradeEvent?.data?.sol_amount;
  const tokenEventAmount = tradeEvent?.data?.token_amount;

  const isBuy = type === 'buy';
  let inAmount = isBuy ? solEventAmount : tokenEventAmount;
  let finalOutAmount = isBuy ? tokenEventAmount : (solEventAmount ?? alternativeAmountOut);
  return {
    type,
    user: userPubkey,
    mint,
    bonding_curve: bondingCurve,
    in_amount: inAmount,
    out_amount: finalOutAmount,
  };
}
