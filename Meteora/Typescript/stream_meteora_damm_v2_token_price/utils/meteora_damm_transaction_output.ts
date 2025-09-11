export function meteoradammTransaction(parsedInstruction,txn){
   const { inner_ixs, instructions } = parsedInstruction;
  const events = inner_ixs?.events || [];
  const innerSwapIxns = inner_ixs?.meteroa_damm_inner_ixs || [];
  if (!events.length || !innerSwapIxns.length) return;

  const swapEvent = events[0].data;

  const swapTxn = innerSwapIxns.find(ix => ix.name === 'swap');
  const transferCheck = innerSwapIxns.filter(ix => ix.name === 'transferChecked');
  const transfers = transferCheck.map(ix => {
  const mint = ix.accounts.find(acc => acc.name === 'mint')?.pubkey;
  const source = ix.accounts.find(acc => acc.name === 'source')?.pubkey;
  const destination = ix.accounts.find(acc => acc.name === 'destination')?.pubkey;
  const decimal = ix.args.decimals;

  return { mint, source, destination, decimal };
  });
  if (!swapTxn) return txn;

  const baseMint = swapTxn.accounts.find(acc => acc.name === 'token_a_mint')?.pubkey;
  const quoteMint = swapTxn.accounts.find(acc => acc.name === 'token_b_mint')?.pubkey;
  const baseDecimal = transfers.find(acc => acc.mint === baseMint)?.decimal;
  const quoteDecimal = transfers.find(acc => acc.mint === quoteMint)?.decimal;
  const payer = swapTxn.accounts.find(acc => acc.name === 'payer')?.pubkey;
  const {
    swapResult 
  } = swapEvent;

  const sqrtPrice = swapResult.nextSqrtPrice;
  const calculatePrice = sqrtPriceX64ToPrice(sqrtPrice,baseDecimal,quoteDecimal);
  const priceData = {
    Token_A: baseMint,
    Token_B: quoteMint,
    Price: parseFloat(calculatePrice.toString()).toFixed(13) + " SOL"  };
  return priceData;
}
function sqrtPriceX64ToPrice(nextSqrtPriceStr: string, decimalsA: number, decimalsB: number) {
  const sqrtPriceX64 = BigInt(nextSqrtPriceStr);
  const sqrtPrice = Number(sqrtPriceX64) / 2 ** 64;
  let price = sqrtPrice * sqrtPrice;
  const decimalAdjustment = 10 ** (decimalsA - decimalsB);
  price = price * decimalAdjustment;
  return price;
}