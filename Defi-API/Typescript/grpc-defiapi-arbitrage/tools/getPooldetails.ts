const api =  ''
 var myHeaders = new Headers();
 myHeaders.append("x-api-key", api);
 
 var requestOptions:any = {
   method: 'GET',
   headers: myHeaders,
   redirect: 'follow'
 };
 export async function getTokenInfo(addressA, addressB) {
  const url = `https://defi.shyft.to/v0/pools/get_by_pair?tokenA=${addressA}&tokenB=${addressB}`;
  
  const response = await fetch(url, requestOptions);
  const data = await response.json();
  
  const dexes = data?.result?.dexes;
  
  const fluxbeam = dexes?.fluxbeam?.pools;
  const orca = dexes?.orca?.pools;
  const meteoraAmm = dexes?.meteoraAmm?.pools;
  const raydium = dexes?.raydiumAmm?.pools;
  
  // Function to get the pool with the highest liquidity
  const getHighestLiquidityPool = (pools) => {
    if (!pools || pools.length === 0) return null;
    return pools.reduce((maxPool, currentPool) => {
      return currentPool.liquidity > maxPool.liquidity ? currentPool : maxPool;
    });
  };

  const highestFluxbeam = getHighestLiquidityPool(fluxbeam);
  const highestOrca = getHighestLiquidityPool(orca);
  const highestMeteoraAmm = getHighestLiquidityPool(meteoraAmm);
  const highestRaydium = getHighestLiquidityPool(raydium);

 

  return {
    fluxbeam: highestFluxbeam,
    orca: highestOrca,
    meteoraAmm: highestMeteoraAmm,
    raydium: highestRaydium,
  };
}

//getTokenInfo('5LafQUrVco6o7KMz42eqVEJ9LW31StPyGjeeu5sKoMtA','So11111111111111111111111111111111111111112')