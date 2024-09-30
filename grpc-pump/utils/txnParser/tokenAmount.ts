import { Connection } from "@solana/web3.js";

export const api = "api"
const connection = new Connection(`https://rpc.shyft.to?api_key=${api}`, 'confirmed');

export async function TXN(signature){
    const transaction:any = await connection.getParsedTransaction(signature,{
        maxSupportedTransactionVersion: 0
    })
    //transaction.meta.innerInstructions[1].instructions[0].parsed?.info
    const innerInstructions = transaction?.meta.innerInstructions[0];
    const tokenAmount = innerInstructions?.instructions[0]?.parsed?.info.amount;
    const solAmount = innerInstructions?.instructions[1]?.parsed?.info?.lamports !== undefined?
    innerInstructions?.instructions[1]?.parsed?.info?.lamports: 
    transaction?.meta?.innerInstructions[1]?.instructions[0]?.parsed?.info?.lamports;
    const signer = innerInstructions?.instructions[0]?.parsed?.info?.source;
    const token = transaction?.meta?.preTokenBalances[0]?.mint;
    return {
      tokenAmount,
      solAmount,
      signer,
      token
    }
 }