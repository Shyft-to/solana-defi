var myHeaders = new Headers();
const api = 'Your api'

myHeaders.append("x-api-key", api);

var requestOptions:any = {
  method: 'GET',
  headers: myHeaders,
  redirect: 'follow'
};
export async function getTokenInfo(token){
 const info = await fetch(`https://api.shyft.to/sol/v1/token/get_info?network=mainnet-beta&token_address=${token}`, requestOptions)
 const infoJson = await info.json();
 const name = infoJson.result.name;
 const symbol = infoJson.result.symbol;
 const desc = infoJson.result.description;
 const decimal = infoJson.result.decimals;
 const supply = infoJson.result.current_supply;
  return{
    name,
    symbol,
    desc,
    supply,
    decimal
  }
}

