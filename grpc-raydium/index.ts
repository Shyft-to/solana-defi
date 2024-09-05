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
import { LIQUIDITY_STATE_LAYOUT_V4 } from "@raydium-io/raydium-sdk";

//YMyDOr87OBzT6TWr 
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
      if (data.account != undefined) {
       const info = await tOutPut(data);
       console.log(
        new Date(),
        ":",
        `New transaction https://translator.shyft.to/tx/${info.signature} \n`,
        JSON.stringify(info, null, 2) + "\n",
      );
    }
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
const request: SubscribeRequest = {
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
            "offset": LIQUIDITY_STATE_LAYOUT_V4.offsetOf('marketProgramId').toString(), // Filter for only Raydium markets that contain references to Serum
            "base58": "srmqPvymJeFKQ4zGQed1GFppgkRHL9kaELCbyksJtPX"
          }
        },
        {
          "memcmp": {
            "offset": LIQUIDITY_STATE_LAYOUT_V4.offsetOf('swapQuoteInAmount').toString(), // Hack to filter for only new tokens. There is probably a better way to do this
            "bytes": Uint8Array.from([0])
          }
         },
        {
          "memcmp": {
            "offset": LIQUIDITY_STATE_LAYOUT_V4.offsetOf('swapBaseOutAmount').toString(), // Hack to filter for only new tokens. There is probably a better way to do this
            "bytes": Uint8Array.from([0])
          }
        }
      ],
      "owner": ["675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"] // raydium program id to subscribe to
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

subscribeCommand(client, request);