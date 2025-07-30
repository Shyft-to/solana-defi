export function parseSwapTransactionOutput(txn, parsedInstruction) {
  const parsedIxs = parsedInstruction?.inner_ixs?.pumpfun_amm_inner_ixs || [];
  const events = parsedInstruction?.inner_ixs?.events || [];
  const parsedEvents = events[0]?.data;

  const swapInstruction = parsedIxs.find(
    (ix) => ix.name === 'buy' || ix.name === 'sell'
  );
  if (!swapInstruction) return;

  const bondingCurve = swapInstruction.accounts.find(a => a.name === 'bondingCurve')?.pubkey;
  const type = swapInstruction.name;

  const mint = parsedEvents?.mint;
  const signer = parsedEvents?.user;
  const tokenReserves = parsedEvents?.virtualTokenReserves;

  const lamports = 1_000_000_000;
  const solRaw = parsedEvents?.virtualSolReserves;
  const solReserves = typeof solRaw === 'number' ? solRaw / lamports : undefined;

  const signature = txn?.transaction?.signatures?.[0];
  const calculated_progress = typeof solReserves === 'number' ? calculateProgress(solReserves) : undefined;

  // 🚫 If any key data is missing, skip this transaction entirely
  if (
    !mint ||
    !signer ||
    !tokenReserves ||
    typeof solReserves !== 'number' ||
    Number.isNaN(solReserves)
  ) {
    return;
  }

  return {
    TYPE: type,
    MINT: mint,
    SIGNER: signer,
    BONDING_CURVE: bondingCurve,
    TOKEN_RESERVE: tokenReserves,
    SOL_RESERVE: solReserves + " SOL",
    PROGRESS: calculated_progress,
    SIGNATURE: signature,
  };
}

function calculateProgress(solAmount: number): string {
  const TARGET_POOL_SIZE = 84;
  const percentage = (solAmount / TARGET_POOL_SIZE) * 100;
  return percentage.toFixed(2) + "%"; // returns a string like "93.57"
}