export function transactionOutput(txn){
    const type = txn?.instructions[0]?.name === "create"?txn?.instructions[0]?.name:txn?.instructions[1]?.name;
    const events = txn?.events[0]?.data;
    const mint = events?.mint;
    const args = txn?.instructions[0]?.name === "create"?txn?.instructions[0]:txn?.instructions[1];
    const name = args?.name;
    const symbols = args?.symbols;
    const uri = args?.uri
    
    return{
        type,
        mint,
        name,
        symbols,
        uri
    }
}