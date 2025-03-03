import {  decodePoolData, decodeTransact } from "./decodeTransaction";

/**
 * Deserializes and trims blockchain data to extract relevant information.
 * @param {Object} data - The data object containing blockchain account information.
 * @returns {Object} - An object containing the deserialized signature, public key, owner, and pool state.
 */
export function tOutPut(data) {
    try{
    if (!data || !data.account || !data.account.account)return;

      const dataTx = data.account.account;

    // Safely decode each piece of transaction data
    const signature = dataTx.txnSignature ? decodeTransact(dataTx.txnSignature) : null;
    const pubKey = dataTx.pubkey ? decodeTransact(dataTx.pubkey) : null;
    const owner = dataTx.owner ? decodeTransact(dataTx.owner) : null;
    
    let poolstate = null;
    try {
        poolstate = decodePoolData(dataTx.data);
    
    } catch (error) {
       // console.error("Failed to decode pool state:", error);
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
