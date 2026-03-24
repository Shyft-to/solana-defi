export interface SwapPrice {
  tokenIn: {
    mint: string;
    decimals: number;
    amount: string;
    uiAmount: number;
  };
  tokenOut: {
    mint: string;
    decimals: number;
    amount: string;
    uiAmount: number;
  };
  priceOfTokenIn: string;
  priceOfTokenOut: string;
}

/** Format a number in full decimal notation (no scientific notation), trimming trailing zeros. */
function toFullDecimal(value: number, maxDecimals = 20): string {
  if (!isFinite(value) || value === 0) return "0";
  // toFixed gives us up to 20 decimal places without scientific notation
  const fixed = value.toFixed(maxDecimals);
  // Trim trailing zeros but keep at least one decimal place
  return fixed.replace(/(\.\d*?)0+$/, "$1").replace(/\.$/, ".0");
}

export function calculateSwapPrice(
  parsedInstruction: any,
  txn: any,
): SwapPrice | null {
  const events: any[] = parsedInstruction.inner_ixs?.events ?? [];
  const swapEvent = events.find((e: any) => e.name === "TradeEvent");
  if (!swapEvent) return null;

  const { amountIn, amountOut } = swapEvent.data;
  if (!amountIn || !amountOut) return null;

  const swapIx = parsedInstruction.instructions?.find(
    (ix: any) => ix.name === "swap",
  );
  if (!swapIx) return null;

  const userSourceToken = swapIx.accounts?.find(
    (a: any) => a.name === "userSourceToken",
  )?.pubkey;
  const userDestinationToken = swapIx.accounts?.find(
    (a: any) => a.name === "userDestinationToken",
  )?.pubkey;

  if (!userSourceToken || !userDestinationToken) return null;

  const pre: any[] = txn.meta?.preTokenBalances ?? [];
  const post: any[] = txn.meta?.postTokenBalances ?? [];

  const accountKeys: string[] =
    txn.transaction?.message?.staticAccountKeys?.map((k: any) => k?.toString()) ?? [];
  const loadedWritable: string[] =
    txn.meta?.loadedAddresses?.writable?.map((k: any) => k?.toString()) ?? [];
  const loadedReadonly: string[] =
    txn.meta?.loadedAddresses?.readonly?.map((k: any) => k?.toString()) ?? [];
  const allAccounts = [...accountKeys, ...loadedWritable, ...loadedReadonly];

  const sourceIndex = allAccounts.findIndex((k) => k === userSourceToken?.toString());
  const destIndex = allAccounts.findIndex((k) => k === userDestinationToken?.toString());

  if (sourceIndex === -1 || destIndex === -1) return null;

  const findBalance = (index: number, balances: any[]) =>
    balances.find((b: any) => b.accountIndex === index);

  const sourceEntry = findBalance(sourceIndex, pre) ?? findBalance(sourceIndex, post);
  const destEntry = findBalance(destIndex, post) ?? findBalance(destIndex, pre);

  if (!sourceEntry || !destEntry) return null;

  const tokenInDecimals: number = sourceEntry.uiTokenAmount.decimals;
  const tokenOutDecimals: number = destEntry.uiTokenAmount.decimals;

  const inAmountRaw = BigInt(amountIn);
  const outAmountRaw = BigInt(amountOut);

  const inAmountUi = Number(inAmountRaw) / 10 ** tokenInDecimals;
  const outAmountUi = Number(outAmountRaw) / 10 ** tokenOutDecimals;

  const priceOfTokenIn = outAmountUi / inAmountUi;
  const priceOfTokenOut = inAmountUi / outAmountUi;

  return {
    tokenIn: {
      mint: sourceEntry.mint,
      decimals: tokenInDecimals,
      amount: amountIn,
      uiAmount: inAmountUi,
    },
    tokenOut: {
      mint: destEntry.mint,
      decimals: tokenOutDecimals,
      amount: amountOut,
      uiAmount: outAmountUi,
    },
    priceOfTokenIn: toFullDecimal(priceOfTokenIn),
    priceOfTokenOut: toFullDecimal(priceOfTokenOut),
  };
}

export function meteoraPoolTxn(parsedInstruction: any, txn: any) {
  return calculateSwapPrice(parsedInstruction, txn);
}