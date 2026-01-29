export function meteoradammTransactionOutput(parsedInstruction, txn) {
  const ixns = parsedInstruction?.inner_ixs;
  if (!Array.isArray(ixns)) return;

  const swapTxn = ixns.find(
    (ix) => ix.name === "swap" || ix.name === "swap2"
  );
  if (!swapTxn) return;

  const params =
    swapTxn?.args?.params ??
    swapTxn?.args?._params ??
    {};

  const getAccount = (name) =>
    swapTxn.accounts.find((a) => a.name === name)?.pubkey;

  const baseMint = getAccount("token_a_mint");
  const quoteMint = getAccount("token_b_mint");
  const payer = getAccount("payer");

  let amountIn;
  let amountOut;
  let type;

  if (swapTxn.name === "swap2") {
    amountIn = params.amount_0;
    amountOut = params.amount_1;
    type = params.swap_mode === 0 ? "Sell" : "Buy";
  } else {
    amountIn = params.amount_in;
    amountOut = params.minimum_amount_out;

    type = amountIn < amountOut ? "Buy" : "Sell";
  }

  return {
    Type: type,
    BaseMint: baseMint,
    QuoteMint: quoteMint,
    User: payer,
    AmountIn: amountIn,
    AmountOut: amountOut
  };
}
