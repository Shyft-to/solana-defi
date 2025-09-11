export function meteoradbcTransactionOutput(parsedInstruction, txn) {
  const { inner_ixs, instructions } = parsedInstruction;
  const events = inner_ixs?.events || [];
  const innerSwapIxns = inner_ixs?.meteroa_dbc_inner_ixs || [];

  if (!events.length || !innerSwapIxns.length) return txn;

  const swapEvent = events[0].data;

  const swapTxn = innerSwapIxns.find(ix => ix.name === 'swap');
  if (!swapTxn) return txn;

  const baseMint = swapTxn.accounts.find(acc => acc.name === 'base_mint')?.pubkey;
  const payer = swapTxn.accounts.find(acc => acc.name === 'payer')?.pubkey;
  const {
    tradeDirection,
    params ,
    swapResult 
  } = swapEvent;
  const firstAmount = params?.amountIn;
  const SecondAmount = swapResult?.outputAmount;
  const type = Object.keys(tradeDirection)[0]; // "buy" or "sell"

  const amountIn = type === "buy" ? firstAmount : SecondAmount;
  const amountOut = type === "sell" ? firstAmount : SecondAmount;

  const tradeData = {
    type,
    user: payer,
    mint: baseMint,
    in_amount: amountIn,
    out_amount: amountOut,
  };

  if (txn.version === 0) {
    return {
      ...txn,
      meta: {
        ...txn.meta,
        innerInstructions: inner_ixs,
      },
      transaction: {
        ...txn.transaction,
        message: {
          ...txn.transaction.message,
          compiledInstructions: instructions,
        },
      },
      TradeEvent: tradeData,
    };
  }

  return txn;
}
