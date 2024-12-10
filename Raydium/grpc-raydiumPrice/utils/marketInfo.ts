import { getTokenInfo } from "./tokenInfo";
import { getSolBalance, getTokenBalance } from "./walletInfo";

export async function getMarketInfo(baseBal, quoteBal, currentSupply) {
    try {
      const quote$ = quoteBal * 134.4//quoteBal * sol value in $$;
      const price = quote$ / baseBal;
      const marketcap = currentSupply * price;
  
      return {
        price,
        marketcap,
        currentSupply,
        quote$,
      };
    } catch (error) {
      console.error('Error getting market info:', error);
      throw error;
    }
  }