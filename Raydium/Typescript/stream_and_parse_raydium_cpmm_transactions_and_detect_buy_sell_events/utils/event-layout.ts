import { PublicKey } from "@solana/web3.js";
import { publicKey, u64 } from "@solana/buffer-layout-utils";
import { struct, u8 } from "@solana/buffer-layout";

/**
 * SwapEvent structure (from Raydium CP-Swap)
 */
export interface SwapEvent {
  poolId: PublicKey;
  inputVaultBefore: bigint;
  outputVaultBefore: bigint;
  inputAmount: bigint;
  outputAmount: bigint;
  inputTransferFee: bigint;
  outputTransferFee: bigint;
  baseInput: boolean;
  inputMint: PublicKey;
  outputMint: PublicKey;
  tradeFee: bigint;
  creatorFee: bigint;
  creatorFeeOnInput: boolean;
}

export const SwapEventLayout = struct<any>([
  publicKey("poolId"),
  u64("inputVaultBefore"),
  u64("outputVaultBefore"),
  u64("inputAmount"),
  u64("outputAmount"),
  u64("inputTransferFee"),
  u64("outputTransferFee"),
  u8("baseInput"), 
  publicKey("inputMint"),
  publicKey("outputMint"),
  u64("tradeFee"),
  u64("creatorFee"),
  u8("creatorFeeOnInput"), 
]);


export function decodeSwapEvent(data: string, encoding: "base64" | "hex" = "base64") {
  const buf = Buffer.from(data, encoding);

  // If prefixed by Anchor 8-byte discriminator, skip first 8 bytes
  const eventBuf = buf.length % 8 === 0 ? buf : buf.subarray(8);

  const decoded = SwapEventLayout.decode(eventBuf);

  return {
    poolId: decoded.poolId.toBase58(),
    inputVaultBefore: decoded.inputVaultBefore.toString(),
    outputVaultBefore: decoded.outputVaultBefore.toString(),
    inputAmount: decoded.inputAmount.toString(),
    outputAmount: decoded.outputAmount.toString(),
    inputTransferFee: decoded.inputTransferFee.toString(),
    outputTransferFee: decoded.outputTransferFee.toString(),
    baseInput: !!decoded.baseInput,
    inputMint: decoded.inputMint.toBase58(),
    outputMint: decoded.outputMint.toBase58(),
    tradeFee: decoded.tradeFee.toString(),
    creatorFee: decoded.creatorFee.toString(),
    creatorFeeOnInput: !!decoded.creatorFeeOnInput,
  };
}
