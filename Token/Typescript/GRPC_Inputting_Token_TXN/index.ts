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

let subscribedWallets: string[] = [
  "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
  "5n2WeFEQbfV65niEP63sZc3VA7EgC4gxcTzsGGuXpump",
  "4oJh9x5Cr14bfaBtUsXN1YUZbxRhuae9nrkSyWGSpump",
  "GBpE12CEBFY9C74gRBuZMTPgy2BGEJNCn4cHbEPKpump",
  "oraim8c9d1nkfuQk9EzGYEUGxqL3MHQYndRw1huVo5h",
];

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

  stream.on("data", async (data) => {
    try {
      const result = await tOutPut(data);
      if (result.signature == "") return;
      console.log(result);
      console.log("Subsribed Wallet: " + subscribedWallets)
    } catch (error) {
      console.log(error);
    }
  });
  async function updateSubscription() {
    try {
      const request = await fetchWallets();
  
      if (request.length === 0) {
        return;
      }
      const uniqueWallets = Array.from(new Set(request));
      subscribedWallets = uniqueWallets;
      // Update the subscription request
      args.transactions.migration.accountInclude = subscribedWallets;
  
      // Send the new request
      stream.write(args);
    } catch (error) {
      console.error("Failed to send new request:", error);
    }
  }

  
  // Periodically fetch and update wallets every 5 seconds
  setInterval(updateSubscription, 5000);

  await new Promise<void>((resolve, reject) => {
    stream.write(args, (err: any) => (err ? reject(err) : resolve()));
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
  "gRPC-URL",
  "gRPC-TOKEN",
  undefined
);

const req: SubscribeRequest = {
  accounts: {},
  slots: {},
  transactions: {
    migration: {
      vote: false,
      failed: false,
      signature: undefined,
      accountInclude: subscribedWallets, 
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

async function fetchWallets() {
  try {
    const response = await axios.get("http://localhost:3000/wallets");
    const data: string[] = response.data;

    const uniqueWallets = Array.from(new Set(data));
    return uniqueWallets;
  } catch (error) {
    return []; 
  }
}

subscribeCommand(client, req);