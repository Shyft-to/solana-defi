import { Connection, PublicKey } from "@solana/web3.js";


export const api = ""
const connection = new Connection(`https://rpc.shyft.to?api_key=${api}`, 'confirmed');

 export async function getSolBalance(address){
    if (!address) {
        throw new Error('Vault address is undefined');
    }
     const account = await connection.getBalance(new PublicKey(address));
     const balance = account/1000000000;
  
    return balance;
 }  
 export async function getTokenBalance(address){
    const account = await connection.getTokenAccountBalance(new PublicKey(address))
    const balance = Number(account.value.amount);
    return balance;
 }
 
