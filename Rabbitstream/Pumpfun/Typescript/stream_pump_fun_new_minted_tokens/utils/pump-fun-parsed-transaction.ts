export function pumpFunParsedTransaction(parsedInstruction, txn) {

  const filterMintTransactions = parsedInstruction.inner_ixs.find(
    (x) => x.name === "create" || x.name === "create_v2"
  );
  if (!filterMintTransactions) return;

  const createEvents = parsedInstruction.events.find(
    (x) => x.name === "CreateEvent"
  );
  if (!createEvents) return;

  const virtual_token_reserves = createEvents.data.virtual_token_reserves;
  const virtual_sol_reserves = createEvents.data.virtual_sol_reserves;
  const real_token_reserves = createEvents.data.real_token_reserves;
  const token_total_supply = createEvents.data.token_total_supply;

  const args = filterMintTransactions.args;

  const name = args.name;
  const symbols = args.symbol;
  const url = args.uri;
  const creator = args.creator;

  const mint = filterMintTransactions.accounts.find((ix) => ix.name === "mint")?.pubkey;
  const bondingCurve = filterMintTransactions.accounts.find((ix) => ix.name === "bonding_curve")?.pubkey;

  return {
    Name: name,
    Symbol: symbols,
    Uri: url,
    Mint: mint,
    Bonding_Curve: bondingCurve,
    Creator: creator,
    VirtualTokenReserves: virtual_token_reserves,
    VirtualSolReserves: virtual_sol_reserves,
    RealTokenReserves: real_token_reserves,
    TokenTotalSupply: token_total_supply
  };
}
