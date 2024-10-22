const api =  'api'
 var myHeaders = new Headers();
 myHeaders.append("x-api-key", api);
 
 var requestOptions:any = {
   method: 'GET',
   headers: myHeaders,
   redirect: 'follow'
 };
 
   export async function getTokenInfo(addressA,addressB){
     const url = `https://defi.shyft.to/v0/pools/get_by_pair?tokenA=${addressA}&tokenB=${addressB}`;

     const response = await fetch(url,requestOptions);
      const data = await response.json();
      const dexes = data?.result?.dexes;
      const fluxbeam = dexes?.fluxbeam?.pools[0];
      const orca = dexes?.orca?.pools[0];
      const meteoraAmm = dexes?.meteoraAmm?.pools[0];
      const raydium = dexes?.raydiumAmm?.pools[0];
      return {
        fluxbeam,
        orca,
        meteoraAmm,
        raydium
      }
  }
