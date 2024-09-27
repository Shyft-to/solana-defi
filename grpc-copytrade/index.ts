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
import { Liquidity, LIQUIDITY_STATE_LAYOUT_V4 } from "@raydium-io/raydium-sdk";
import { getSolBalance, getTokenBalance } from "utils/walletInfo";
import { getTokenInfo } from "utils/tokenInfo";
import { getMarketInfo } from "utils/marketInfo";
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
     
    // Handle updates
    stream.on("data", async (data) => {
      try{
    const result = await tOutPut(data);
     const baseVault = result.poolstate.baseVault.toString();
     const quoteVault = result.poolstate.quoteVault.toString();
     const mint = result.poolstate.baseMint.toString();
    console.log(result)
    const tokenInfo = await getTokenInfo(mint)
    const quoteBal = await getSolBalance(quoteVault);
    const baseBal = await getTokenBalance(baseVault)/ 10 ** tokenInfo.decimal;
     const marketInfo = await getMarketInfo(baseBal,quoteBal,tokenInfo.currentSupply)
     const quoteBal$ = marketInfo.quote$
     const price = marketInfo.price;
     const marketcap = marketInfo.marketcap;
     const supply = marketInfo.currentSupply;
     if(supply === undefined && tokenInfo.decimal === undefined){
     }else{
     console.log(`
        CA : ${mint}
        Supply : ${supply}
        BaseVault : ${baseVault}
        QuoteVault : ${quoteVault}
        Decimal : ${tokenInfo.decimal}
        Swap in : ${result.poolstate.swapBaseInAmount}
        Swap Out : ${result.poolstate.swapQuoteOutAmount}
        Price : $${price}
        MarketCap : $${marketcap}
        PoolInfo : ${quoteBal}($${quoteBal$})
                   ${baseBal}
      `)
    }
  }catch(error){
    if(error){
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
  slots: {},
  accounts: {
    usdc: {
      account: [],
      owner: ["JUPyiwrYJFskUPiHa7hkeR8VUtAeFoSYbKedZNsDvCN"],
      filters: [
        {
          memcmp: {
            offset: String(0),
            base58: "",
          },
        },
      ],
    },
  },
  transactions: {},
  transactionsStatus: {},
  blocks: {},
  blocksMeta: {},
  entry: {},
  accountsDataSlice: [],
  commitment: CommitmentLevel.CONFIRMED,
};
  subscribeCommand(client, req);
  
