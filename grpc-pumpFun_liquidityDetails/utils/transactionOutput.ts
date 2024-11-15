export function transactionOutput(txn){
    const type = txn.instructions[0].name === "sell"?"SELL":"BUY";
    const events = txn.events[0].data;
    const mint = events?.mint;
    const solAmount = events?.solAmount/1000000000
    const tokenAmount = events?.tokenAmount;
    const user = events?.user
    return{
        type,
        mint,
        solAmount,
        tokenAmount,
        user
    }
}