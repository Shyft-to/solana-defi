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
  import { PublicKey, VersionedTransactionResponse } from "@solana/web3.js";
import { tOutPut } from "./utils/transactionOutput";
import { getDexScreener } from "./utils/dexScreener";

const raydium_PROGRAM_ID = new PublicKey(
  "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
);


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
     const mint = result?.meta?.postTokenBalances[0]?.mint;
     const mintDetails = await getDexScreener(mint);
     const priceUsd =  mintDetails?.priceUsd;
     const priceChangeM5 = mintDetails?.priceChange.m5;
     const priceChangeH1 = mintDetails?.priceChange.h1;
     const priceChangeH24 = mintDetails?.priceChange.h24;
     console.log(`
        CA : ${mint}
        Price USD : $${priceUsd}
        Price Change 5mins : ${priceChangeM5}
        Price Change 1Hr : ${priceChangeH1}
        Price Change 24Hr : ${priceChangeH24}
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
  const req = {
    accounts: {},
    slots: {},
    transactions: {
      bondingCurve: {
        vote: false,
        failed: false,
        signature: undefined,
        accountInclude: [raydium_PROGRAM_ID.toString()], //Address 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
        accountExclude: [],
        accountRequired: [],
      },
    },
    transactionsStatus: {},
    entry: {},
    blocks: {},
    blocksMeta: {},
    accountsDataSlice: [],
    ping: undefined,
    commitment: CommitmentLevel.CONFIRMED, //for receiving confirmed txn updates
  };
  subscribeCommand(client, req);
  
