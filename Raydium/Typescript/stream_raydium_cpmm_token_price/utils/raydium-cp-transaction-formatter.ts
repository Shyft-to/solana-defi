export function raydiumCPFormatter(parsedInstruction, txn) {
  let output = {};

  const instructions = parsedInstruction.instructions || parsedInstruction.raydiumCPIxs;
  const innerInstructions =
    parsedInstruction.inner_ixs ||
    parsedInstruction.innerIx ||
    parsedInstruction.innerInstructions;

  const preTB = txn.meta?.postTokenBalances;

  const ev = parsedInstruction.events;
  const events = (Array.isArray(ev) ? ev[0]?.data || ev[0] : ev) || {};

  const swapInstruction =
    parsedInstruction.swapInstruction ||
    instructions?.find(
      (x) => x.name === "swap_base_input" || x.name === "swap_base_output"
    ) ||
    innerInstructions?.find(
      (x) => x.name === "swap_base_input" || x.name === "swap_base_output"
    );

  if (!swapInstruction) return undefined;

  const inputAmount = events.inputAmount ?? events.input_amount;
  const outputAmount = events.outputAmount ?? events.output_amount;
  const inputToken = events.inputMint ?? events.input_mint;
  const outputToken = events.outputMint ?? events.output_mint;
  const inputVault = events.inputVaultBefore ?? events.input_vault_before;
  const outputVault = events.outputVaultBefore ?? events.output_vault_before;

  if (!inputAmount || !inputVault || !outputVault) return undefined;

  const martDeterminer = (mint: string) =>
    mint === "So11111111111111111111111111111111111111112";
  const type = martDeterminer(inputToken) ? "Buy" : "Sell";

  const inputDecimals = preTB.find((x) => x.mint === inputToken)?.uiTokenAmount?.decimals;
  const outputDecimals = preTB.find((x) => x.mint === outputToken)?.uiTokenAmount?.decimals;

  const rawPrice =
    Number(inputVault) > 0
      ? Number(outputVault) / Number(inputVault)
      : undefined;

  let adjustedPrice: number | undefined;

  if (martDeterminer(inputToken)) {
    adjustedPrice =
      Number(outputVault) > 0
        ? (Number(inputVault) / 10 ** inputDecimals) /
          (Number(outputVault) / 10 ** outputDecimals)
        : undefined;
  } else {
    adjustedPrice =
      Number(inputVault) > 0
        ? (Number(outputVault) / 10 ** outputDecimals) /
          (Number(inputVault) / 10 ** inputDecimals)
        : undefined;
  }

  output = {
    Type: type,
    InputToken: inputToken,
    OutputToken: outputToken,
    InputVaultBefore: inputVault,
    OutputVaultBefore: outputVault,
    Price: adjustedPrice?.toFixed(17) + " SOL",
  };

  return output;
}
