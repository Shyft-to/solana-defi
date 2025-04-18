import base58 from "bs58";
import { blob, publicKey, struct, u64, u8, u16, seq } from "../layout-type";
import { bnLayoutFormatter } from "./bn-layout-formatter";

export const GLOBAL_CONFIG_DISC = new Uint8Array([149, 8, 156, 202, 160, 252, 176, 217]);
export const GlobalConfigLayout = struct([
  u8("curveType"),
  u16("index"),
  u64("migrateFee"),
  u64("tradeFeeRate"),
  u64("maxShareFeeRate"),
  u64("minBaseSupply"),
  u64("maxLockRate"),
  u64("minBaseSellRate"),
  u64("minBaseMigrateRate"),
  u64("minQuoteFundRaising"),
  publicKey("quoteMint"),
  publicKey("protocolFeeOwner"),
  publicKey("migrateFeeOwner"),
  publicKey("migrateToAmmWallet"),
  publicKey("migrateToCpswapWallet"),
  seq(u64(), 16, "padding"),
]);

export const PLATFORM_CONFIG_DISC = new Uint8Array([160, 78, 128, 0, 248, 83, 230, 160]);
export const PlatformConfigLayout = struct([
  u64("epoch"),
  publicKey("platformFeeWallet"),
  publicKey("platformNftWallet"),
  u64("platformScale"),
  u64("creatorScale"),
  u64("burnScale"),
  u64("feeRate"),
  blob(64, "name"),
  blob(256, "web"),
  blob(256, "img"),
  blob(256, "padding"),
]);
export const LaunchpadVestingSchedule = struct([
  u64("totalLockedAmount"),
  u64("cliffPeriod"),
  u64("unlockPeriod"),
  u64("startTime"),
  u64("totalAllocatedShare"),
]);

export const POOLSTATE_DISC = new Uint8Array([247, 237, 227, 245, 215, 195, 222, 70]);
export const LaunchpadPool = struct([
  u64(),
  u64("epoch"),
  u8("authBump"),
  u8("status"),
  u8("baseDecimals"),
  u8("quoteDecimals"),
  u8("migrateType"),

  u64("supply"),
  u64("totalBaseSell"),
  u64("virtualBase"),
  u64("virtualQuote"),
  u64("realBase"),
  u64("realQuote"),

  u64("totalQuoteFundRaising"),
  u64("quoteProtocolFee"),
  u64("platformFee"),
  u64("migrateFee"),

  LaunchpadVestingSchedule.replicate("vestingSchedule"),

  publicKey("globalConfig"),
  publicKey("platformConfig"),
  publicKey("baseMint"),
  publicKey("quoteMint"),
  publicKey("baseVault"),
  publicKey("quoteVault"),

  publicKey("creator"),

  seq(u64(), 8),
]);

export const LaunchpadVesting = struct([
  u64(),
  u64("epoch"),
  publicKey("poolId"),
  publicKey("beneficiary"),
  u64("claimedAmount"),
  u64("tokenShareAmount"),
  seq(u64(), 8),
]);
export const VESTING_RECORD_DISC = new Uint8Array([106, 243, 221, 205, 230, 126, 85, 83]);
export const VestingRecordLayout = struct([
  u64("epoch"),
  publicKey("pool"),
  publicKey("beneficiary"),
  u64("claimedAmount"),
  u64("tokenShareAmount"),
  seq(u64(), 8, "padding"),
]);

export function decodeGlobalConfig(buffer: Buffer) {
  const decoded = GlobalConfigLayout.decode(buffer);
  bnLayoutFormatter(decoded);
  return decoded;
}

export function decodePlatformConfig(buffer: Buffer) {
  const decoded = PlatformConfigLayout.decode(buffer);
  bnLayoutFormatter(decoded);
  return decoded;
}

export function decodePoolState(buffer: Buffer) {
  const decoded = LaunchpadPool.decode(buffer);
  bnLayoutFormatter(decoded);
  return decoded;
}

export function decodeVestingRecord(buffer: Buffer) {
  const decoded = VestingRecordLayout.decode(buffer);
  bnLayoutFormatter(decoded);
  return decoded;
}
export function base64ToBase58(data: string) {
  return base58.encode(Buffer.from(data, 'base64'));
}