export function parsedTransactionOutput(parsedInstruction, transaction) {
  const swapInstruction = parsedInstruction.instructions.find((instruction) => 
    instruction.name === 'swap_v2' || instruction.name === 'swap');
  const swapEvent = parsedInstruction.events[0]?.data;
  if (!swapInstruction) {
    return;
  }
  const token_A = swapInstruction.accounts.find((ix) => ix.name === 'output_vault_mint')?.pubkey;
  const token_B = swapInstruction.accounts.find((ix) => ix.name === 'input_vault_mint')?.pubkey;
  const payer = swapInstruction.accounts.find((ix) => ix.name === 'payer')?.pubkey;
  const amount_A = swapEvent.amount_0;
  const amount_B = swapEvent.amount_1;
  const swap_type = swapEvent.zero_for_one? "Buy" : "Sell";
  const transactionEvent = {
    type: swap_type,
    user: payer,
    mint_A: token_A,
    mint_B: token_B,
    amount_in: amount_A,
    amount_out: amount_B
  };

  return transactionEvent;
}
