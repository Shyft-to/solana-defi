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
import { Connection, PublicKey, VersionedTransactionResponse } from "@solana/web3.js";
import { tOutPut } from "./utils/transactionOutput";
import { publicKey } from "@solana/buffer-layout-utils";
import { getTokenBalance } from "./utils/token";
import { LIQUIDITY_STATE_LAYOUT_V4 } from "@raydium-io/raydium-sdk";
import { Boolean } from "@solana/buffer-layout";
import { structure } from "./utils/decodeTransaction";
import { getTokenInfo } from "./utils/defiApi";
const pumpfun = '6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P';


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
    if(!result) return;
    const tokenInfo = await getTokenBalance(result.pubKey);
    const poolInfo = await getTokenInfo(tokenInfo?.ca);
    if(poolInfo.lp === undefined) return;
    console.log(
      `
      PUMPFUN -- RAYDIUM
      CA : ${tokenInfo?.ca}
      Name : ${tokenInfo?.name} (${tokenInfo?.symbol})
      POOL DETAILS : Base Vault ${poolInfo?.baseVault}
                     Quote Vault ${poolInfo?.quoteVault}
                     Public Key ${poolInfo?.pubKey}
                     LP Mint ${poolInfo?.lp}
      BONDING CURVE STATUS : COMPLETED                    
      `
   )
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
  "pumpfun": {
    "account": [],
    "filters": [
      {
        "memcmp": {
          "offset": structure.offsetOf('complete').toString(), // Hack to filter for swapped. There is probably a better way to do this
          "bytes" : Uint8Array.from([1])
        }
      }
    ],
    "owner": [pumpfun] // raydium program id to subscribe to
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
