export function meteoraPoolTxn(parsedInstruction: any, txn: any) {
  const swapInstruction = parsedInstruction.instructions.find(
    (ix: any) => ix.name === "swap"
  );
  if (!swapInstruction) return;

  const tradeEvent = parsedInstruction.inner_ixs?.events?.find(
    (e: any) => e.name === "TradeEvent"
  );
  if (!tradeEvent) return;

  const payer = swapInstruction.accounts.find((a: any) => a.name === "user")?.pubkey?.toString();

  const preTokenBalances = txn.meta?.preTokenBalances ?? [];
  const postTokenBalances = txn.meta?.postTokenBalances ?? [];

  const userPre = preTokenBalances.filter((b: any) => b.owner === payer);

  let tokenIn: string | undefined;
  let tokenOut: string | undefined;
  let side : boolean | undefined;
  for (const pre of userPre) {
    const post = postTokenBalances.find((p: any) => p.accountIndex === pre.accountIndex);
    if (!post) continue;

    const preBal = BigInt(pre.uiTokenAmount.amount);
    const postBal = BigInt(post.uiTokenAmount.amount);

    if (preBal > postBal){tokenIn = pre.mint; side = true};   
    if (postBal > preBal){tokenOut = post.mint; side = false};
  }
  const type = side ? "Sell" : "Buy"; 

  return {
    type,
    user: payer,
    tokenIn,
    tokenOut,
    amountIn: tradeEvent.data?.amountIn?.toString(),
    amountOut: tradeEvent.data?.amountOut?.toString(),
  };
}