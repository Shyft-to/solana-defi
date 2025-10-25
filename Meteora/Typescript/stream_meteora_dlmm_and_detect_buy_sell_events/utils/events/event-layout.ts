import { PublicKey } from "@solana/web3.js";
import {
  publicKey,
  u64,
  u128
} from "@solana/buffer-layout-utils";
import {
  struct,
  u8,
  s32,  // signed 32-bit (for i32)
  u8 as boolLayout
} from "@solana/buffer-layout";

// Define TypeScript interface for strongly typed output
export interface RawSwapEvent {
  lb_pair: PublicKey;
  from: PublicKey;
  start_bin_id: number;
  end_bin_id: number;
  amount_in: bigint;
  amount_out: bigint;
  swap_for_y: boolean;
  fee: bigint;
  protocol_fee: bigint;
  fee_bps: bigint;
  host_fee: bigint;
}

// Define the layout structure
export const SwapLayout = struct<RawSwapEvent>([
  publicKey("lb_pair"),
  publicKey("from"),
  s32("start_bin_id"),
  s32("end_bin_id"),
  u64("amount_in"),
  u64("amount_out"),
  boolLayout("swap_for_y"),
  u64("fee"),
  u64("protocol_fee"),
  u128("fee_bps"),
  u64("host_fee"),
]);

// Decode function
export function decodeSwap(hex: string) {
  const rawBuffer = Buffer.from(hex, "hex");

  const dataStart = 16; // Skip discriminator or header bytes if necessary
  const eventBuffer = rawBuffer.subarray(dataStart);

  const decoded = SwapLayout.decode(eventBuffer);

  return {
    lb_pair: decoded.lb_pair.toBase58(),
    from: decoded.from.toBase58(),
    start_bin_id: decoded.start_bin_id,
    end_bin_id: decoded.end_bin_id,
    amount_in: decoded.amount_in.toString(),
    amount_out: decoded.amount_out.toString(),
    swap_for_y: !!decoded.swap_for_y,
    fee: decoded.fee.toString(),
    protocol_fee: decoded.protocol_fee.toString(),
    fee_bps: decoded.fee_bps.toString(),
    host_fee: decoded.host_fee.toString(),
  };
}
