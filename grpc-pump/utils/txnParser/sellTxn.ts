export function sellTXN(logMessages:[]){
    const logSlice:any[] = logMessages.slice(0, 10);
    const containSell = logSlice?.some(instruction=> instruction.match(instruction.match(/Sell/i)));
    if(!containSell)return undefined;
    return containSell;
} 