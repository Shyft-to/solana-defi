import { TOKEN_PROGRAM_ID } from "@raydium-io/raydium-sdk";
import { Connection, GetProgramAccountsFilter, PublicKey } from "@solana/web3.js";

const api = 'YMyDOr87OBzT6TWr'
// 'pCdmc-vpYN9w2etM'
const connection = new Connection(`https://rpc.shyft.to?api_key=${api}`, 'confirmed');
const ADDRESS = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');

const POOL_DATA_SIZE = 24;
const OFFSET_FOR_TOKEN_A = 0;

// DEX program addresses
const dexes = [
    { name: 'Raydium', address: '675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8' },
    { name: 'Orca', address: 'whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc' },
    { name: 'Meteora', address: 'LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo' }
];
const raydium = new PublicKey('675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8');
// Token you want to fetch pools for
const tokenA = new PublicKey('EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v');

async function getRaydiumTransactions() {
  const tokenSigns = await connection.getSignaturesForAddress(tokenA);
  let txnAccounts = [];
//console.log(tokenSigns.length)
 
    const tokenTxn = await connection.getParsedTransaction('23F76GSWNSadi76Erj48yJqyEWa5TvMZgopTuCpnEuSaX22kMYSYpGwecYBYsuSwUAQPP2PAfNyeyP6tzJeB38Az', {
      maxSupportedTransactionVersion: 0,
    });
   // console.log(tokenTxn);
    
    // Check if tokenTxn is valid and contains instructions
    if (tokenTxn && tokenTxn.transaction.message.instructions) {
      const accounts = tokenTxn.transaction.message.instructions.filter(ix => 
        ix.programId.toBase58() === raydium.toBase58()
      );
      console.log(accounts)
      console.log(tokenTxn.transaction.message.instructions)

      // If accounts exist, push them into txnAccounts
      if (accounts.length > 0) {
        txnAccounts.push(...accounts); // Spread operator to add all matched accounts
        return;
      }
    }
    console.log(txnAccounts)
  }


  async function main(){
    const tokenAccount = await connection.getParsedProgramAccounts(
      TOKEN_PROGRAM_ID,
      { filters }
    );
    tokenAccount.forEach((accounts, i)=>{
      const parsedAccount = accounts.account.data
      const address = parsedAccount
      console.log(parsedAccount)
    })
  }
  
  


getRaydiumTransactions()