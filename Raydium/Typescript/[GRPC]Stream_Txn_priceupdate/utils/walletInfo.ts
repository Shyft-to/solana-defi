import { Connection, PublicKey } from "@solana/web3.js";


export const api = "your api"
const connection = new Connection(`https://rpc.shyft.to?api_key=${api}`, 'confirmed');

export async function getTokenBalance(address){
    const account = await connection.getTokenAccountBalance(new PublicKey(address))
    const balance = Number(account.value.amount);
    return balance;
 }
 export async function getSolBalance(address){
     const account = await connection.getBalance(new PublicKey(address));
     const balance = account/1000000000;
    return balance;
 }  
