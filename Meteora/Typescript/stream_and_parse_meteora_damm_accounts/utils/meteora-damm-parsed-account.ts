import { BorshAccountsCoder } from "@coral-xyz/anchor";
import * as fs from 'fs';
import base58 from "bs58";
import { bnLayoutFormatter } from "./bn-layout-formatter";

const program_idl = JSON.parse(fs.readFileSync('./idls/meteora_damm.json', "utf8"));
const coder = new BorshAccountsCoder(program_idl);

/**
 * Deserializes and trims blockchain data to extract relevant information.
 * @param {Object} data - The data object containing blockchain account information.
 * @returns {Object} - An object containing the deserialized signature, public key, owner, and pool state.
 */
export function meteoraDammParsedAccount(data) {
    try{

    if (!data || !data.account || !data.account.account)return;

      const dataTx = data.account.account;

    // Safely decode each piece of transaction data
    const signature = dataTx.txnSignature ? decodeTransact(dataTx.txnSignature) : null;
    const pubKey = dataTx.pubkey ? decodeTransact(dataTx.pubkey) : null;
    const owner = dataTx.owner ? decodeTransact(dataTx.owner) : null;
    
    let parsedAccount = null;
    try {
        parsedAccount = coder.decodeAny(dataTx.data);
        bnLayoutFormatter(parsedAccount)
    
    } catch (error) {
       // console.error("Failed to decode pool state:", error);
    }

    return {
        signature,
        pubKey,
        owner,
        parsedAccount
    };
   }catch(error){
   }
}
 function decodeTransact(data: string) {
  return base58.encode(Buffer.from(data, 'base64'));
}