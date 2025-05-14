import base58 from "bs58";

export function convertBase64ToBase58(data){
    const output = base58.encode(Buffer.from(data,'base64'))
    return output;
}
