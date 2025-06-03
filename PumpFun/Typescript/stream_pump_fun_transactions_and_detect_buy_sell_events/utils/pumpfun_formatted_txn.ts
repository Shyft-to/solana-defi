export function parseSwapTransactionOutput(parsedInstruction){

    const swapInstruction = parsedInstruction.instructions.find(
         (instruction) => instruction.name === 'buy' || instruction.name === 'sell'
      );
      if (!swapInstruction) {
           return;
      }

      const signerPubkey = swapInstruction.accounts.find((account) => account.name === 'user')?.pubkey;
     const out_amount = parsedInstruction?.events?.[0]?.data?.solAmount;
     const determineBuySellEvent = () => {
         const mint = swapInstruction.accounts.find((account) => account.name === 'mint')?.pubkey;
         if (!mint ) {
             console.error("Base or quote mint not found in swap accounts");
             return { type: "Unknown", mint: null };
         }

         const eventType = swapInstruction.name === 'buy' ? "Buy" : "Sell";

         return { type: eventType, mint };
     };

     const buySellEvent = determineBuySellEvent();
     const base_amount_in =  swapInstruction.args?.amount;
     
      const amountIn = swapInstruction.name === 'buy'
         ? out_amount
         : base_amount_in;

  const amountOut = swapInstruction.name === 'sell'
          ? out_amount
         : base_amount_in;
     const transactionEvent = {
         type: swapInstruction.name,
         user: signerPubkey,
         mint: buySellEvent.mint,
         out_amount: amountOut,
         in_amount: amountIn, 
      };


    return  transactionEvent ;
}