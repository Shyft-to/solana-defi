export function parseRaydiumSwapTransaction(parsedInstruction, txn) {
    const SOL_MINT = 'So11111111111111111111111111111111111111112';
    const USDC_MINT = 'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v';
    const USDT_MINT = 'Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB';
    
    let swapInstruction = 
        parsedInstruction?.instructions?.find(
            instruction => instruction.name === 'swapBaseIn' || instruction.name === 'swapBaseOut'
        ) ||
        parsedInstruction?.inner_ixs?.find(
            instruction => instruction.name === 'swapBaseIn' || instruction.name === 'swapBaseOut'
        );

    if (!swapInstruction) return null;

    const swapEvent = parsedInstruction?.events?.find(e => e.name === 'swapBaseIn' || e.name === 'swapBaseOut');
    if (!swapEvent) return null;

    const eventData = swapEvent.data;
    
    const amountIn = eventData.amountIn ?? eventData.maxIn;
    const amountOut = eventData.outAmount ?? eventData.amountOut;
    const direction = eventData.direction;
    const poolCoinReserves = eventData.poolCoin;  
    const poolPcReserves = eventData.poolPc;     
    
    const preTokenBalances = txn?.meta?.preTokenBalances || [];
    const postTokenBalances = txn?.meta?.postTokenBalances || [];
    const allTokenBalances = [...preTokenBalances, ...postTokenBalances];
    
    const poolCoinTokenAccount = swapInstruction?.accounts?.find(
        account => account.name === 'poolCoinTokenAccount'
    )?.pubkey;
    
    const poolPcTokenAccount = swapInstruction?.accounts?.find(
        account => account.name === 'poolPcTokenAccount'
    )?.pubkey;
    
    let coinMint, pcMint;
    let coinDecimals = 6, pcDecimals = 6;
    
    for (const balance of allTokenBalances) {
        const accountIndex = balance.accountIndex;
        if (accountIndex >= txn.transaction.message.staticAccountKeys.length) continue;
        
        const accountPubkey = txn.transaction.message.staticAccountKeys[accountIndex].toString();
        
        if (accountPubkey === poolCoinTokenAccount) {
            coinMint = balance.mint;
            coinDecimals = balance.uiTokenAmount?.decimals || coinDecimals;
        }
        if (accountPubkey === poolPcTokenAccount) {
            pcMint = balance.mint;
            pcDecimals = balance.uiTokenAmount?.decimals || pcDecimals;
        }
    }
    
    if (!coinMint || !pcMint) {
        const uniqueMints = [...new Set(allTokenBalances.map(b => b.mint))];
        if (uniqueMints.length >= 2) {
            const quoteCandidates = [SOL_MINT, USDC_MINT, USDT_MINT];
            const quoteMint = uniqueMints.find(m => quoteCandidates.includes(m));
            
            if (quoteMint) {
                pcMint = quoteMint;
                coinMint = uniqueMints.find(m => m !== quoteMint) || uniqueMints[0];
            } else {
                coinMint = uniqueMints[0];
                pcMint = uniqueMints[1];
            }
            
            const coinBalance = allTokenBalances.find(b => b.mint === coinMint);
            const pcBalance = allTokenBalances.find(b => b.mint === pcMint);
            
            if (coinBalance?.uiTokenAmount?.decimals) coinDecimals = coinBalance.uiTokenAmount.decimals;
            if (pcBalance?.uiTokenAmount?.decimals) pcDecimals = pcBalance.uiTokenAmount.decimals;
        }
    }
    
    if (!coinMint || !pcMint) return null;
    
    let sourceMint: string, destMint: string;
    let sourceDecimals: number, destDecimals: number;
    
    if (direction === 1) {
        sourceMint = coinMint;
        destMint = pcMint;
        sourceDecimals = coinDecimals;
        destDecimals = pcDecimals;
    } else {
        sourceMint = pcMint;
        destMint = coinMint;
        sourceDecimals = pcDecimals;
        destDecimals = coinDecimals;
    }
    
    const amountInDecimal = Number(amountIn) / Math.pow(10, sourceDecimals);
    const amountOutDecimal = Number(amountOut) / Math.pow(10, destDecimals);
    
    if (amountOutDecimal === 0) return null;
        
    const coinReservesDecimal = Number(poolCoinReserves) / Math.pow(10, coinDecimals);
    const pcReservesDecimal = Number(poolPcReserves) / Math.pow(10, pcDecimals);
    
    
    let baseTokenPrice: number;  
    let quoteTokenPrice: number; 
    
    if (direction === 1) {
        baseTokenPrice = amountOutDecimal / amountInDecimal;
        quoteTokenPrice = amountInDecimal / amountOutDecimal;
    } else {
        baseTokenPrice = amountInDecimal / amountOutDecimal;
        quoteTokenPrice = amountOutDecimal / amountInDecimal;
    }
    
    let poolBaseTokenPrice = 0;
    let poolQuoteTokenPrice = 0;
    
    if (coinReservesDecimal > 0 && pcReservesDecimal > 0) {
        poolBaseTokenPrice = pcReservesDecimal / coinReservesDecimal;  
        poolQuoteTokenPrice = coinReservesDecimal / pcReservesDecimal; 
    }
    
    const baseSymbol = getTokenSymbol(coinMint);
    const quoteSymbol = getTokenSymbol(pcMint);
    
    let basePriceDisplay: string;
    let quotePriceDisplay: string;
    let poolBasePriceDisplay: string;
    let poolQuotePriceDisplay: string;
    
    if (pcMint === SOL_MINT) {
        basePriceDisplay = `${formatSmallPrice(baseTokenPrice)} SOL`;
        quotePriceDisplay = `${formatLargePrice(quoteTokenPrice)} ${baseSymbol}`;
        poolBasePriceDisplay = `${formatSmallPrice(poolBaseTokenPrice)} SOL`;
        poolQuotePriceDisplay = `${formatLargePrice(poolQuoteTokenPrice)} ${baseSymbol}`;
    } 
    else if (pcMint === USDC_MINT || pcMint === USDT_MINT) {
        basePriceDisplay = `$${formatSmallPrice(baseTokenPrice)}`;
        quotePriceDisplay = `${formatLargePrice(quoteTokenPrice)} ${baseSymbol}`;
        poolBasePriceDisplay = `$${formatSmallPrice(poolBaseTokenPrice)}`;
        poolQuotePriceDisplay = `${formatLargePrice(poolQuoteTokenPrice)} ${baseSymbol}`;
    }
    else {
        basePriceDisplay = `${formatSmallPrice(baseTokenPrice)} ${quoteSymbol}`;
        quotePriceDisplay = `${formatLargePrice(quoteTokenPrice)} ${baseSymbol}`;
        poolBasePriceDisplay = `${formatSmallPrice(poolBaseTokenPrice)} ${quoteSymbol}`;
        poolQuotePriceDisplay = `${formatLargePrice(poolQuoteTokenPrice)} ${baseSymbol}`;
    }
    
    let priceImpact = 'N/A';
    if (poolBaseTokenPrice > 0) {
        const impact = ((baseTokenPrice - poolBaseTokenPrice) / poolBaseTokenPrice) * 100;
        priceImpact = `${impact.toFixed(2)}%`;
    }
    
    return {
        base_mint: coinMint,
        quote_mint: pcMint,
        amount_in: amountIn,
        amount_out: amountOut,
        amount_in_formatted: `${formatNumber(amountInDecimal)} ${getTokenSymbol(sourceMint)}`,
        amount_out_formatted: `${formatNumber(amountOutDecimal)} ${getTokenSymbol(destMint)}`,
        pool_base_reserves: poolCoinReserves,
        pool_quote_reserves: poolPcReserves,
        pool_base_reserves_decimals: coinReservesDecimal,
        pool_quote_reserves_decimals: pcReservesDecimal,                
        base_token_price: baseTokenPrice,          
        quote_token_price: quoteTokenPrice,        
        pool_base_token_price: poolBaseTokenPrice,  
        pool_quote_token_price: poolQuoteTokenPrice,
        swap_type: swapInstruction.name,
        direction: direction,
        pool_price: poolBasePriceDisplay
    };
}
function formatSmallPrice(price: number): string {
    if (price === 0) return '0';

    if (price >= 1) {
        return price.toFixed(2);
    } else if (price >= 0.01) {
        return price.toFixed(4);
    } else if (price >= 0.0001) {
        return price.toFixed(6);
    } else if (price >= 0.000001) {
        return price.toFixed(8);
    } else if (price >= 0.00000001) {
        return price.toFixed(10);
    } else if (price >= 0.0000000001) {
        return price.toFixed(15);
    } else {
        return price.toFixed(20);
    }
}

function formatLargePrice(price: number): string {
    return formatSmallPrice(price);
}

function formatNumber(num: number): string {
    if (num >= 1000) {
        return num.toFixed(2);
    } else if (num >= 1) {
        return num.toFixed(2);
    } else if (num > 0) {
        return num.toFixed(20); 
    } else {
        return '0';
    }
}


function getTokenSymbol(mint: string): string {
    const knownTokens: {[key: string]: string} = {
        'So11111111111111111111111111111111111111112': 'SOL',
        'EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v': 'USDC',
        'Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB': 'USDT',
        'DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263': 'BONK',
        'AZsHEMXd36Bj1EMNXhowJajpUXzrKcK57wW4ZGXVa7yR': 'GUAC',
    };
    
    return knownTokens[mint] || mint.slice(0, 4) + '...' + mint.slice(-4);
}
