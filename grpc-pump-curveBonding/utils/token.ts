var myHeaders = new Headers();
export const api = "your api"
myHeaders.append("x-api-key", api);

var requestOptions:any = {
  method: 'GET',
  headers: myHeaders,
  redirect: 'follow'
};
export async function getTokenBalance(address){

 const info = await fetch(`https://api.shyft.to/sol/v1/wallet/all_tokens?network=mainnet-beta&wallet=${address}`, requestOptions)
 const infoJson = await info.json();
 const result = infoJson?.result[0];
 const balance = result?.balance;
 return balance;
  
}
