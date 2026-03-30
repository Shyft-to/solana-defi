const SOL = "So11111111111111111111111111111111111111112";

export function meteoradammV2Price(parsedInstruction) {
  const swapInstruction = parsedInstruction.instructions.find(
    (instruction) => instruction.name === "swap"
  );
  if (!swapInstruction) return;

  const tradeEvent = parsedInstruction.inner_ixs?.events?.find(
    (e) => e.name === "TradeEvent"
  );
  if (!tradeEvent) return;

  const { data } = tradeEvent;

  const baseMint  = swapInstruction.accounts.find((a) => a.name === "token_a_mint")?.pubkey;
  const quoteMint = swapInstruction.accounts.find((a) => a.name === "token_b_mint")?.pubkey;

  // Pull actual transferred amounts from transferChecked — ground truth
  const transfers = parsedInstruction.inner_ixs?.meteroa_damm_inner_ixs
    ?.filter((ix) => ix.name === "transferChecked")
    .map((ix) => ({
      mint:        ix.accounts.find((a) => a.name === "mint")?.pubkey,
      decimal:     ix.args.decimals,
      amount:      BigInt(ix.args.amount),
      source:      ix.accounts.find((a) => a.name === "source")?.pubkey,
      destination: ix.accounts.find((a) => a.name === "destination")?.pubkey,
    })) ?? [];

  if (transfers.length < 2) return;

  const baseTransfer  = transfers.find((t) => t.mint === baseMint);
  const quoteTransfer = transfers.find((t) => t.mint === quoteMint);

  if (!baseTransfer || !quoteTransfer) return;

  const baseDecimal  = baseTransfer.decimal;
  const quoteDecimal = quoteTransfer.decimal;
  const baseAmount   = baseTransfer.amount;
  const quoteAmount  = quoteTransfer.amount;

  const isBuy  = "buy" in data.tradeDirection;
  const solIsA = baseMint === SOL;

  let solRaw:   bigint;
  let tokenRaw: bigint;
  let solDec:   number;
  let tokenDec: number;

  if (solIsA) {
    solRaw   = baseAmount;
    tokenRaw = quoteAmount;
    solDec   = baseDecimal;
    tokenDec = quoteDecimal;
  } else {
    solRaw   = quoteAmount;
    tokenRaw = baseAmount;
    solDec   = quoteDecimal;
    tokenDec = baseDecimal;
  }

  const solHuman   = Number(solRaw)   / 10 ** solDec;
  const tokenHuman = Number(tokenRaw) / 10 ** tokenDec;

  const price = tokenHuman > 0 ? solHuman / tokenHuman : 0;

  return {
    pool:      data.pool,
    timestamp: data.currentTimestamp,
    trade: {
      direction:  isBuy ? "Buy" : "Sell",
      amount_in:  data.params.amountIn,
      amount_out: data.swapResult.outputAmount,
    },
    price: price.toFixed(13), // SOL per token
    mints: {
      token_a:          baseMint,
      token_b:          quoteMint,
      token_a_decimals: baseDecimal,
      token_b_decimals: quoteDecimal,
    },
  };
}
