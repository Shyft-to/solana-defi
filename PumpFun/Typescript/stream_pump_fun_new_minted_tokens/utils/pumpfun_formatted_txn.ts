export function parseSwapTransactionOutput(parsedInstruction) {
   let swapInstruction = 
        parsedInstruction?.instructions?.pumpAmmIxs?.find(
            instruction => instruction.name === 'create'
        ) ||
        parsedInstruction?.inner_ixs?.find(
            instruction => instruction.name === 'create' 
        ) ||
        parsedInstruction?.inner_ixs?.pump_amm_inner_ixs?.find(
            instruction => instruction.name === 'create' 
        );

  if (!swapInstruction) return;
    const args = swapInstruction.args;
  const name = args.name;
  const symbols = args.symbol;
  const url = args.uri;
  const creator = args.creator
  const mint = swapInstruction.accounts.find((ix)=> ix.name == "mint").pubkey;
  const bondingCurve = swapInstruction.accounts.find((ix) => ix.name == "bonding_curve").pubkey;
  let output = {};
    output = {
     Name : name,
     Symbol : symbols,
     Uri : url,
     Mint: mint,
     Bonding_Curve: bondingCurve,
     Creator: creator,
    }
  return output;
}