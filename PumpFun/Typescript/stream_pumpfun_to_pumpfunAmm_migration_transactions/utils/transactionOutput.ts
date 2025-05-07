import { decodeTransact } from "./decodeTransaction";

export function tOutPut(data){
    const dataTx = data?data?.transaction?.transaction:null;
    const signature = decodeTransact(dataTx?.signature);
    const message = dataTx?.transaction?.message
    const header = message?.header;
    const accountKeys = message?.accountKeys.map((t)=>{
        return  decodeTransact(t)
    })
    const recentBlockhash =  decodeTransact(message?.recentBlockhash);
    const instructions = message?.instructions
    const meta = dataTx?.meta
    return {
        signature,
        message:{
           header,
           accountKeys,
           recentBlockhash,
           instructions
        },
        meta
    }

}

export function transactionOutput(parsedInstruction,txn){
    let output = {};
  
    if(txn.version === 0){
      output = {
        ...txn,
        meta: {
          ...txn.meta,
          innerInstructions: parsedInstruction.inner_ixs,
        },
        transaction: {
          ...txn.transaction,
          message: {
            ...txn.transaction.message,
            compiledInstructions: parsedInstruction.instructions,
          },
        }
      }
    }
    else {
      output = {
        ...txn,
        meta: {
          ...txn.meta,
          innerInstructions: parsedInstruction.inner_ixs,
        },
        transaction: {
          ...txn.transaction,
          message: {
            ...txn.transaction.message,
            instructions: parsedInstruction.instructions,
          },
        }
      }
    }
  
    return output;
  }
