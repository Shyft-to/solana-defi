import { id } from 'ethers';
import {Telegraf, TelegramError} from 'telegraf'
import { Connection, Keypair, PublicKey } from '@solana/web3.js';
import { gql, GraphQLClient } from "graphql-request";
import * as anchor from "@project-serum/anchor";
import {Buy,Sell} from "./swapAmm"
import {
  Metadata,
  PROGRAM_ID as TOKEN_METADATA_PROGRAM_ID,
} from "@metaplex-foundation/mpl-token-metadata";

import { Wallet } from '@project-serum/anchor';
import { quote } from 'telegraf/typings/format';
//const Extra = require('telegraf/extra')
const Markup = require('telegraf/markup')
const { getOrCreateAssociatedTokenAccount } = require('@solana/spl-token');

const privateKey = new Uint8Array([
 //add your private key
]);
const wallet = new Wallet(Keypair.fromSecretKey(privateKey));
const api =" " //add your api;

const SESSION_HASH = 'QNDEMO' + Math.ceil(Math.random() * 1e9); // Random unique identifier for your session

const endpoint = `https://programs.shyft.to/v0/graphql/?api_key=${api}`;
// const connection = new Connection(`https://solana-mainnet.core.chainstack.com/500a50e949070da73951d8d4f493532b`, {   
//     wsEndpoint: `wss://solana-mainnet.core.chainstack.com/ws/500a50e949070da73951d8d4f493532b`,
//     httpHeaders: {"x-session-hash": SESSION_HASH}
// });
const connection = new Connection(`https://rpc.shyft.to?api_key= your api key`, 'confirmed');

//`https://solana-mainnet.core.chainstack.com/968251fbfe51d98ea3cee0bf693ba515`
//wss://solana-mainnet.core.chainstack.com/ws/968251fbfe51d98ea3cee0bf693ba515`

 const graphQLClient = new GraphQLClient(endpoint, {
   method: `POST`,
   jsonSerializer: {
     parse: JSON.parse,
    stringify: JSON.stringify,
   },
 });


const TELEGRAM_BOT_TOKEN = "Your Token"
const bot = new Telegraf(TELEGRAM_BOT_TOKEN)
const msgId = 'input your chat id';
var myHeaders = new Headers();


let token;
let pair;
let decimal;
let confirm;
const amount = 1000000 // input any amount of your choice



var trades : Array<String> = []


//bot.use(Telegraf.log())

bot.command('enterTrade', async (ctx) => await ctx.reply('Hi, Ready to enter Trade, Please give the command ', {
  reply_markup : {
    inline_keyboard : [
       [
         {
          text : "setUp",
          callback_data : "SetUp"
         },
         {
          text : "Clear",
          callback_data: "Delete"
         },
         {
          text : "Fetch",
          callback_data: "get"
         }
       ]    
    ]
  }
}));
bot.command('sell', async(ctx) => {
  console.log(`Token ${token}, Pair ${pair}, Decimal ${decimal}`)
   orderSell(token,pair,Number(decimal));
  // if(successful == true){
  //   ctx.reply("Sell Successful");
  // }
})

bot.action(/.+/,async (ctx) => {
  if(ctx.match[0] == "SetUp"){
    ctx.reply("Enter token contract address");
  }else if(ctx.match[0] == "Delete"){
    ctx.reply("Deleting All Entry")
    trades = []
  }else if(ctx.match[0] == "get"){
    if(trades.length == 0){
      ctx.reply("Entry is Empty");
    }else{
    ctx.reply("Fetching all Locked in Tokens and Details");  
    ctx.reply(`${trades.join(", ")}`)
    }
  }
})

bot.on('text', async(ctx) => {
  const Response = ctx.message?.text!
  if(trades.length >= 3){
   ctx.reply("Entry Filled");
  }else{ 
    console.log(Response)
    ctx.reply(`Locking in Token Address : ${Response}`)
    trades.push(Response);
   // ctx.deleteMessage();
    if(trades.length>=1){
       let tokenInfo = await info(Response);
       let tInfo;
       if(tokenInfo == null){
          tInfo = true;
       }
       if(
          tInfo == true
        ){               
           ctx.reply("Token is yet to Launch");
           if(Response == trades[0]){
            ctx.reply(`${trades[0]} Without Lp
           Saving Contract till LP
           `)
            console.log("trade : " + trades[0])
           let interval = setInterval(async() => { 
              tokenInfo = await info(trades[0])
              console.log(`Searching for ${trades[0]}`)
               if(tokenInfo != null){
                  console.log(`Search over`);
                 tInfo = false;
                 clearInterval(interval)
                 console.log(`${tokenInfo.tokenName} Launched!!`)
                 ctx.reply(
                  `  🪙 ${tokenInfo.tokenName} Token Info
          
                  Name:  ${tokenInfo.tokenName}
          
                  Symbol:  ${tokenInfo.tokenSym}
          
                  Pair Token : SOLANA
          
                  Pair: ${tokenInfo.lp}
          
                  Token: ${tokenInfo.token}
          
                  Vault: ${tokenInfo.vault}
          
                  Sol Bal: ${Number(tokenInfo.vaultBalance).toFixed(2)} Sol
          
                  Decimal: ${tokenInfo.decimal}
          
                  OpenTime: ${tokenInfo.startTime}
          
                  
          
                  👨🏻‍💻 Owner Info
          
                  Address: ${tokenInfo.owner}
          
                  Balance: ${Number(tokenInfo.ownerBalance).toFixed(2)} SOL
                  `
                 )  
                 token = tokenInfo.token
                 pair = tokenInfo.lp;
                 decimal = tokenInfo.decimal;              
                const now : any = new Date();
                const targetTime : any = new Date(tokenInfo.startTime.getFullYear(), tokenInfo.startTime.getMonth(),tokenInfo.startTime.getDate(), tokenInfo.startTime.getHours(), tokenInfo.startTime.getMinutes(), tokenInfo.startTime.getSeconds(),tokenInfo.startTime.getMilliseconds());
                const timeDiff : any = targetTime - now;
                const msUntilTarget = timeDiff > 0 ? timeDiff : 86400000 - Math.abs(timeDiff);
                await orderBuys(tokenInfo.token.toString(),tokenInfo.lp.toString(),Number(tokenInfo.decimal))
                 const MAX_RETRIES = 10000;
                 const BASE_DELAY = 1000;
                 let retries = 0;
                 let delay = BASE_DELAY;
                 const Addr = new PublicKey(tokenInfo.vault)
                 let profit : Number;
                 const subscriptionID = connection.onAccountChange(
                 Addr,
                 async(updatedAccountInfo, context) => {
                   const Bal= updatedAccountInfo.lamports/1000000000
                   const prof = Number(Bal) / Number(tokenInfo?.vaultBalance);
                   console.log(prof);
                while (retries < MAX_RETRIES) {
                  try {     
                  await ctx.reply(
                  `
                   ${tokenInfo.tokenSym} : ${tokenInfo.token}
          
                   SOL : So11111111111111111111111111111111111111112
          
                   ${Number(tokenInfo.vaultBalance).toFixed(2)} SOL
                   Profit : ${Number(prof).toFixed(2)}x
                  `
                  )
                 return;
                } catch (error) {
                  if (error.response && error.response.error_code === 429) {
                    console.log(`Profit on console.. TG in ${delay}ms...`);
                    await new Promise(resolve => setTimeout(resolve, delay));
                    retries++;
                    delay *= 1.5; // double the delay for each retry
                    if (delay > 60000) { // limit the maximum delay to 60 seconds
                      delay = 60000;
                    }
                  } else {
                    throw error;
                  }
                }
              }
               throw new Error('Max retries exceeded');
                }
                 )   
                
              }
              }, 10000)
            }  
       }else{ 
       ctx.reply(
        `  🪙 ${tokenInfo.tokenName} Token Info

        Name:  ${tokenInfo.tokenName}

        Symbol:  ${tokenInfo.tokenSym}

        Pair Token : SOLANA

        Pair: ${tokenInfo.lp}

        Token: ${tokenInfo.token}

        Vault: ${tokenInfo.vault}

        Sol Bal: ${Number(tokenInfo.vaultBalance).toFixed(2)} Sol

        Decimal: ${tokenInfo.decimal}

        OpenTime: ${tokenInfo.startTime}

        

        👨🏻‍💻 Owner Info

        Address: ${tokenInfo.owner}

        Balance: ${Number(tokenInfo.ownerBalance).toFixed(2)} SOL
        `
       )
      //  if(tokenInfo.successful == true){
      //   ctx.reply(`Buy Successful`);
      //  }
      token = tokenInfo.token
      pair = tokenInfo.lp;
      decimal = tokenInfo.decimal;

      const now : any = new Date();
      const targetTime : any = new Date(tokenInfo.startTime.getFullYear(), tokenInfo.startTime.getMonth(),tokenInfo.startTime.getDate(), tokenInfo.startTime.getHours(), tokenInfo.startTime.getMinutes(), tokenInfo.startTime.getSeconds(),tokenInfo.startTime.getMilliseconds());
      const timeDiff : any = targetTime - now;
      const msUntilTarget = timeDiff > 0 ? timeDiff : 86400000 - Math.abs(timeDiff);
      if(now >= targetTime){
      await orderBuys(tokenInfo.token.toString(),tokenInfo.lp.toString(),Number(tokenInfo.decimal))
      }
       const MAX_RETRIES = 10000;
       const BASE_DELAY = 1000;
       let retries = 0;
       let delay = BASE_DELAY;
       const Addr = new PublicKey(tokenInfo.vault)
       let profit : Number;
       const subscriptionID = connection.onAccountChange(
       Addr,
       async(updatedAccountInfo, context) => {
         const Bal= updatedAccountInfo.lamports/1000000000
         const prof = Number(Bal) / Number(tokenInfo?.vaultBalance);
         console.log(prof);
      while (retries < MAX_RETRIES) {
        try {     
        await ctx.reply(
        `
         ${tokenInfo.tokenSym} : ${tokenInfo.token}

         SOL : So11111111111111111111111111111111111111112

         ${Number(tokenInfo.vaultBalance).toFixed(2)} SOL
         Profit : ${Number(prof).toFixed(2)}x
        `
        )
       return;
      } catch (error) {
        if (error.response && error.response.error_code === 429) {
          console.log(`Profit on console.. TG in ${delay}ms...`);
          await new Promise(resolve => setTimeout(resolve, delay));
          retries++;
          delay *= 1.5; // double the delay for each retry
          if (delay > 60000) { // limit the maximum delay to 60 seconds
            delay = 60000;
          }
        } else {
          throw error;
        }
      }
    }
     throw new Error('Max retries exceeded');
      }
       )   
     
    }
  }
    } 
});

bot.launch()


async function info(pair) {
  const SOLANA : string = 'So11111111111111111111111111111111111111112';
  let vault;
  let check;
  //  for(const trade of array){
  //   pair = trade;
  //  }
   if(pair == undefined || pair == null){
    console.log(`it is null`)
    }else{
    const mint = new anchor.web3.PublicKey(
     pair
    );

  const [metadataPDA] =  anchor.web3.PublicKey.findProgramAddressSync(
    [
      Buffer.from("metadata"),
      TOKEN_METADATA_PROGRAM_ID.toBuffer(),
      mint.toBuffer(),
    ],
    TOKEN_METADATA_PROGRAM_ID
  );
  const accInfo = await connection.getAccountInfo(metadataPDA);
  const metadata = Metadata.deserialize(accInfo.data, 0);
  const tokenName = metadata[0].data.name;
  const tokenSym = metadata[0].data.symbol; 
  const data:any = await queryLpMintInfo(pair,SOLANA);
  
   const info = data.Raydium_LiquidityPoolv4[0];
   if(info == null){
    return null
   }else{
   let ownerBalance;
   const openTime = info.poolOpenTime;
   const startTime = new Date(openTime * 1000); 
   const token : String = info.baseMint
   const tokenVault : String = info.baseVault
   const solVault: String = info.quoteVault;
   
   const lp = info.pubkey
   const decimal = info.baseDecimal;
  // const qvault = info.quoteVault;
   const owner = metadata[0].updateAuthority;
   const lpReserve = info.lpReserve;
   const ownerInfo = await connection.getAccountInfo(new PublicKey(owner))
   if(ownerInfo == null){
    ownerBalance = "Renounced"
   }else{
   ownerBalance = await ownerInfo.lamports/1000000000;
   }
   
   const tokenAddress = new PublicKey(solVault)
   
    vault = solVault
    check =  await connection.getTokenAccountBalance(
    tokenAddress
  )
   const vaultBalance = await check.value.uiAmount;
  //  const successful =  await orderBuys(msUntilTarget,token.toString(),lp.toString(),Number(decimal))
   return {
    tokenName,
    tokenSym,
     lp,
     vaultBalance,
     startTime,
     token,
     decimal,
     vault,
     owner,
   //  successful,
     ownerBalance
   }
  }
   }
  }


async function queryLpMintInfo(token: string, sol: string) {
  // See how we are only querying what we need
  const query = gql`
    query MyQuery ($where: Raydium_LiquidityPoolv4_bool_exp) {
  Raydium_LiquidityPoolv4(
    where: $where
  ) {
    baseMint
    lpMint
    lpReserve
    baseVault
    poolOpenTime
    lpVault
    quoteMint
    quoteVault
    baseDecimal
    owner
    pubkey
  }
}`;

const variables = {
  where: {
    baseMint: {
      _eq: token,
    },
    quoteMint: {
      _eq: sol,
    },
  },
};

  return await graphQLClient.request(query, variables);
}


async function orderBuys(token,pool, decimal) {
 await Buy(token,pool,amount,decimal);
};

 
 async function orderSell(token,pool,decimal) {
 const tokenAdd = new PublicKey(token);

 const tokenAddress = await getOrCreateAssociatedTokenAccount(connection,wallet, tokenAdd  ,wallet.publicKey )// RAY
 const balance = Number(tokenAddress.amount);
 await Sell(token,pool,balance,decimal);
 return true;
 }
