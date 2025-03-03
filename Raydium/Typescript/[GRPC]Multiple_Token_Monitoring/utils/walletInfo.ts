import { Connection, PublicKey } from "@solana/web3.js";


export const api = "api"
const connection = new Connection(`https://rpc.shyft.to?api_key=${api}`, 'confirmed');

export async function getTokenBalance(address){
    try{
    const account = await connection.getTokenAccountBalance(new PublicKey(address))
    const balance = Number(account?.value?.amount);
    if(!balance)return;
    return balance;
    }catch(err){
    }
 }
 export async function getSolBalance(address){
     const account = await connection.getBalance(new PublicKey(address));
     const balance = account/1000000000;
    return balance;
 }  
