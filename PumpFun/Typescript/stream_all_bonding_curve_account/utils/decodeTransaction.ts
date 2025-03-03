import base58 from "bs58";
import { struct, bool, u64, Layout } from "@coral-xyz/borsh";

export const structure = struct([
    u64("discriminator"),
    u64("virtualTokenReserves"),
    u64("virtualSolReserves"),
    u64("realTokenReserves"),
    u64("realSolReserves"),
    u64("tokenTotalSupply"),
    bool("complete"),
  ]);


export function decodeTransact(data){
    const output = base58.encode(Buffer.from(data,'base64'))
    return output;
}
export function  bondingCurveData(buffer: Buffer) {

    let value = structure.decode(buffer);
    const discriminator = BigInt(value.discriminator);
    const virtualTokenReserves = BigInt(value.virtualTokenReserves);
    const virtualSolReserves = BigInt(value.virtualSolReserves);
    const realTokenReserves = BigInt(value.realTokenReserves);
    const realSolReserves = BigInt(value.realSolReserves);
    const tokenTotalSupply = BigInt(value.tokenTotalSupply);
    const complete = value.complete;
    return {
        discriminator,
        virtualTokenReserves,
        virtualSolReserves,
        realTokenReserves,
        realSolReserves,
        tokenTotalSupply,
        complete
    };
  }