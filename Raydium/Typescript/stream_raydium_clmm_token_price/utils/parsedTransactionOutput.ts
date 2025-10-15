export function parsedTransactionOutput(parsedInstruction, transaction) {
  const swapInstruction = parsedInstruction.instructions.find((instruction) => 
    instruction.name === 'swap_v2' || instruction.name === 'swap');
  const swapEvent = parsedInstruction.events[0]?.data;
  const preTB = transaction.meta.preTokenBalances;
  if (!swapInstruction) {
    return;
  }
  const tokenA = swapInstruction.accounts.find((ix) => ix.name === 'output_vault_mint')?.pubkey;
  const tokenB = swapInstruction.accounts.find((ix) => ix.name === 'input_vault_mint')?.pubkey;
  const payer = swapInstruction.accounts.find((ix) => ix.name === 'payer')?.pubkey;
  const SOL_MINT = "So11111111111111111111111111111111111111112";

  const isBaseSOL = tokenA === SOL_MINT;

  let baseDecimals = preTB.find(acc => acc.mint === tokenA)?.uiTokenAmount?.decimals;
  let quoteDecimals = preTB.find(acc => acc.mint === tokenB)?.uiTokenAmount?.decimals;
  
  let invert = false;

  if (isBaseSOL) {
  [baseDecimals, quoteDecimals] = [quoteDecimals, baseDecimals]; 
  invert = true;
  }
  const sqrtPrice = swapEvent.sqrt_price_x64 ?? swapInstruction.args?.sqrtPriceLimitX64;

  if (!sqrtPrice || baseDecimals === undefined || quoteDecimals === undefined) {
   return undefined
  }
  const calculatePrice = sqrtPriceX64ToPrice(sqrtPrice, baseDecimals, quoteDecimals, true); 
  const priceFormated = formatPrice(calculatePrice)
  const transactionEvent = {
    user: payer,
    mint_A: tokenA,
    mint_B: tokenB,
    price: priceFormated + " SOL"
  };

    return transactionEvent;
  }

  function sqrtPriceX64ToPrice(
  sqrtPriceX64Str: string,
  baseDecimals: number,
  quoteDecimals: number,
  invert = false
): number {
  const Q64 = BigInt(2) ** BigInt(64);
  const sqrtPriceX64 = BigInt(sqrtPriceX64Str);

  const squared = sqrtPriceX64 * sqrtPriceX64;
  const denominator = Q64 * Q64;

  let price = Number(squared) / Number(denominator);

  const decimalAdjustment = Math.pow(10, quoteDecimals - baseDecimals);
  price *= decimalAdjustment;

  return invert ? 1 / price : price;
}
function formatPrice(price: number, decimals = 20): string {
  let fixed = price.toFixed(decimals);

  fixed = fixed.replace(/\.?0+$/, '');

  return fixed;
}
