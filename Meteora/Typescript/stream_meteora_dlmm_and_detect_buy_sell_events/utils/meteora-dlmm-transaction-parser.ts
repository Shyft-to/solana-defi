export function meteoraDlmmParsedTransaction(parsedInstruction: any): any {
  let output = {};

  const swapTransaction =
    parsedInstruction?.inner_ixs?.find(
      (ix: any) => ix.name === "swap" || ix.name === "swap2"
    ) ??
    parsedInstruction?.instructions?.meteoraDlmmIxs?.find(
      (ix: any) => ix.name === "swap" || ix.name === "swap2"
    );

  const eventTransaction = parsedInstruction?.instructions?.events?.find(
    (ev: any) => ev.name === "TradeEvent"
  )?.data;

  if (!swapTransaction || !eventTransaction) return null;

  const tokenXMint = swapTransaction.accounts.find(
    (acc: any) => acc.name === "token_x_mint"
  )?.pubkey;
  const tokenYMint = swapTransaction.accounts.find(
    (acc: any) => acc.name === "token_y_mint"
  )?.pubkey;

  const swapForY: boolean = eventTransaction.swapForY;
  const eventType = swapForY ? "Buy" : "Sell";

  const SOL_MINT = "So11111111111111111111111111111111111111112";
  const mint =
    tokenXMint !== SOL_MINT ? tokenXMint : tokenYMint;

  output = {
    Type: eventType,
    Mint: mint,
    Signer: eventTransaction.from,
    Pair: eventTransaction.lbPair,
    AmountIn: eventTransaction.amountIn,
    AmountOut: eventTransaction.amountOut,
  };

  return output;
}