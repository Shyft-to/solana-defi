
import base58 from "bs58";
import { blob, bool, i128, i64, publicKey, s32, seq, struct, u128, u16, u32, u64, u8 } from "../layout-type";
import { bnLayoutFormatter } from "./bn-layout-formatter";



export const CpPoolInfoLayout = struct([
  blob(8),

  publicKey("configId"),
  publicKey("poolCreator"),
  publicKey("vaultA"),
  publicKey("vaultB"),

  publicKey("mintLp"),
  publicKey("mintA"),
  publicKey("mintB"),

  publicKey("mintProgramA"),
  publicKey("mintProgramB"),

  publicKey("observationId"),

  u8("bump"),
  u8("status"),

  u8("lpDecimals"),
  u8("mintDecimalA"),
  u8("mintDecimalB"),

  u64("lpAmount"),
  u64("protocolFeesMintA"),
  u64("protocolFeesMintB"),
  u64("fundFeesMintA"),
  u64("fundFeesMintB"),
  u64("openTime"),

  seq(u64(), 32),
]);
export function decodePoolData(buffer: Buffer) {
  const cpDecode =  CpPoolInfoLayout.decode(buffer);
  bnLayoutFormatter(cpDecode);
  return cpDecode;
}

// Function to decode a base64-encoded transaction
export function decodeTransact(data: string) {
  return base58.encode(Buffer.from(data, 'base64'));
}
