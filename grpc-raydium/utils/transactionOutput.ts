import { LIQUIDITY_STATE_LAYOUT_V4 } from "@raydium-io/raydium-sdk";
import { decodeTransact } from "./decodeTransaction";

export function tOutPut(data){
    const dataTx = data?.account?.account;
    const signature = decodeTransact(dataTx?.txnSignature);
    const pubKey = decodeTransact(dataTx?.pubkey)
    const owner = decodeTransact(dataTx?.owner);
    const poolstate = LIQUIDITY_STATE_LAYOUT_V4.decode(dataTx.data); 
    return {
        signature,
        pubKey,
        owner,
        poolstate
    }

}
