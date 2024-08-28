export async function getDexScreener(token) {
   const url = 'https://api.dexscreener.com/latest/dex/search/?q=' + token;
   try{
   const response = await fetch(url);
     const data = await response.json();
     if(data.pairs[0].dexId=='raydium'){
      return data.pairs[0]
     }else{
      return undefined
     }
    }catch(error){
    }
}
