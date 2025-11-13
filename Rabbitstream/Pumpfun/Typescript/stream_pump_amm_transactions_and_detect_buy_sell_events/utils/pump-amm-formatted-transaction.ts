export function pumpSwapParsedTransaction(parsedInstruction, txn) {
  let output = {};
  const SOL_MINT = 'So11111111111111111111111111111111111111112';

  const swapInstruction = parsedInstruction?.inner_ixs.find(
    (x) => x.name === 'buy' || x.name === 'sell' || x.name === 'buy_exact_quote_in'
  );
  if (!swapInstruction) return;

  const signerPubkey = swapInstruction?.accounts.find(
    (account) => account.name === 'user'
  )?.pubkey;

  const determineBuySellEvent = () => {
    const baseMintPubkey = swapInstruction?.accounts.find(
      (account) => account.name === 'base_mint'
    )?.pubkey;
    const quoteMintPubkey = swapInstruction?.accounts.find(
      (account) => account.name === 'quote_mint'
    )?.pubkey;

    if (!baseMintPubkey || !quoteMintPubkey) {
      console.error("Base or quote mint not found in swap accounts");
      return { type: "Unknown", mint: null };
    }

    const mint = baseMintPubkey === SOL_MINT ? quoteMintPubkey : baseMintPubkey;
    const eventType = swapInstruction.name === 'buy' ? "Buy" : "Sell";

    return { type: eventType, mint };
  };

  const buySellEvent = determineBuySellEvent();

  const base_amount_in =
    swapInstruction?.name === 'sell'
      ? swapInstruction?.args?.base_amount_in
      : swapInstruction?.args?.base_amount_out;

  const amountIn =
    swapInstruction?.name === 'buy'
      ? 0
      : base_amount_in;

  const InAmount =
    swapInstruction?.name === 'buy_exact_quote_in'
      ? swapInstruction?.args?.spendable_quote_in
      : amountIn;

  const amountOut =
    swapInstruction?.name === 'sell'
      ? 0
      : base_amount_in;

  const transactionEvent = {
    type: swapInstruction?.name,
    user: signerPubkey,
    mint: buySellEvent.mint,
    ...(amountOut ? { in_amount: amountOut } : { out_amount: InAmount })
  };

  return transactionEvent;
}
