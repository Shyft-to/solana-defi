const sol = 'So11111111111111111111111111111111111111112'
const usdc = 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v'
const raydium_program_owner = "5Q544fKrFoe6tsEbD7S8EmxGTJYAKtTVhAW5Q5pge4j1";
export function transactionEventParser(txn,parsedTxn){
 const preTB = txn.meta.preTokenBalances;
 const mints = preTB.find(
  (token) => token.mint !== sol && token.mint !== usdc
 )?.mint;
 const postTB =txn.meta.postTokenBalances;
 const preTB_filter = preTB.find((token) => token.owner === raydium_program_owner && token.mint !== sol);
 const preTB_ui_amount = preTB_filter.uiTokenAmount.uiAmount;
 const postTB_filter = postTB.find((token) => token.owner === raydium_program_owner && token.mint !== sol);
 const postTB_ui_amount = postTB_filter.uiTokenAmount.uiAmount;

 const determiner = preTB_ui_amount > postTB_ui_amount? "Buy" : "Sell";

 let userSourceOwner = parsedTxn.instructions
    .flatMap((i) => i.accounts)
    .find((a) => a.name === 'userSourceOwner' && a.isSigner)?.pubkey;

  if (!userSourceOwner) {
    userSourceOwner = parsedTxn.instructions
      .flatMap((i) => i.accounts)
      .find((a) => a.isSigner)?.pubkey;
  }

 const eventName = parsedTxn.events[0].name;
 const data = parsedTxn.events[0].data;

 if(eventName == "swapBaseIn"){
   const signer = userSourceOwner;
    const mint = mints;
    const amount_in = data.amountIn;
    const amount_out = data.outAmount;
    const type = determiner;
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
     const type = determiner;
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
