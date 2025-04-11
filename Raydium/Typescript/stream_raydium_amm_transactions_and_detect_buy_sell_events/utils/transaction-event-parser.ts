const sol = 'So11111111111111111111111111111111111111112'
const usdc = 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v'

export function transactionEventParser(txn,parsedTxn){
 const pTB = txn.meta.postTokenBalances;
 const mints = pTB.find(
  (token) => token.mint !== sol && token.mint !== usdc
 )?.mint;
 const userSourceOwner = parsedTxn.instructions
    .flatMap((i) => i.accounts)
    .find((a) => a.name === 'userSourceOwner' && a.isSigner)?.pubkey;

 const eventName = parsedTxn.events[0].name;
 const data = parsedTxn.events[0].data;
 const poolCoinTokenAccount = parsedTxn.instructions
     .flatMap((i)=> i.accounts)
     .find((a) => a.name === 'poolCoinTokenAccount')?.pubkey;
const determineMarket = (amountIn: number) => {
    const transferInstruction = parsedTxn.instructions.find(
      (instruction) =>
        instruction.name === 'transfer' &&
        instruction.args?.amount === amountIn
    );

    if (transferInstruction) {
      const destinationAccount = transferInstruction.accounts.find(
        (account) => account.name === 'destination'
      )?.pubkey;

      return destinationAccount === poolCoinTokenAccount && (mints !== sol);
    }

    return false;
  };

 console.log("userSourceOwner :", userSourceOwner);
 if(eventName == "swapBaseIn"){
   const signer = userSourceOwner;
    const mint = mints;
    const amount_in = data.amountIn;
    const amount_out = data.outAmount;
    const type = determineMarket(amount_in) ? "Sell" : "Buy";
    return {
       "Event Name: ": eventName,
        "User: " : signer,
        "Mint: " : mint,
        "Amount in: ": amount_in,
        "Amount out: " : amount_out,
        "Type: " : type
    }
 }else if(eventName == "swapBaseOut"){
     const signer = userSourceOwner;
     const mint = mints;
     const amount_in = data.directIn;
     const amount_out = data.amountOut;
     const type = determineMarket(amount_in) ? "Sell" : "Buy";
     return {
        "Event Name: ": eventName,
        "User: ": signer,
        "Mint: " : mint,
        "Amount in: ": amount_in,
        "Amount out: " : amount_out,
        "Type: " : type
     }
 }
}
