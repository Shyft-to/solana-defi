const api = process.env.API;
 var myHeaders = new Headers();
 myHeaders.append("x-api-key", api);
 const sol = 'So11111111111111111111111111111111111111112';
 var requestOptions:any = {
   method: 'GET',
   headers: myHeaders,
   redirect: 'follow'
 };
 
   export async function getTokenInfo(address){
    try{
     const url = `https://defi.shyft.to/v0/pools/get_by_pair?tokenA=${address}&tokenB=${sol}`;

     const response = await fetch(url,requestOptions);
      const data = await response.json();
      const dexes = data?.result?.dexes;
      const pumpfunAmm = dexes?.pumpFunAmm?.pools[0];
      const pubKey = pumpfunAmm?.pubkey;
      const baseVault = pumpfunAmm?.pool_base_token_account;
      const quoteVault = pumpfunAmm?.pool_quote_token_account;
      const lp = pumpfunAmm?.lp_mint;
      const supply = pumpfunAmm?.lp_supply;
      const lamports = pumpfunAmm?.lamports;
      const owner = pumpfunAmm?.creator;
       return {
         pubKey,
         supply,
         baseVault,
         quoteVault,
         lamports,
         lp,
         owner
       }
    }catch(error){
      console.log(error);
    }
  }
 