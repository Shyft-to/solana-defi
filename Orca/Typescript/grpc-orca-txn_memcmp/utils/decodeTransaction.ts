import base58 from "bs58";

export function decodeTransact(data){
    const output = data?base58.encode(Buffer.from(data,'base64')):"";
    return output;
}