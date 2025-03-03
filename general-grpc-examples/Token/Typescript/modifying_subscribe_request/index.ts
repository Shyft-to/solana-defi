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
import axios from "axios";

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
let subscribedWalletsA: string[] = [
  "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
  "5n2WeFEQbfV65niEP63sZc3VA7EgC4gxcTzsGGuXpump",
  "4oJh9x5Cr14bfaBtUsXN1YUZbxRhuae9nrkSyWGSpump",
  "GBpE12CEBFY9C74gRBuZMTPgy2BGEJNCn4cHbEPKpump",
  "oraim8c9d1nkfuQk9EzGYEUGxqL3MHQYndRw1huVo5h",
];
let subscribedWalletsB: string[] = [
    "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"
]
// Predefined subscription requests
const subscribeRequest1: SubscribeRequest = {
  accounts: {},
  slots: {},
  transactions: {
    modifying_A: {
      vote: false,
      failed: false,
      signature: undefined,
      accountInclude: subscribedWalletsA,
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
  commitment: CommitmentLevel.PROCESSED,
};
const subscribeRequest2: SubscribeRequest = {
  "slots": {},
  "accounts": {
    "modifying_B": {
      "account": [],
      "filters": [],
      "owner": subscribedWalletsB // raydium program id to subscribe to
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

async function updateSubscription(stream: any, args: SubscribeRequest) {
  try {
    // Send the updated request to the stream
    stream.write(args);
  } catch (error) {
    console.error("Failed to send new request:", error);
  }
}
async function handleStream(client: Client, args: SubscribeRequest) {
  const stream = await client.subscribe();

  const streamClosed = new Promise<void>((resolve, reject) => {
    stream.on("error", (error) => {
      console.log("ERROR", error);
      reject(error);
      stream.end();
    });
    stream.on("end", resolve);
    stream.on("close", resolve);
  });
   // Switch to the second request after 2 seconds without closing the stream

  setTimeout(async () => {
    console.log("Switched to second subscription request");
    await updateSubscription(stream, subscribeRequest2);  // Update the subscription with the second request
    }, 10000);  // Change request after 2 seconds



  stream.on("data", async (data) => {
    try {
      console.log(data);
    } catch (error) {
      console.log(error);
    }
  });
  await new Promise<void>((resolve, reject) => {
    stream.write(args, (err: any) => (err ? reject(err) : resolve()));
  }).catch((reason) => {
    console.error(reason);
    throw reason;
  });
  await streamClosed;
}

async function subscribeCommand(client: Client, args: SubscribeRequest) {
  const stream = await client.subscribe();  // Make sure stream is available here for updateSubscription

  // Periodically update the subscription, now outside handleStream

  while (true) {
    try {
      await handleStream(client, args); // Start streaming
    } catch (error) {
      console.error("Stream error, restarting in 1 second...", error);
      await new Promise((resolve) => setTimeout(resolve, 1000));
    }
  }
}

const client = new Client(
  process.env.GRPC_URL,
  process.env.X_TOKEN,
  undefined,
);


subscribeCommand(client, subscribeRequest1);
