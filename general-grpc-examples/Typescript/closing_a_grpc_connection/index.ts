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
// import { bs58 } from "@project-serum/anchor/dist/cjs/utils/bytes";
import bs58 from 'bs58';

const PUBLIC_KEY_TO_LISTEN = "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"; //can be any address
const RUN_TIME = 5000; //decides how long to run
const ENDPOINT = "https://grpc.ams.shyft.to";
const XTOKEN = ""; //enter your x-token

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
}

const client = new Client(
  ENDPOINT,
  XTOKEN,
  undefined
);

const req: SubscribeRequest = {
  accounts: {},
  slots: {},
  transactions: {
    raydiumLiquidityPoolV4: {
      vote: false,
      failed: false,
      signature: undefined,
      accountInclude: [PUBLIC_KEY_TO_LISTEN],
      accountExclude: [],
      accountRequired: [],
    },
  },
  transactionsStatus: {},
  entry: {},
  blocks: {},
  blocksMeta: {},
  accountsDataSlice: [],
  commitment: CommitmentLevel.PROCESSED,
};

async function handleStream(client: Client, args: SubscribeRequest) {
  console.log(`Subscribing and starting stream...`);
  const stream = await client.subscribe();
  console.log(`Streaming data and printing transactions...`);

  const streamClosed = new Promise<void>((resolve, reject) => {
    stream.on("error", (err: any) => {
      if (err.code === 1 || err.message.includes("Cancelled")) {
        //this indicates stream was cancelled by user. 
        console.log("✅ Stream cancelled by user");
        resolve();
      } else {
        console.error("❌ Stream error:", err);
        reject(err);
      }
    });

    stream.on("close", () => {
      console.log("Stream closed.");
      resolve();
    });
  });
  
  stream.on("data", (data) => {
    if(data.transaction) {
      console.log("Received: ", bs58.encode(data.transaction.transaction.signature));
    }
  });
  
  // Subscribe
  stream.write(args);
  
  // Cancel after timeout
  setTimeout(() => {
    console.log("Cancelling stream...");
    try {
      stream.cancel();
      /*
      * stream.end() or stream.destroy();
      * Cancels the stream from the user end, and closes the stream.
      * 
      * A "Cancelled on client" error (code 1) will be thrown once this is called,
      * which be caught in stream.on("error") function.
      * This is one of the ways to cancel the stream, and clear your connections, when using Shyft gRPCs.
      * 
      * For more information on clearing connections, please check the FAQ section of gRPC docs
      * https://docs.shyft.to/solana-yellowstone-grpc/grpc-docs#solana-grpc-faq
      */
    } catch (error) {
      if (error.code !== "ERR_STREAM_PREMATURE_CLOSE") {
        console.error("Stream cancel error:", error);
      }
    }
  }, RUN_TIME);
  
  await streamClosed;
}

async function subscribeCommand(client: Client, args: SubscribeRequest) {
  await handleStream(client, args);
}

subscribeCommand(client, req);
