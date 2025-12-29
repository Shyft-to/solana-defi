import { PublicKey } from "@solana/web3.js";
import { publicKey, u64, u128 } from "@solana/buffer-layout-utils";
import { struct, u8, u8 as boolLayout } from "@solana/buffer-layout";


const TradeDirection = {
  Sell: 0,
  Buy: 1,
};

export interface SwapParameters {
  amountIn: bigint;
  minimumAmountOut: bigint;
}

export interface SwapResult {
  actualInputAmount: bigint;
  outputAmount: bigint;
  nextSqrtPrice: bigint;
  tradingFee: bigint;
  protocolFee: bigint;
  referralFee: bigint;
}

export interface RawEvtSwap {
  pool: PublicKey;
  config: PublicKey;
  tradeDirection: number;
  hasReferral: boolean;
  params: SwapParameters;
  swapResult: SwapResult;
  amountIn: bigint;
  currentTimestamp: bigint;
}


const SwapParametersLayout = struct<SwapParameters>([
  u64("amountIn"),
  u64("minimumAmountOut"),
]);

const SwapResultLayout = struct<SwapResult>([
  u64("actualInputAmount"),
  u64("outputAmount"),
  u128("nextSqrtPrice"),
  u64("tradingFee"),
  u64("protocolFee"),
  u64("referralFee"),
]);


export const EvtSwapLayout = struct<RawEvtSwap>([
  publicKey("pool"),
  publicKey("config"),
  u8("tradeDirection"),
  boolLayout("hasReferral"), 
  SwapParametersLayout.replicate("params"),
  SwapResultLayout.replicate("swapResult"),
  u64("amountIn"),
  u64("currentTimestamp"),
]);

function mapTradeDirection(dir: number) {
  return dir === TradeDirection.Buy ? { buy: {} } : { sell: {} };
}

export function decodeEvtSwap(hex: string) {
  const rawBuffer = Buffer.from(hex, "hex");

  const dataStart = 16; 
  const eventBuffer = rawBuffer.subarray(dataStart);

  const decoded = EvtSwapLayout.decode(eventBuffer);

  return {
    pool: decoded.pool.toBase58(),
    config: decoded.config.toBase58(),
    tradeDirection: mapTradeDirection(decoded.tradeDirection),
    hasReferral: !!decoded.hasReferral,
    params: {
      amountIn: decoded.params.amountIn.toString(),
      minimumAmountOut: decoded.params.minimumAmountOut.toString(),
    },
    swapResult: {
      actualInputAmount: decoded.swapResult.actualInputAmount.toString(),
      outputAmount: decoded.swapResult.outputAmount.toString(),
      nextSqrtPrice: decoded.swapResult.nextSqrtPrice.toString(),
      tradingFee: decoded.swapResult.tradingFee.toString(),
      protocolFee: decoded.swapResult.protocolFee.toString(),
      referralFee: decoded.swapResult.referralFee.toString(),
    },
    amountIn: decoded.amountIn.toString(),
    currentTimestamp: decoded.currentTimestamp.toString(),
  };
}
