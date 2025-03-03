const api =  ''

 
 export async function getTokenInfo(address) {
  try{
    var myHeaders = new Headers();
    myHeaders.append("x-api-key", api);
    
    var requestOptions:any = {
      method: 'GET',
      headers: myHeaders,
      redirect: 'follow'
    };
  const url = `https://defi.shyft.to/v0/pools/get_by_token?token=${address}`;
  const response = await fetch(url, requestOptions);
  const data = await response.json();
  
  const dexes = data?.result?.dexes;
  const fluxbeam = dexes?.fluxbeam?.pools[0];
  const orca = dexes?.orca?.pools[0];
  const meteoraAmm = dexes?.meteoraAmm?.pools[0];
  const raydium = dexes?.raydiumAmm?.pools[0];
  // Fluxbeam variables
  const fluxBeamTokenAccountA = fluxbeam?.tokenAccountA;
  const fluxBeamTokenAccountB = fluxbeam?.TokenAccountB;
  const fluxBeamMintA = fluxbeam?.mintA;
  const fluxBeamMintB = fluxbeam?.mintB;
  const fluxBeamLamports = fluxbeam?.lamports;
  const fluxBeamLpMint = fluxbeam?.tokenPool;
  const fluxBeamPublicKey = fluxbeam?.pubkey;
  const fluxBeamVaultLiquidity = fluxbeam?.liquidity;

  // Orca variables
  const orcaTokenAccountA = orca?.tokenVaultA;
  const orcaTokenAccountB = orca?.tokenVaultB;
  const orcaMintA = orca?.tokenMintA;
  const orcaMintB = orca?.tokenMintB;
  const orcaLiquidity = orca?.liquidity;
  const orcaLamports = orca?.lamports;
  const orcaPublicKey = orca?.pubkey;

  // Meteora AMM variables
  const meteoraTokenAccountA = meteoraAmm?.aVault;
  const meteoraTokenAccountB = meteoraAmm?.bVault;
  const meteoraMintA = meteoraAmm?.tokenAMint;
  const meteoraMintB = meteoraAmm?.tokenBMint;

  const meteoraLamports = meteoraAmm?._lamports;

  const meteoraPublicKey = meteoraAmm?.pubkey;
  const meteoraVaultLpA = meteoraAmm?.aVaultLp;
  const meteoraVaultLpB = meteoraAmm?.bVaultLp;

  // Raydium variables
  const raydiumTokenAccountA = raydium?.baseVault;
  const raydiumTokenAccountB = raydium?.quoteVault;
  const raydiumMintA = raydium?.quoteMint;
  const raydiumMintB = raydium?.baseMint;
  const raydiumLamports = raydium?.lamports;
  const raydiumLpMint = raydium?.lpMint;
  const raydiumPublicKey = raydium?.pubkey;
  const reserve = raydium?.lpReserve


  return {
      fluxBeamTokenAccountA,
      fluxBeamTokenAccountB,
      fluxBeamMintA,
      fluxBeamMintB,
      fluxBeamLamports,
      fluxBeamLpMint,
      fluxBeamPublicKey,

 

      orcaTokenAccountA,
      orcaTokenAccountB,
      orcaMintA,
      orcaMintB,
      orcaLiquidity,
      orcaLamports,
    
      orcaPublicKey,

   

      meteoraTokenAccountA,
      meteoraTokenAccountB,
      meteoraMintA,
      meteoraMintB,
     
      meteoraLamports,
     
      meteoraPublicKey,
      meteoraVaultLpA,
      meteoraVaultLpB,

      raydiumTokenAccountA,
      raydiumTokenAccountB,
      raydiumMintA,
      raydiumMintB,
      raydiumLamports,
      raydiumLpMint,
      raydiumPublicKey,
       reserve
  };
}catch{
  console.log("error")
}
}
