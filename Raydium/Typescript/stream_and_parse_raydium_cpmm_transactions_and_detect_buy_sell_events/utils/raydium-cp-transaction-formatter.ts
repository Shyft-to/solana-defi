export function raydiumCPFormatter(parsedInstruction, txn) {
  let output = {};

  const instructions = parsedInstruction.instructions || parsedInstruction.raydiumCPIxs;
  const innerInstructions = parsedInstruction.inner_ixs || parsedInstruction.innerIx || parsedInstruction.innerInstructions;
  const preTB = txn.meta.postTokenBalances[0].owner
  const ev = parsedInstruction.events;
  const events =
    (Array.isArray(ev) ? ev[0]?.data || ev[0] : ev) || {};

  const swapInstruction =
    parsedInstruction.swapInstruction ||
    instructions?.find(
      (x) => x.name === "swap_base_input" || x.name === "swap_base_output"
    ) ||
    innerInstructions?.find(
     (x) => x.name === "swap_base_input" || x.name === "swap_base_output"
    ) ||
    undefined;
  const inputAmount = events.inputAmount ?? events.input_amount;
  const outputAmount = events.outputAmount ?? events.output_amount;
  const inputToken = events.inputMint ?? events.input_mint;
  const outputToken = events.outputMint ?? events.output_mint;

  if ( !inputAmount) return undefined;

  const payer =
    swapInstruction?.accounts?.find((x) => x.name === "payer")?.pubkey ?? preTB;

  const martDeterminer = (mint: string) => mint === "So11111111111111111111111111111111111111112"; 
  const type = (e => e?"Buy":"Sell")(martDeterminer(inputToken))
  output = {
    Type: type,
    Payer: payer,
    InputToken: inputToken,
    OutPutToken: outputToken,
    InAmount: inputAmount,
    OutAmount: outputAmount,
  };

  return output;
}
