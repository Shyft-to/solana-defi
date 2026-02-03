export function parseSwapTransactionOutput(parsedInstruction) {
  if (!parsedInstruction) return;
  const innerInstructions =
    parsedInstruction?.inner_ixs?.pumpfun_inner_ixs ??
    parsedInstruction?.inner_ixs?.pump_amm_inner_ixs ??
    parsedInstruction?.inner_ixs ??
    [];

  const swapInstruction =
    parsedInstruction?.instructions?.pumpAmmIxs?.find(ix =>
      ix.name === 'buy' ||
      ix.name === 'sell' ||
      ix.name === 'buy_exact_sol_in'
    ) ||
    innerInstructions.find(ix =>
      ix.name === 'buy' ||
      ix.name === 'sell' ||
      ix.name === 'buy_exact_sol_in'
    );

  if (!swapInstruction) return;

  const { name: type, accounts = [], args = {} } = swapInstruction;

  const bondingCurve = accounts.find(a => a.name === 'bondingCurve')?.pubkey;
  const userPubkey = accounts.find(a => a.name === 'user')?.pubkey;
  const mint = accounts.find(a => a.name === 'mint')?.pubkey;

  const tradeEvent = parsedInstruction?.events?.find(
    e => e.name === 'TradeEvent'
  );

  if (!tradeEvent?.data) return;

  const {
    sol_amount,
    token_amount,
    timestamp,
    virtual_sol_reserves,
    virtual_token_reserves,
    real_sol_reserves,
    real_token_reserves,
    fee,
    fee_basis_points,
    creator_fee,
    creator_fee_basis_points,
  } = tradeEvent.data;

  const isBuy = type === 'buy' || type === 'buy_exact_sol_in';

  const inAmount = isBuy ? sol_amount : token_amount;
  const outAmount = isBuy ? token_amount : sol_amount;

  const readableTime = timestamp
    ? new Date(timestamp * 1000).toISOString()
    : null;
   const price = calculatePumpFunPrice(
        virtual_sol_reserves,
        virtual_token_reserves
    ); 
 
  const priceSolPerToken =
    sol_amount && token_amount
      ? sol_amount / token_amount
      : null;

  const poolState = {
    virtual: {
      sol: virtual_sol_reserves,
      token: virtual_token_reserves,
    },
    real: {
      sol: real_sol_reserves,
      token: real_token_reserves,
    },
  };

  const fees = {
    protocol: {
      amount: fee,
      bps: fee_basis_points,
    },
    creator: {
      amount: creator_fee,
      bps: creator_fee_basis_points,
    },
  };

  const behavior = {
    is_buy: isBuy,
    is_sell: !isBuy,
    large_trade: sol_amount > 5_000_000_000, 
    early_pool:
      virtual_sol_reserves < 20_000_000_000, 
  };

  return {
    type,
    user: userPubkey,
    mint,
    bonding_curve: bondingCurve,

    in_amount: inAmount,
    out_amount: outAmount,

    timestamp_in_blockchain: timestamp,
    readable_time_of_trade: readableTime,

    price: {
      sol_per_token: priceSolPerToken,
      token_per_sol: priceSolPerToken ? 1 / priceSolPerToken : null,
      market_price : price,
    },

    pool_state: poolState,
    fees,
    behavior,
  };
}
function calculatePumpFunPrice(
    virtualSolReserves: number,
    virtualTokenReserves: number
): string {
    const sol = virtualSolReserves / 1_000_000_000; 
    const tokens = virtualTokenReserves / 1_000_000; 
    const price = sol / tokens;
    const formattedPrice = price.toFixed(20).replace(/0+$/, '');
    return formattedPrice;
}