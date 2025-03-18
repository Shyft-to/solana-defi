
import base58 from "bs58";
import { blob, bool, i128, i64, publicKey, s32, seq, struct, u128, u16, u32, u64, u8 } from "../marshmallow";
import { bnLayoutFormatter } from "./bn-layout-formatter";



export const RewardInfo = struct([
  u8("rewardState"),
  u64("openTime"),
  u64("endTime"),
  u64("lastUpdateTime"),
  u128("emissionsPerSecondX64"),
  u64("rewardTotalEmissioned"),
  u64("rewardClaimed"),
  publicKey("tokenMint"),
  publicKey("tokenVault"),
  publicKey("creator"),
  u128("rewardGrowthGlobalX64"),
]);
export const PoolLayout = struct([
  blob(8),
  u8("bump"),
  publicKey("ammConfig"),
  publicKey("creator"),
  publicKey("mintA"),
  publicKey("mintB"),
  publicKey("vaultA"),
  publicKey("vaultB"),
  publicKey("observationId"),
  u8("mintDecimalsA"),
  u8("mintDecimalsB"),
  u16("tickSpacing"),
  u128("liquidity"),
  u128("sqrtPriceX64"),
  s32("tickCurrent"),
  u32(),
  u128("feeGrowthGlobalX64A"),
  u128("feeGrowthGlobalX64B"),
  u64("protocolFeesTokenA"),
  u64("protocolFeesTokenB"),

  u128("swapInAmountTokenA"),
  u128("swapOutAmountTokenB"),
  u128("swapInAmountTokenB"),
  u128("swapOutAmountTokenA"),

  u8("status"),

  seq(u8(), 7, ""),

  seq(RewardInfo, 3, "rewardInfos"),
  seq(u64(), 16, "tickArrayBitmap"),

  u64("totalFeesTokenA"),
  u64("totalFeesClaimedTokenA"),
  u64("totalFeesTokenB"),
  u64("totalFeesClaimedTokenB"),

  u64("fundFeesTokenA"),
  u64("fundFeesTokenB"),

  u64("startTime"),

  seq(u64(), 15 * 4 - 3, "padding"),
]);

export function decodePoolData(buffer: Buffer) {
  const clmmDecode =  PoolLayout.decode(buffer);
  bnLayoutFormatter(clmmDecode);
  return clmmDecode;
}

// Function to decode a base64-encoded transaction
export function decodeTransact(data: string) {
  return base58.encode(Buffer.from(data, 'base64'));
}
