const api =  'api'
 var myHeaders = new Headers();
 myHeaders.append("x-api-key", api);
 const sol = 'So11111111111111111111111111111111111111112';
 var requestOptions:any = {
   method: 'GET',
   headers: myHeaders,
   redirect: 'follow'
 };
 
   export async function getTokenInfo(address){
     const url = `https://defi.shyft.to/v0/pools/get_by_pair?tokenA=${address}&tokenB=${sol}`;

     const response = await fetch(url,requestOptions);
      const data = await response.json();
      const dexes = data?.result?.dexes;
      const raydium = dexes?.raydiumAmm?.pools[0];
      const pubKey = raydium?.pubkey;
      const baseVault = raydium?.baseVault;
      const quoteVault = raydium?.quoteVault;
      const lp = raydium?.lpMint;
      const owner = raydium?.owner;
      return {
        pubKey,
        baseVault,
        quoteVault,
        lp,
        owner
      }
  }
 