export function parsedTransactionOutput(parsedInstruction, transaction) {
  let output = {};
  let Sol = "So11111111111111111111111111111111111111112"; 
  let USD1 = "USD1ttGY1N17NEEHLmELoaybftRBUSErhqYiQzvEmuB";
  
  const supportedInstructions = ['buy_exact_in', 'buy_exact_out', 'sell_exact_in', 'sell_exact_out'];
  const swapInstruction = parsedInstruction.instructions.find((instruction) =>
    supportedInstructions.includes(instruction.name)
  );
  
  const events = parsedInstruction?.events?.find((ix)=> ix.name === "TradeEvent")?.data;
  if (!swapInstruction || !events) {
    console.error("Supported instruction or TradeEvent not found");
    return transaction;
  }

  // Extract reserves
  const baseBeforeReserve = events.real_base_before;
  const quoteBeforeReserve = events.real_quote_before;
  const baseAfterReserve = events.real_base_after;
  const quoteAfterReserve = events.real_quote_after;
  const virtualBaseReserves = events?.virtual_base;
  const virtualQuoteReserve = events?.virtual_quote;

  // Token info
  const baseMintPubkey = swapInstruction.accounts.find((account) => account.name === 'base_token_mint')?.pubkey;
  const preTB = transaction.meta.preTokenBalances;

  // Find decimals safely
  const virtualBaseDecimal = preTB?.find((ix)=> ix.mint !== Sol && ix.mint !== USD1)?.uiTokenAmount?.decimals || 6;
  const virtualQuoteDecimal = preTB?.find((ix) => ix.mint !== baseMintPubkey)?.uiTokenAmount?.decimals || 9;

  const signerPubkey = swapInstruction.accounts.find((account) => account.name === 'payer')?.pubkey;

  // Compute before & after prices
  let beforePrice = calculateRaydiumLaunchPadPrice(baseBeforeReserve, quoteBeforeReserve, virtualQuoteDecimal, virtualBaseDecimal);
  let afterPrice = calculateRaydiumLaunchPadPrice(baseAfterReserve, quoteAfterReserve, virtualQuoteDecimal, virtualBaseDecimal);

  // ✅ Handle NaN or Infinity
  if (!isFinite(beforePrice)) beforePrice = afterPrice;
  if (!isFinite(afterPrice)) afterPrice = beforePrice;

  // Format to readable
  const beforeReadablePrice = formatReadableNumber(beforePrice, 12);
  const afterReadablePrice = formatReadableNumber(afterPrice, 12);

  output = {
    User: signerPubkey,
    Mint: baseMintPubkey,
    VirtualBaseReserves: virtualBaseReserves,
    virtualQuoteReserve: virtualQuoteReserve,
    PriceBeforeSale: beforeReadablePrice,
    PriceAfterSales: afterReadablePrice,
  };

  return output;
}

function calculateRaydiumLaunchPadPrice(
  virtualBaseReserves,
  virtualQuoteReserve,
  quoteDecimals,
  baseDecimals
) {
  const quoteAmount = virtualQuoteReserve / 10 ** quoteDecimals;
  const baseAmount = virtualBaseReserves / 10 ** baseDecimals;
  if (baseAmount === 0 || quoteAmount === 0) return NaN; // prevent division by zero
  return quoteAmount / baseAmount;
}

function formatReadableNumber(value, decimals = 12) {
  if (!isFinite(value) || isNaN(value)) return "0"; // fallback for bad values
  let str = value.toPrecision(decimals);
  if (str.includes('e')) {
    str = value.toFixed(decimals);
  }
  return str;
}
