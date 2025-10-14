export function meteoradammV2TransactionOutput(parsedInstruction, txn) {
  let output = {};
  let SOL = "So11111111111111111111111111111111111111112"
  const swapInstruction = parsedInstruction.instructions.find(
    (instruction) => instruction.name === 'swap'
  );
  if (!swapInstruction) return;
  const input_amount = swapInstruction.args.params.amount_in;
  const pool_authority = swapInstruction.accounts.find(a => a.name == "pool_authority").pubkey;
  const mint_a = swapInstruction.accounts.find(a => a.name === "token_a_mint").pubkey;
  const mint_b = swapInstruction.accounts.find(a => a.name === "token_b_mint").pubkey;
  const payer = swapInstruction.accounts.find(a => a.name === "payer").pubkey;
  const preTokenBalances = txn.meta.preTokenBalances.find(a=> a.owner === pool_authority && a.mint != SOL)?.uiTokenAmount?.uiAmount;
  const postTokenBalance = txn.meta.postTokenBalances.find(a=> a.owner === pool_authority && a.mint != SOL)?.uiTokenAmount?.uiAmount;
  const buy_sell_determiner = preTokenBalances > postTokenBalance ? "Buy" : "Sell";


  const outputTransfer = parsedInstruction.inner_ixs.find(
    (ix) =>
      ix.name === "transferChecked" &&
      ix.args.amount != input_amount
  );
 
  const event_type = {
    type: buy_sell_determiner,
    user: payer,
    mint_a: mint_a,
    mint_b: mint_b,
    amount_in: input_amount,
    amount_out: outputTransfer.args.amount
  };

  return event_type;
}
