import { PublicKey } from "@solana/web3.js";
import { publicKey, u64, u128 } from "@solana/buffer-layout-utils";
import { struct, u8, u8 as boolLayout } from "@solana/buffer-layout";

const TradeDirection = {
  Sell: 0,
  Buy: 1,
};

export interface SwapParameters2 {
  amountIn: bigint;
  minimumAmountOut: bigint;
}

export interface SwapResult2 {
  outputAmount: bigint;
  nextSqrtPrice: bigint;
  lpFee: bigint;
  protocolFee: bigint;
  partnerFee: bigint;
  referralFee: bigint;
}

export interface RawEvtSwap2 {
  pool: PublicKey;
  tradeDirection: number;
  collectFeeMode: number;
  hasReferral: boolean;
  params: SwapParameters2;
  swapResult: SwapResult2;
  includedTransferFeeAmountIn: bigint;
  includedTransferFeeAmountOut: bigint;
  excludedTransferFeeAmountOut: bigint;
  currentTimestamp: bigint;
  reserveAAmount: bigint;
  reserveBAmount: bigint;
}

const SwapParameters2Layout = struct<SwapParameters2>([
  u64("amountIn"),
  u64("minimumAmountOut"),
]);

const SwapResult2Layout = struct<SwapResult2>([
  u64("outputAmount"),
  u128("nextSqrtPrice"),
  u64("lpFee"),
  u64("protocolFee"),
  u64("partnerFee"),
  u64("referralFee"),
]);

export const EvtSwap2Layout = struct<RawEvtSwap2>([
  publicKey("pool"),
  u8("tradeDirection"),
  u8("collectFeeMode"),
  boolLayout("hasReferral"),
  SwapParameters2Layout.replicate("params"),
  SwapResult2Layout.replicate("swapResult"),
  u64("includedTransferFeeAmountIn"),
  u64("includedTransferFeeAmountOut"),
  u64("excludedTransferFeeAmountOut"),
  u64("currentTimestamp"),
  u64("reserveAAmount"),
  u64("reserveBAmount"),
]);

function mapTradeDirection(dir: number) {
  return dir === TradeDirection.Buy ? { buy: {} } : { sell: {} };
}

export function decodeEvtSwap(hex: string) {
  const rawBuffer = Buffer.from(hex, "hex");

  const dataStart = 16;
  const eventBuffer = rawBuffer.subarray(dataStart);

  const decoded = EvtSwap2Layout.decode(eventBuffer);

  return {
    pool: decoded.pool.toBase58(),
    tradeDirection: mapTradeDirection(decoded.tradeDirection),
    collectFeeMode: decoded.collectFeeMode,
    hasReferral: !!decoded.hasReferral,
    params: {
      amountIn: decoded.params.amountIn.toString(),
      minimumAmountOut: decoded.params.minimumAmountOut.toString(),
    },
    swapResult: {
      outputAmount: decoded.swapResult.outputAmount.toString(),
      nextSqrtPrice: decoded.swapResult.nextSqrtPrice.toString(),
      lpFee: decoded.swapResult.lpFee.toString(),
      protocolFee: decoded.swapResult.protocolFee.toString(),
      partnerFee: decoded.swapResult.partnerFee.toString(),
      referralFee: decoded.swapResult.referralFee.toString(),
    },
    includedTransferFeeAmountIn: decoded.includedTransferFeeAmountIn.toString(),
    includedTransferFeeAmountOut: decoded.includedTransferFeeAmountOut.toString(),
    excludedTransferFeeAmountOut: decoded.excludedTransferFeeAmountOut.toString(),
    currentTimestamp: decoded.currentTimestamp.toString(),
    reserveAAmount: decoded.reserveAAmount.toString(),
    reserveBAmount: decoded.reserveBAmount.toString(),
  };
}