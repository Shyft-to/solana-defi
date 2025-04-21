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

// Interface for the subscription request structure
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

const ADDRESS_TO_STREAM_FROM = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

/**
 * Subscribes to the gRPC stream and handles incoming data.
 * 
 * @param client - Yellowstone gRPC client
 * @param args - The Subscription request which specifies what data to stream
 */
async function handleStream(client: Client, args: SubscribeRequest) {
  const stream = await client.subscribe();

  // Promise that resolves when the stream ends or errors out
  const streamClosed = new Promise<void>((resolve, reject) => {
    stream.on("error", (error) => {
      console.error("Stream error:", error);
      reject(error);
      stream.end();
    });

    stream.on("end", resolve);
    stream.on("close", resolve);
  });

  // Handle incoming transaction data
  stream.on("data", (data) => {
    if (data?.transaction) {
      
      console.log("Received Transaction:");
      console.log(data?.transaction);
    }
  });

  // Send the subscription request
  await new Promise<void>((resolve, reject) => {
    stream.write(args, (err: any) => {
      err ? reject(err) : resolve();
    });
  }).catch((err) => {
    console.error("Failed to send subscription request:", err);
    throw err;
  });

  // Wait for the stream to close
  await streamClosed;
}

/**
 * Entry point to start the subscription stream.
 * 
 */
async function subscribeCommand(client: Client, args: SubscribeRequest) {
  await handleStream(client, args);

}

// Instantiate Yellowstone gRPC client with env credentials
const client = new Client(
  process.env.GRPC_URL, //Your Region specific gRPC URL
  process.env.X_TOKEN, // your Access Token
  undefined
);

/**
 * Subscribe Request: The `transactions` field filters transaction streams to only include those
 * that involve the specified address in `accountInclude`.
 */
const req: SubscribeRequest = {
  accounts: {},
  slots: {},
  transactions: {
    pumpFun: {
      vote: false,
      failed: false,
      signature: undefined,
      accountInclude: [ADDRESS_TO_STREAM_FROM],
      accountExclude: [],
      accountRequired: [],
    },
  },
  transactionsStatus: {},
  blocks: {},
  blocksMeta: {},
  entry: {},
  accountsDataSlice: [],
  ping: undefined,
  commitment: CommitmentLevel.CONFIRMED,
};

// Start the subscription
subscribeCommand(client, req);