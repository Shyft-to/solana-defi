import { publicKey, struct, bool, u8, u32, u16, u64, Layout, option, i64, i32, u128, array } from "@coral-xyz/borsh";
import base58 from "bs58";

const VariableParameters = struct([
  u32('volatilityAccumulator'),
  u32('volatilityReference'),
  i32('indexReference'),
  array(u8(), 4, 'padding'),
  i64('lastUpdateTimestamp'),
  array(u8(), 8, 'padding1'),
]);

const StaticParameters = struct([
  u16('baseFactor'),
  u16('filterPeriod'),
  u16('decayPeriod'),
  u16('reductionFactor'),
  u32('variableFeeControl'),
  u32('maxVolatilityAccumulator'),
  i32('minBinId'),
  i32('maxBinId'),
  u16('protocolShare'),
  array(u8(), 6, 'padding'),
]);

const ProtocolFee = struct([
  u64('amountX'),
  u64('amountY'),
]);

const RewardInfo = struct([
  publicKey('mint'),
  publicKey('vault'),
  publicKey('funder'),
  u64('rewardDuration'),
  u64('rewardDurationEnd'),
  u128('rewardRate'),
  u64('lastUpdateTime'),
  u64('cumulativeSecondsWithEmptyLiquidityReward'),
]);

const PoolLayout = struct([
  ...StaticParameters.fields,
  ...VariableParameters.fields,
  array(u8(), 1, 'bumpSeed'),
  array(u8(), 2, 'binStepSeed'),
  u8('pairType'),
  i32('activeId'),
  u16('binStep'),
  u8('status'),
  u8('requireBaseFactorSeed'),
  array(u8(), 2, 'baseFactorSeed'),
  u8('activationType'),
  u8('padding0'),
  publicKey('tokenXMint'),
  publicKey('tokenYMint'),
  publicKey('reserveX'),
  publicKey('reserveY'),
  ProtocolFee,
  array(u8(), 32, 'padding1'),
  array(RewardInfo, 2, 'rewardInfos'),
  publicKey('oracle'),
  struct([u64('binArrayBitmap')]),
  i64('lastUpdatedAt'),
  array(u8(), 32, 'padding2'),
  publicKey('preActivationSwapAddress'),
  publicKey('baseKey'),
  u64('activationPoint'),
  u64('preActivationDuration'),
  array(u8(), 8, 'padding3'),
  u64('padding4'),
  publicKey('creator'),
  array(u8(), 24, 'reserved'),
]);

export function decodeTransact(data) {
  return base58.encode(Buffer.from(data, 'base64'));
}

export function decodePoolData(buffer: Buffer) {
  return PoolLayout.decode(buffer);
}
