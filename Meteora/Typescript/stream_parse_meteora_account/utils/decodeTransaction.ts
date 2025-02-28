import { publicKey,struct, bool,u8,u32,u16, u64, Layout, option } from "@coral-xyz/borsh";
//import { publicKey as pk } from "@solana/buffer-layout-utils";
import base58 from "bs58";
// Define the layout structure for LbPair


const PoolFees = struct([
  u64('tradeFeeNumerator'),
  u64('tradeFeeDenominator'),
  u64('protocolTradeFeeNumerator'),
  u64('protocolTradeFeeDenominator'),
]);

const PoolType = struct([
  u8('value'),  // We'll store the enum as a u8 value (0 or 1)
]);

const Bootstrapping =struct([
    u64('activationPoint'),
    publicKey('whitelistedVault'),
    publicKey('poolCreator'),
    u8('activationType'),
  ]);


const PartnerInfo = struct([
  u64("feeNumerator"),
  publicKey('partnerAddress'),
  u64('pendingFeeA'),
  u64('pendingFeeB'),
]);

const Padding = struct([u8('padding', 24)]);

const CurveType = struct([
  u8('value'),  // Represent curve types as u8 (0 for CONSTANT_PRODUCT, 1 for STABLE_SWAP)
]);

const PoolLayout = struct([
  publicKey('lpMint'),
  publicKey('tokenAMint'),
  publicKey('tokenBMint'),
  publicKey('aVault'),
  publicKey('bVault'),
  publicKey('aVaultLp'),
  publicKey('bVaultLp'),
  u8('aVaultLpBump'),
  u8('enabled'),
  publicKey('protocolTokenAFee'),
  publicKey('protocolTokenBFee'),
  u64('feeLastUpdatedAt'),
  Padding,  // padding0
  PoolFees,
  PoolType,
  publicKey('stake'),
  u64('totalLockedLp'),
  Bootstrapping,
  PartnerInfo,
  Padding,  // padding
  CurveType,
]);



export function decodeTransact(data){
    const output = base58.encode(Buffer.from(data,'base64'))
    return output;
}


export function decodePoolData(buffer: Buffer) {
  let decoded = PoolLayout.decode(buffer);
  return decoded;
}
