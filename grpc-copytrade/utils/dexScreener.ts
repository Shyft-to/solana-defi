export async function getDexScreener(token) {
   const url = 'https://api.dexscreener.com/latest/dex/search/?q=' + token;
   try{
   const response = await fetch(url);
     const data = await response.json();
     if(data.pairs[0].dexId=='raydium'){
        const info = data.pairs[0];
        const pair = info.pairAddress;
        const name = info.baseToken.name;
        const symbol = info.baseToken.symbol
        const marketcap = info.fdv;
        const price = info.priceUsd
      return {
        name,
        symbol,
        price,
        pair,
        marketcap
      }
     }else{
      return undefined
     }
    }catch(error){
    }
}
