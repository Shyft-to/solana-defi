import { parseCLMMPoolData, decodeTransactionBase58 } from "./decodePoolData";

export async function parsedRaydiumAccount(data) {
    try{
    if (!data || !data.account || !data.account.account)return;

      const dataTx = data.account.account;

    const signature = dataTx.txnSignature ? decodeTransactionBase58(dataTx.txnSignature) : null;
    const pubKey = dataTx.pubkey ? decodeTransactionBase58(dataTx.pubkey) : null;
    const owner = dataTx.owner ? decodeTransactionBase58(dataTx.owner) : null;
    
    let poolstate = null;
    try {
        poolstate = await parseCLMMPoolData(dataTx.data);
    
    } catch (error) {
    }

    return {
        signature,
        pubKey,
        owner,
        poolstate
    };
   }catch(error){
   }
}