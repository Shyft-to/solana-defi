 const api = "your api"
var myHeaders = new Headers();
myHeaders.append("x-api-key", api);

var requestOptions:any = {
  method: 'GET',
  headers: myHeaders,
  redirect: 'follow'
};

  export async function getTokenInfo(address){
    const info = await fetch(`https://api.shyft.to/sol/v1/token/get_info?network=mainnet-beta&token_address=${address}`, requestOptions)
    const infoJson = await info.json();
    const result = infoJson.result;
    const currentSupply = result?.current_supply;
    const decimal = result?.decimals;
    return {currentSupply,decimal} 
   }
