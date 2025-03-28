import base58 from "bs58";
import { struct, bool, u64, Layout } from "@coral-xyz/borsh";

export function decodeTransact(data){
    const output = base58.encode(Buffer.from(data,'base64'))
    return output;
}
