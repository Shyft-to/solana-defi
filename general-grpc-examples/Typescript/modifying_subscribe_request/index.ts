require('dotenv').config()
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

interface SubscribeRequest {
  accounts: { [key: string]: SubscribeRequestFilterAccounts };
  slots: { [key: string]: SubscribeRequestFilterSlots };
  transactions: { [key: string]: SubscribeRequestFilterTransactions };
  transactionsStatus: { [key: string]: SubscribeRequestFilterTransactions };
  blocks: { [key: string]: SubscribeRequestFilterBlocks };
  blocksMeta: { [key: string]: SubscribeRequestFilterBlocksMeta };
  entry: { [key: string]: SubscribeRequestFilterEntry };
  commitment?: CommitmentLevel;
  accountsDataSlice: SubscribeRequestAccountsDataSlice[];
  ping?: SubscribeRequestPing;
}

const subscribedWalletsA: string[] = [
  "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
  "5n2WeFEQbfV65niEP63sZc3VA7EgC4gxcTzsGGuXpump",
  "4oJh9x5Cr14bfaBtUsXN1YUZbxRhuae9nrkSyWGSpump",
  "GBpE12CEBFY9C74gRBuZMTPgy2BGEJNCn4cHbEPKpump",
  "oraim8c9d1nkfuQk9EzGYEUGxqL3MHQYndRw1huVo5h",
];

const subscribedWalletsB: string[] = [
  "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
];

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

// Subscribes to account changes for program-owned accounts of subscribedWalletsB
const subscribeRequest2: SubscribeRequest = {
  accounts: {
    modifying_B: {
      account: [],
      filters: [],
      owner: subscribedWalletsB,
    },
  },
  slots: {},
  transactions: {},
  transactionsStatus: {},
  blocks: {},
  blocksMeta: {
    block: [],
  },
  entry: {},
  accountsDataSlice: [],
  ping: undefined,
  commitment: CommitmentLevel.PROCESSED,
};

/**
 * Dynamically updates the current stream subscription with new request parameters.
 */
async function updateSubscription(stream: any, args: SubscribeRequest) {
  try {
    stream.write(args);
  } catch (error) {
    console.error("Failed to send updated subscription request:", error);
  }
}

/**
 * Handles a single streaming session.
 * Automatically switches to a second subscription request after a timeout.
 */
async function handleStream(client: Client, args: SubscribeRequest) {
  const stream = await client.subscribe();

  // Waits for the stream to close or error out
  const streamClosed = new Promise<void>((resolve, reject) => {
    stream.on("error", (error) => {
      console.error("Stream Error:", error);
      reject(error);
      stream.end();
    });
    stream.on("end", resolve);
    stream.on("close", resolve);
  });

  // Automatically switch subscription after 10 seconds
  setTimeout(async () => {
    console.log("🔁 Switching to second subscription request...");
    await updateSubscription(stream, subscribeRequest2);
  }, 10000);

  // Handle incoming data
  stream.on("data", async (data) => {
    try {
      console.log("📦 Streamed Data:", data);
      // You can add more processing logic here
    } catch (error) {
      console.error("Error processing stream data:", error);
    }
  });

  // Send initial subscription request
  await new Promise<void>((resolve, reject) => {
    stream.write(args, (err: any) => (err ? reject(err) : resolve()));
  }).catch((reason) => {
    console.error("Initial stream write failed:", reason);
    throw reason;
  });

  await streamClosed;
}

/**
 * Starts the stream and continuously attempts to reconnect on errors.
 */
async function subscribeCommand(client: Client, args: SubscribeRequest) {
  while (true) {
    try {
      await handleStream(client, args);
    } catch (error) {
      console.error("Stream error. Retrying in 1 second...", error);
      await new Promise((resolve) => setTimeout(resolve, 1000));
    }
  }
}

const client = new Client(process.env.GRPC_URL, process.env.X_TOKEN, undefined);

// Start streaming with the first subscription
subscribeCommand(client, subscribeRequest1);
