export function meteoradammV2TransactionOutput(parsedInstruction, txn) {
  const swapInstruction = parsedInstruction.instructions.find(
    (instruction) => instruction.name === "swap"
  );
  if (!swapInstruction) return;

  const tradeEvent = parsedInstruction.events?.find(
    (e) => e.name === "TradeEvent"
  );
  if (!tradeEvent) return;

  const { data } = tradeEvent;

  const mint_a = swapInstruction.accounts.find((a) => a.name === "token_a_mint")?.pubkey;
  const mint_b = swapInstruction.accounts.find((a) => a.name === "token_b_mint")?.pubkey;
  const payer  = swapInstruction.accounts.find((a) => a.name === "payer")?.pubkey;

  const isBuy = "buy" in data.tradeDirection;

  const token_in  = isBuy ? mint_b : mint_a;
  const token_out = isBuy ? mint_a : mint_b;

  return {
    type: isBuy ? "Buy" : "Sell",
    user: payer,
    token_in,
    token_out,
    amount_in:  data.params.amountIn,
    amount_out: data.swapResult.outputAmount,
    lp_fee:           data.swapResult.lpFee,
    protocol_fee:     data.swapResult.protocolFee,
    pool:             data.pool,
    collect_fee_mode: data.collectFeeMode,
    reserve_a:        data.reserveAAmount,
    reserve_b:        data.reserveBAmount,
    timestamp:        data.currentTimestamp,
  };
}