var myHeaders = new Headers();
export const api = "api"
myHeaders.append("x-api-key", api);

var requestOptions:any = {
  method: 'GET',
  headers: myHeaders,
  redirect: 'follow'
};
export async function getTokenBalance(address){
 try{
 const info = await fetch(`https://api.shyft.to/sol/v1/wallet/all_tokens?network=mainnet-beta&wallet=${address}`, requestOptions)
 const infoJson = await info.json();
 const result = infoJson?.result[0];
 const ca = result?.address
 const name = result?.info?.name
 const symbol = result?.info.symbol
const balance = result?.balance;
 return {
  name,
  symbol,
  ca,
  balance
 };
}catch(err){
  console.log('Error');
}
}
