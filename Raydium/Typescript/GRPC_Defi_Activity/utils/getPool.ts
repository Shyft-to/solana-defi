 const api = ""
var myHeaders = new Headers();
myHeaders.append("x-api-key", api);
type Dex = {
  pools: any[];
  programId: string;
};

var requestOptions:any = {
  method: 'GET',
  headers: myHeaders,
  redirect: 'follow'
};

  export async function getPools(address){
    const info = await fetch(`https://defi.shyft.to/v0/pools/get_by_token?token=${address}&page=1&limit=2`, requestOptions)
    const infoJson = await info.json();
    const dexes : Record<string, Dex>  = infoJson?.result.dexes;
    const nonEmptyDexes = Object.entries(dexes)
    .filter(([_, value]) => value.pools.length > 0) // Filter dexes with non-empty pools
     .map(([key, value]) => ({ name: key, ...value }))
     const pools = nonEmptyDexes.flatMap(pool => pool.pools);
     const pubkeys = pools.flatMap(pubkey => pubkey.pubkey);
     return pubkeys;
   }
