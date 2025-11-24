export function pump_amm_formatter(parsedInstruction, txn) {
  let output = {};
  const createPoolIx = parsedInstruction?.instructions.find(
    (x) => x.name === "create_pool"
  );

  if (!createPoolIx) return;

  const event = parsedInstruction.events?.[0]?.data;
  if (!event) return;

  const getPubkey = (name) =>
    createPoolIx.accounts?.find((x) => x.name === name)?.pubkey;

  const extracted = {
    lp_mint: getPubkey("lp_mint"),
    quote_mint: getPubkey("quote_mint"),
    base_mint: getPubkey("base_mint"),
    creator: getPubkey("creator"),
    pool: getPubkey("pool"),

    base_mint_decimals: event.base_mint_decimals,
    quote_mint_decimals: event.quote_mint_decimals,
    base_amount_in: event.base_amount_in,
    quote_amount_in: event.quote_amount_in,
    pool_base_amount: event.pool_base_amount,
    pool_quote_amount: event.pool_quote_amount,
    minimum_liquidity: event.minimum_liquidity,
    initial_liquidity: event.initial_liquidity,
    lp_token_amount_out: event.lp_token_amount_out,
    is_mayhem_mode: event.is_mayhem_mode
  };

  
  return  extracted;
}
