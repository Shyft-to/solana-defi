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
import { LIQUIDITY_STATE_LAYOUT_V4 } from "@raydium-io/raydium-sdk";
import { getTokenInfo } from "./tools/getPooldetails";
import { getSolBalance } from "./tools/getBalance";

const sol = new PublicKey(
  "So11111111111111111111111111111111111111112"
);
const token = new PublicKey(
"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
); // token we are interested in
const raydium_PROGRAM_ID = new PublicKey(
  "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
);//listen to swaps

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
     const poolInfo = await getTokenInfo(token.toString(),sol.toString());
     const raydium = poolInfo.raydium;
     const fluxbeam = poolInfo?.fluxbeam;
     if (!raydium) return;
    if (!fluxbeam) return;
     const fluxbeamVault = fluxbeam?.tokenAccountA;
     const raydiumVault = raydium?.quoteMint === sol.toString()? raydium?.quoteVault:raydium?.baseVault;
     const fVaultBal =await getSolBalance(fluxbeamVault);
     const rVaultBal = await getSolBalance(raydiumVault)
     const arbitrageCal = arbCalculation(fVaultBal,rVaultBal);
     if(fluxbeamVault !== undefined || rVaultBal !== undefined){
     console.log(`
        ARBITRAGE OPPORTUNITY FOUND
        DEX A : RAYDIUM ${rVaultBal}
        DEX B : FLUXBEAM ${fVaultBal}
        LIQUIDITY DIFF : ${arbitrageCal}
      `)
     }
   }catch(error){
   console.log(error)
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
    "raydium": {
      "account": [],
      "filters": [
        {
          "memcmp": {
            "offset": LIQUIDITY_STATE_LAYOUT_V4.offsetOf('quoteMint').toString(), // Filter for only tokens paired with SOL
            "base58": "So11111111111111111111111111111111111111112"
          }
        },
        {
          "memcmp": {
            "offset" : LIQUIDITY_STATE_LAYOUT_V4.offsetOf('baseMint').toString(),
            "base58" : token.toBase58()
          }
        },
        {
          "memcmp": {
            "offset": LIQUIDITY_STATE_LAYOUT_V4.offsetOf('marketProgramId').toString(), // Filter for only Raydium markets that contain references to Serum
            "base58": "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX"
          }
        }
      ],
      "owner": [raydium_PROGRAM_ID.toString()] // raydium program id to subscribe to
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
function arbCalculation(amountA, amountB){
  if(amountA > amountB){
    const result = amountA - amountB
    return result;
  }else{
    return amountB - amountA
  }
}

subscribeCommand(client, req);

