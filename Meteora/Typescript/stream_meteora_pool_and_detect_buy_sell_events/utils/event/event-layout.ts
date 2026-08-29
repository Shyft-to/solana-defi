import { PublicKey } from "@solana/web3.js";
import { publicKey, u64 } from "@solana/buffer-layout-utils";
import { struct, u8 as boolLayout, s32 } from "@solana/buffer-layout";


export interface RawEvtSwap {
  inAmount: bigint;
  outAmount: bigint;
  tradeFee: bigint;
  protocolFee: bigint;
  hostFee: bigint;
}

export const EvtDlmmSwapLayout = struct<RawEvtSwap>([
  u64("inAmount"),
  u64("outAmount"),
  u64("tradeFee"),
  u64("protocolFee"),
  u64("hostFee"),
]);

export function decodeEvtSwap(hex: string) {
  const rawBuffer = Buffer.from(hex, "hex");

  const dataStart = 8; // 8-byte Anchor discriminator, NOT 16
  const eventBuffer = rawBuffer.subarray(dataStart);

  const decoded = EvtDlmmSwapLayout.decode(eventBuffer);
  return {
    amountIn: decoded.inAmount.toString(),
    amountOut: decoded.outAmount.toString(),
    fee: decoded.tradeFee.toString(),
    protocolFee: decoded.protocolFee.toString(),
    hostFee: decoded.hostFee.toString(),
  };
}