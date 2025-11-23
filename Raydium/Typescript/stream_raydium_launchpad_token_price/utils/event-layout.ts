import { PublicKey } from "@solana/web3.js";
import { publicKey, u64 } from "@solana/buffer-layout-utils";
import { struct, u8 } from "@solana/buffer-layout";

const TradeDirection = {
  Buy: 0,
  Sell: 1,
};

const PoolStatus = {
  Fund: 0,
  Migrate: 1,
  Trade: 2,
};

export interface RawTradeEvent {
  poolState: PublicKey;
  totalBaseSell: bigint;  
  virtualBase: bigint;    
  virtualQuote: bigint;   
  realBaseBefore: bigint; 
  realQuoteBefore: bigint; 
  realBaseAfter: bigint;  
  realQuoteAfter: bigint; 
  amountIn: bigint;       
  amountOut: bigint;      
  protocolFee: bigint;    
  platformFee: bigint;    
  shareFee: bigint;       
  tradeDirection: number; 
  poolStatus: number;     
}

export const TradeEventLayout = struct<RawTradeEvent>([
  publicKey("poolState"),
  u64("totalBaseSell"), 
  u64("virtualBase"),
  u64("virtualQuote"),
  u64("realBaseBefore"),
  u64("realQuoteBefore"),
  u64("realBaseAfter"),
  u64("realQuoteAfter"),
  u64("amountIn"),
  u64("amountOut"),
  u64("protocolFee"),
  u64("platformFee"),
  u64("shareFee"),
  u8("tradeDirection"),
  u8("poolStatus"),
]);

function mapTradeDirection(dir: number) {
  return dir === TradeDirection.Buy ? { buy: {} } : { sell: {} };
}

function mapPoolStatus(status: number) {
  switch (status) {
    case PoolStatus.Fund: return { fund: {} };
    case PoolStatus.Migrate: return { migrate: {} };
    case PoolStatus.Trade: return { trade: {} };
    default: return status;
  }
}

export function decodeTradeEvent(hex: string) {
  const rawBuffer = Buffer.from(hex, "hex");
  
  const dataStart = 16; 
  const eventBuffer = rawBuffer.subarray(dataStart);

  const decoded = TradeEventLayout.decode(eventBuffer);

  return {
    poolState: decoded.poolState.toBase58(),
    totalBaseSell: decoded.totalBaseSell.toString(),
    virtualBase: decoded.virtualBase.toString(),
    virtualQuote: decoded.virtualQuote.toString(),
    realBaseBefore: decoded.realBaseBefore.toString(),
    realQuoteBefore: decoded.realQuoteBefore.toString(),
    realBaseAfter: decoded.realBaseAfter.toString(),
    realQuoteAfter: decoded.realQuoteAfter.toString(),
    amountIn: decoded.amountIn.toString(),
    amountOut: decoded.amountOut.toString(),
    protocolFee: decoded.protocolFee.toString(),
    platformFee: decoded.platformFee.toString(),
    shareFee: decoded.shareFee.toString(),
    tradeDirection: mapTradeDirection(decoded.tradeDirection),
    poolStatus: mapPoolStatus(decoded.poolStatus),
  };
}