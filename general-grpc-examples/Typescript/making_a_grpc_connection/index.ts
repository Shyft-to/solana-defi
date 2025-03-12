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
import { TransactionFormatter } from "./utils/transaction-formatter";


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

const ADDRESS_TO_STREAM_FROM = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
const TXN_FORMATTER = new TransactionFormatter();

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
  stream.on("data", (data) => {
    if (data?.transaction) {
      //when subscribing to transactions, raw transactions are streamed from gRPC. 
      // The following function formats the received transaction in a format which is commonly used and returned via any Solana RPC.
      const txn = TXN_FORMATTER.formTransactionFromJson(
        data.transaction,
        Date.now(),
      );

      console.log("The Received Transaction: ")
      console.dir(txn, { depth: null });

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
  await handleStream(client, args);
  // while (true) {
  //   try {
  //     await handleStream(client, args);
  //   } catch (error) {
  //     console.error("Stream error, restarting in 1 second...", error);
  //     await new Promise((resolve) => setTimeout(resolve, 1000));
  //   }
  // }
}

const client = new Client(
  process.env.GRPC_URL,
  process.env.X_TOKEN,
  undefined,
);

/*
  The SubscribeRequest interface is defined in the `@triton-one/yellowstone-grpc` package. It has several fields such as `accounts`, `slots`, `transactions`, `transactionsStatus`, `blocks`, `blocksMeta`, `entry`, `commitment`, `accountsDataSlice`, and `ping`. 
  The accounts field is used for streaming account updates, while the slots field is used for streaming slot updates. Further specifications related to the stream is defined in the subfields. 
  The transactions field is used for streaming full transaction along with its metadata, while the transactionsStatus field is used for streaming status of a transactions(signature only). The blocks field can stream transactions specified entirely in a block.

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
  }, //for this example we have demonstrated streaming transactions, so we have added the address in the accountInclude param of the transaction field in the subscribe request.
  transactionsStatus: {},
  entry: {},
  blocks: {},
  blocksMeta: {},
  accountsDataSlice: [],
  ping: undefined,
  commitment: CommitmentLevel.CONFIRMED,
};

subscribeCommand(client, req);

