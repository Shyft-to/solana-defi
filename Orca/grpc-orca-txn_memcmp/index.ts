import Client, {
    CommitmentLevel,
    SubscribeRequestAccountsDataSlice,
    SubscribeRequestFilterAccounts,
    SubscribeRequestFilterBlocks,
    SubscribeRequestFilterBlocksMeta,
    SubscribeRequestFilterEntry,
    SubscribeRequestFilterSlots,
    SubscribeRequestFilterTransactions,
  } from "@triton-one/yellowstone-grpc";
  import { SubscribeRequestPing } from "@triton-one/yellowstone-grpc/dist/grpc/geyser";
  import { VersionedTransactionResponse } from "@solana/web3.js";
import { tOutPut } from "./utils/transactionOutput";
import { getTokenInfo } from "./utils/tokenInfo";
//import { sqrtPriceX64ToPrice } from "@orca-so/whirlpool-sdk";
import { sqrtPriceX64ToPrice } from "./utils/calculatePrice";
 
  interface SubscribeRequest {
    accounts: { [key: string]: SubscribeRequestFilterAccounts };
    slots: { [key: string]: SubscribeRequestFilterSlots };
    transactions: { [key: string]: SubscribeRequestFilterTransactions };
    transactionsStatus: { [key: string]: SubscribeRequestFilterTransactions };
    blocks: { [key: string]: SubscribeRequestFilterBlocks };
    blocksMeta: { [key: string]: SubscribeRequestFilterBlocksMeta };
    entry: { [key: string]: SubscribeRequestFilterEntry };
    commitment?: CommitmentLevel | undefined;
    accountsDataSlice: SubscribeRequestAccountsDataSlice[];
    ping?: SubscribeRequestPing | undefined;
  } 
   
   const ORCA = 'whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc';

    async function handleStream(client: Client, args: SubscribeRequest) {
    // Subscribe for events
    const stream = await client.subscribe();
  
    // Create `error` / `end` handler
    const streamClosed = new Promise<void>((resolve, reject) => {
      stream.on("error", (error) => {
        console.log("ERROR", error);
        reject(error);
        stream.end();
      });
      stream.on("end", () => {
        resolve();
      });
      stream.on("close", () => {
        resolve();
      });
    });
  
    // Handle updates
    stream.on("data", async (data) => {
      try{
      const result = await tOutPut(data);
      const signature = result.signature;
      const pubKey = result.pubKey;
      const mintA = result.poolstate.tokenMintA;
      const mintB = result.poolstate.tokenMintB;
      const sqrtPrice = result.poolstate.sqrtPrice;
      const decimalA = await getTokenInfo(mintA);
      const decimalB = await getTokenInfo(mintB);
      const price = sqrtPriceX64ToPrice(sqrtPrice,Number(decimalA),Number(decimalB));
      const mintBPrice = 219/Number(price) 
   //   const priceChecker = sqrtPriceX64ToPrice
      console.log(`
        signature : ${signature}
        Public Key : ${pubKey}
        Mint A : ${mintA}
        Mint B : ${mintB}
        SqrtPrice : ${sqrtPrice}
        Price : ${price}
        Mint B Price : ${mintBPrice}
        `)
  }catch(error){
    if(error){
      console.log(error)
    }
  }
});
  
    // Send subscribe request
    await new Promise<void>((resolve, reject) => {
      stream.write(args, (err: any) => {
        if (err === null || err === undefined) {
          resolve();
        } else {
          reject(err);
        }
      });
    }).catch((reason) => {
      console.error(reason);
      throw reason;
    });
  
    await streamClosed;
  }
  
  async function subscribeCommand(client: Client, args: SubscribeRequest) {
    while (true) {
      try {
        await handleStream(client, args);
      } catch (error) {
        console.error("Stream error, restarting in 1 second...", error);
        await new Promise((resolve) => setTimeout(resolve, 1000));
      }
    }
  }

  const client = new Client(
    'gRPC REGION URL',
    'gRPC TOKEN',
    undefined,
  );
const req: SubscribeRequest = {
  "slots": {},
  "accounts": {
    "ORCA": {
      "account": [],
      "filters": [],
      "owner": [ORCA] // raydium program id to subscribe to
    }
  },
  "transactions": {},
  "blocks": {},
  "blocksMeta": {
    "block": []
  },
  "accountsDataSlice": [],
  "commitment": CommitmentLevel.PROCESSED, // Subscribe to processed blocks for the fastest updates
  entry: {},
  transactionsStatus: {}
}
  subscribeCommand(client, req);
  
