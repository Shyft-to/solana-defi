export function parseSwapTransactionOutput(parsedInstruction) {
   let createInstruction = 
        parsedInstruction?.instructions?.pumpAmmIxs?.find(
            instruction => instruction.name === 'create' || instruction.name === 'create_v2'
        ) ||
        parsedInstruction?.inner_ixs?.find(
            instruction => instruction.name === 'create' || instruction.name === 'create_v2'
        ) ||
        parsedInstruction?.inner_ixs?.pump_amm_inner_ixs?.find(
            instruction => instruction.name === 'create' || instruction.name === 'create_v2'
        );
  const createEvents = parsedInstruction?.events?.find((x) => x.name === "CreateEvent");
  if (!createInstruction) return;
  const args = createInstruction.args;
  const name = args.name;
  const symbols = args.symbol;
  const url = args.uri;
  const creator = args.creator;
  const virtual_token_reserves = createEvents.virtual_token_reserves;
  const virtual_sol_reserves = createEvents.virtual_sol_reserves;
  const real_token_reserves = createEvents.real_token_reserves;
  const token_total_supply = createEvents.token_total_supply;
  const mint = createInstruction.accounts.find((ix)=> ix.name == "mint").pubkey;
  const bondingCurve = createInstruction.accounts.find((ix) => ix.name == "bonding_curve").pubkey;
  let output = {};
    output = {
     Name : name,
     Symbol : symbols,
     Uri : url,
     Mint: mint,
     Bonding_Curve: bondingCurve,
     Creator: creator,
     VirtualTokenReserves: virtual_token_reserves,
     VirtualSolReserves: virtual_sol_reserves,
     RealTokenReserves: real_token_reserves,
     TokenTotalSupply: token_total_supply
    }
  return output;
}