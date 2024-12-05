import { Connection, PublicKey } from "@solana/web3.js";
export const api = "" 

const sentTransactions = new Set();

const shyft = `https://rpc.ny.shyft.to?api_key=${api}`;
const pumpfun = '6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P';
const connection = new Connection(shyft,'confirmed')
export async function getBondingCurveAddress(transaction : any[]){
    let bondingCurve;
    let solBalance;
    const eachOwners = transaction?.flatMap(inner => inner.owner);
    for (const owner in eachOwners){
      const address = new PublicKey(eachOwners[owner]);
      const systemOwner = await connection.getAccountInfo(address);
      if(systemOwner.owner.toString() === pumpfun)
        bondingCurve = address;
        solBalance = systemOwner.lamports;
        return {bondingCurve,solBalance}
    }
    return {bondingCurve,solBalance}
  }