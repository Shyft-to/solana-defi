import { getTokenInfo } from "./tokenInfo";
import { getSolBalance, getTokenBalance } from "./walletInfo";

export async function getMarketInfo(baseBal, quoteBal, currentSupply) {
    try {
      const quote$ = quoteBal//quoteBal * sol value in $$ use any api to fetch sol price;
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
