export function pumpFunParsedTransaction(parsedInstruction,txn){
  const instructions = parsedInstruction?.instructions.find((x)=> x.name === "migrate") 
                  || parsedInstruction?.inner_ixs.find((x) => x.name === "migrate");
   if(!instructions) return;  
  return instructions;
}