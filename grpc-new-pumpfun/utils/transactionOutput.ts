import { decodeTransact } from "./decodeTransaction";

export function tOutPut(data){
    const dataTx = data?.transaction?.transaction
    const signature = data != undefined?decodeTransact(dataTx?.signature):"";
    const message = dataTx?.transaction?.message
    const header = message?.header;
    const accountKeys = data != undefined?message?.accountKeys?.map((t)=>{
        return  decodeTransact(t)
    }):"";
    const recentBlockhash =  decodeTransact(message?.recentBlockhash);
    const instructions = message?.instructions
    const meta = dataTx?.meta
    const logs : any[] = meta?.logMessages;
    const logFilter = logs?.some(instruction =>
         instruction.match(instruction.match(/MintTo/i)));
   if(logFilter === true){}
    return {
        signature,
        message:{
           header,
           accountKeys,
           recentBlockhash,
           instructions
        },
        meta,
        logFilter
    }
 }
