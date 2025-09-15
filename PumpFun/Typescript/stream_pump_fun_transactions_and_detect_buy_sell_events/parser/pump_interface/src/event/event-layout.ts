import { PublicKey } from "@solana/web3.js";
import { bool, publicKey, u64 } from "@solana/buffer-layout-utils";
import { struct, u8 } from "@solana/buffer-layout";


import { blob } from "@solana/buffer-layout";

export const i64 = (property: string = 'i64') => {
  return blob(8, property);
};

export interface TradeEvent {
  mint: PublicKey;
  sol_amount: bigint;
  token_amount: bigint;
  is_buy: boolean;
  user: PublicKey;
  timestamp: bigint;
  virtual_sol_reserves: bigint;
  virtual_token_reserves: bigint;
  real_sol_reserves: bigint;
  real_token_reserves: bigint;
  fee_recipient: PublicKey;
  fee_basis_points: bigint;
  fee: bigint;
  creator: PublicKey;
  creator_fee_basis_points: bigint;
  creator_fee: bigint;
  track_volume: boolean;
  total_unclaimed_tokens: bigint;
  total_claimed_tokens: bigint;
  current_sol_volume: bigint;
  last_update_timestamp: bigint;
}

export const TradeEventLayout = struct<TradeEvent>([
  publicKey("mint"),
  u64("sol_amount"),
  u64("token_amount"),
  bool("is_buy"),
  publicKey("user"),
  u64("timestamp"),
  u64("virtual_sol_reserves"),
  u64("virtual_token_reserves"),
  u64("real_sol_reserves"),
  u64("real_token_reserves"),
  publicKey("fee_recipient"),
  u64("fee_basis_points"),
  u64("fee"),
  publicKey("creator"),
  u64("creator_fee_basis_points"),
  u64("creator_fee"),
  bool("track_volume"),
  u64("total_unclaimed_tokens"),
  u64("total_claimed_tokens"),
  u64("current_sol_volume"),
  u64("last_update_timestamp"),
]);

export function decodeTradeEvent(hex: string) {
  try {
    const rawBuffer = Buffer.from(hex, "hex");
    const dataStart = 0; 
    const eventBuffer = rawBuffer.subarray(dataStart);

    const decoded = TradeEventLayout.decode(eventBuffer);

    return {
      mint: decoded.mint.toBase58(),
      sol_amount: decoded.sol_amount.toString(),
      token_amount: decoded.token_amount.toString(),
      is_buy: decoded.is_buy,
      user: decoded.user.toBase58(),
      timestamp: Number(decoded.timestamp), 
      virtual_sol_reserves: decoded.virtual_sol_reserves.toString(),
      virtual_token_reserves: decoded.virtual_token_reserves.toString(),
      real_sol_reserves: decoded.real_sol_reserves.toString(),
      real_token_reserves: decoded.real_token_reserves.toString(),
      fee_recipient: decoded.fee_recipient.toBase58(),
      fee_basis_points: decoded.fee_basis_points.toString(),
      fee: decoded.fee.toString(),
      creator: decoded.creator.toBase58(),
      creator_fee_basis_points: decoded.creator_fee_basis_points.toString(),
      creator_fee: decoded.creator_fee.toString(),
      track_volume: decoded.track_volume,
      total_unclaimed_tokens: decoded.total_unclaimed_tokens.toString(),
      total_claimed_tokens: decoded.total_claimed_tokens.toString(),
      current_sol_volume: decoded.current_sol_volume.toString(),
      last_update_timestamp: Number(decoded.last_update_timestamp), // Convert to number
    };
  } catch (error) {
    console.error("Error decoding trade event:", error);
    return null;
  }
}
