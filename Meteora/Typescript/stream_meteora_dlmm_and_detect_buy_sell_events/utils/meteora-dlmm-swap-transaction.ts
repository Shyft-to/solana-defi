export function transactionOutput(parsedInstruction,txn){
   let output = {};
   const swapTransactions = parsedInstruction.instructions.find((x)=> x.name == "swap" || x.name == "swap2");
   const eventTransactions = parsedInstruction?.events[0]?.data;
   if(!swapTransactions) return;
   const ptb = txn.meta.preTokenBalances.find((x) => x.mint !== "So11111111111111111111111111111111111111112").mint;
   const amountIn = eventTransactions.amount_in;
   const amountOut = eventTransactions.amount_out;
   const pair = eventTransactions.lb_pair;
   const signer = eventTransactions.from;
   const eventType = !eventTransactions.swap_for_y?"Sell":"Buy";
   output = {
    Type: eventType,
    Mint: ptb,
    Signer: signer,
    Pair: pair,
    AmountIn: amountIn,
    AmountOut: amountOut
   }
   return output
   
 }