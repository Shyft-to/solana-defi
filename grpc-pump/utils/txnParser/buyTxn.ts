export function buyTXN(logMessages:[]){
    const logSlice:any[] = logMessages.slice(0, 10);
    const containBuy = logSlice?.some(instruction=> instruction.match(instruction.match(/Buy/i)));
    if(!containBuy)return undefined;
    return containBuy;
} 