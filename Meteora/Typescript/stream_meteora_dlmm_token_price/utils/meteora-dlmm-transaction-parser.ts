import { BN } from "@coral-xyz/anchor";

export function meteoraDlmmParsedTransaction(parsedInstruction: any, txn: any): any {
  const priceInfo = computeSwapPrice(parsedInstruction, txn);
  return priceInfo;
}
function computeSwapPrice(parsedInstruction: any, txn: any): any {
  try {
    const tradeEvent = parsedInstruction?.instructions?.events?.find(
      (ev: any) => ev.name === "TradeEvent"
    );
    if (!tradeEvent) return null;

   
    const swapIx =
      parsedInstruction?.inner_ixs?.find(
        (ix: any) => ix.name === "swap" || ix.name === "swap2"
      ) ??
      parsedInstruction?.instructions?.meteoraDlmmIxs?.find(
        (ix: any) => ix.name === "swap" || ix.name === "swap2"
      );
    if (!swapIx) return null;

    const tokenXMint = swapIx.accounts.find(
      (acc: any) => acc.name === "token_x_mint"
    )?.pubkey;
    const tokenYMint = swapIx.accounts.find(
      (acc: any) => acc.name === "token_y_mint"
    )?.pubkey;
    if (!tokenXMint || !tokenYMint) return null;

    const mintDecimals = new Map<string, number>();
    const balances = [
      ...(txn?.meta?.preTokenBalances ?? []),
      ...(txn?.meta?.postTokenBalances ?? []),
    ];
    for (const bal of balances) {
      if (bal.mint && !mintDecimals.has(bal.mint)) {
        mintDecimals.set(bal.mint, bal.uiTokenAmount.decimals);
      }
    }

    let decimalsX = mintDecimals.get(tokenXMint);
    let decimalsY = mintDecimals.get(tokenYMint);

    const KNOWN_DECIMALS: Record<string, number> = {
      So11111111111111111111111111111111111111112: 9,  
      EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v: 6, 
      Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB: 6, 
    };
    if (decimalsX === undefined && KNOWN_DECIMALS[tokenXMint] !== undefined) {
      decimalsX = KNOWN_DECIMALS[tokenXMint];
    }
    if (decimalsY === undefined && KNOWN_DECIMALS[tokenYMint] !== undefined) {
      decimalsY = KNOWN_DECIMALS[tokenYMint];
    }

    if (decimalsX === undefined || decimalsY === undefined) {
      console.warn(
        `Missing decimals — X(${tokenXMint}): ${decimalsX}, Y(${tokenYMint}): ${decimalsY}`
      );
      return null;
    }

    const amountInRaw  = new BN(tradeEvent.data.amountIn);
    const amountOutRaw = new BN(tradeEvent.data.amountOut);
    const swapForY: boolean = tradeEvent.data.swapForY;

   
    let tokenInMint: string;
    let tokenOutMint: string;
    let amountInHuman: number;
    let amountOutHuman: number;

    if (swapForY) {
      tokenInMint    = tokenXMint;
      tokenOutMint   = tokenYMint;
      amountInHuman  = amountInRaw.toNumber()  / Math.pow(10, decimalsX);
      amountOutHuman = amountOutRaw.toNumber() / Math.pow(10, decimalsY);
    } else {
      tokenInMint    = tokenYMint;
      tokenOutMint   = tokenXMint;
      amountInHuman  = amountInRaw.toNumber()  / Math.pow(10, decimalsY);
      amountOutHuman = amountOutRaw.toNumber() / Math.pow(10, decimalsX);
    }

    const price = amountOutHuman / amountInHuman;

    const priceOfXInYRaw = swapForY
      ? amountOutHuman / amountInHuman   
      : amountInHuman  / amountOutHuman; 

    const TOKEN_SYMBOLS: Record<string, string> = {
      So11111111111111111111111111111111111111112:   "SOL",
      EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v: "USDC",
      Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB: "USDT",
    };
    const quoteSymbol = TOKEN_SYMBOLS[tokenYMint] ?? tokenYMint.slice(0, 6);
    const priceOfXInY = `${priceOfXInYRaw.toFixed(8)} ${quoteSymbol}`;

    // Fee info
    const fee         = Number(tradeEvent.data.fee);
    const protocolFee = Number(tradeEvent.data.protocolFee);
    const feeBps      = Number(tradeEvent.data.feeBps);

    return {
      lbPair: tradeEvent.data.lbPair,
      from:   tradeEvent.data.from,

      swapDirection: swapForY ? "Sell" : "Buy",

      tokenInMint,
      tokenOutMint,

      amountIn:  amountInHuman,
      amountOut: amountOutHuman,

      amountInRaw:  amountInRaw.toString(),
      amountOutRaw: amountOutRaw.toString(),

      price,
      priceOfXInY,

      decimals: {
        [tokenXMint]: decimalsX,
        [tokenYMint]: decimalsY,
      },

      binId: {
        start: tradeEvent.data.startBinId,
        end:   tradeEvent.data.endBinId,
      },

      fees: {
        fee,
        protocolFee,
        feeBps,
        feePercent: feeBps / 1_000_000, 
      },
    };
  } catch (error) {
    console.error("Error computing swap price:", error);
    return null;
  }
}