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
     const tokenDetails = result?.meta?.postTokenBalances;
     const mint = tokenDetails[0]?.mint
     const tokenInfo = await getDexScreener(mint);
     const signature = result.signature;
     const amount = tokenDetails[0].uiTokenAmount.uiAmount;
     const time = new Date();
     const priceBought = tokenInfo.price * amount
     console.log(`
        Swapped Time :: ${time.getHours()}:${time.getMinutes()}
        CA : ${mint}
        Name : ${tokenInfo.name}
        Symbol : ${tokenInfo.symbol}
        Price : ${tokenInfo.price}
        Pair : ${tokenInfo.pair}
        MarketCap : ${tokenInfo.marketcap}
        Amount Swapped : ${amount} ${tokenInfo.symbol}
        Amount Value in Usd : $${priceBought} 
        tx : https://solscan.io/tx/${signature}
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
      copyTrade: {
        vote: false,
        failed: false,
        signature: undefined,
        accountInclude: ['Wallet Address'], //Wallet address you would love to monitor
        accountExclude: [],
        accountRequired: [raydium_PROGRAM_ID.toString()],
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
  

  // Swapped Time :: 15:6
  // CA : 82qt3HNBkvqpo46FYcBcHXNnUf3tMq6GNBkoARt3pump
  // Name : So Much Higher
  // Symbol : SMH
  // Price : 0.001132
  // Pair : 7oguisXbogr7o3o713dpe1PH2uwixtLBi4zabmAPvGsW
  // MarketCap : 1132770
  // Amount Swapped : 42932.733913 SMH
  // Amount Value in Usd : $48.599854789515994
  // tx : https://solscan.io/tx/4SBAtV7CUkvsdKUfL5ywNN1yLbbp7rhRRc7543wna5NnZ5Px8EVhAqaZLx6urQLDdTyXaM6mQtzykMyfDx3ZMHxs