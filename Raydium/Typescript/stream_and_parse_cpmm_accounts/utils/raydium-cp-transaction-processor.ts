import { decodeRaydiumCpPool, base64ToBase58 } from "./raydium-cp-pool-utils";


export async function decodeRaydiumCpTxnData(data) {
    try{
    if (!data || !data.account || !data.account.account)return;

      const dataTx = data.account.account;

    const signature = dataTx.txnSignature ? base64ToBase58(dataTx.txnSignature) : null;
    const pubKey = dataTx.pubkey ? base64ToBase58(dataTx.pubkey) : null;
    const owner = dataTx.owner ? base64ToBase58(dataTx.owner) : null;
    
    let poolstate = null;
    try {
        poolstate = await decodeRaydiumCpPool(dataTx.data);
    
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
