import { PublicKey } from "@solana/web3.js";
import { publicKey, u64 } from "@solana/buffer-layout-utils";
import { struct, u8 as boolLayout, s32 } from "@solana/buffer-layout";


export interface RawEvtSwap {
  lbPair: PublicKey;
  from: PublicKey;
  startBinId: number;
  endBinId: number;
  amountIn: bigint;
  amountOut: bigint;
  swapForY: boolean;
  fee: bigint;
  protocolFee: bigint;
  feeBps: bigint;
  hostFee: bigint;
}

export const EvtDlmmSwapLayout = struct<RawEvtSwap>([
  publicKey("lbPair"),
  publicKey("from"),
  s32("startBinId"),
  s32("endBinId"),
  u64("amountIn"),
  u64("amountOut"),
  boolLayout("swapForY"),
  u64("fee"),
  u64("protocolFee"),
  u64("feeBps"),
  u64("hostFee"),
]);


export function decodeEvtSwap(hex: string) {
  const rawBuffer = Buffer.from(hex, "hex");

  const dataStart = 16; 
  const eventBuffer = rawBuffer.subarray(dataStart);

  const decoded = EvtDlmmSwapLayout.decode(eventBuffer);

  return {
    lbPair: decoded.lbPair.toBase58(),
    from: decoded.from.toBase58(),
    startBinId: decoded.startBinId,
    endBinId: decoded.endBinId,
    amountIn: decoded.amountIn.toString(),
    amountOut: decoded.amountOut.toString(),
    swapForY: !!decoded.swapForY,
    fee: decoded.fee.toString(),
    protocolFee: decoded.protocolFee.toString(),
    feeBps: decoded.feeBps.toString(),
    hostFee: decoded.hostFee.toString(),
  };
}