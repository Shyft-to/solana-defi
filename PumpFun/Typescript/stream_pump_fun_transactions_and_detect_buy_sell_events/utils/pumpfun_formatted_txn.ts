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

  const transferInstruction = innerInstructions.find(
    ix => ix.name === 'transfer' && ix.args.amount !== baseAmountIn
  );
  const alternativeAmountOut = innerInstructions.find(
    ix =>
      ix.name === 'transfer' &&
      ix.args.amount !== baseAmountIn &&
      ix.accounts.some(acct => acct.pubkey === bondingCurve)
  )?.args?.lamports;
  const solEventAmount = parsedInstruction?.events?.[0]?.data?.solAmount;
  const outAmount = solEventAmount ?? alternativeAmountOut;


  const isBuy = type === 'buy';
  const inAmount = isBuy ? outAmount : baseAmountIn;
  const finalOutAmount = isBuy ? baseAmountIn : outAmount;

  return {
    type,
    user: userPubkey,
    mint,
    bonding_curve: bondingCurve,
    in_amount: inAmount,
    out_amount: finalOutAmount,
  };
}
