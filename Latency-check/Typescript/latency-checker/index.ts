import "dotenv/config";

import * as fastq from 'fastq';
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
import { VersionedTransactionResponse, Connection } from "@solana/web3.js";

import { TransactionFormatter } from "./utils/transaction-formatter";
import { ReportGenerator } from "./utils/report-generator";

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

const TXN_FORMATTER = new TransactionFormatter();
const PUBLIC_KEY_TO_LISTEN = process.env.PUBLIC_KEY_TO_LISTEN ?? "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
const RUN_TIME = Number(process.env.RUN_TIME ?? 2) * 60 * 1000; 
const REPORT_GEN = new ReportGenerator();


let commitment = process.env.COMMITMENT_LEVEL ?? "confirmed";
const commitmentLevelToUse = commitment === "confirmed" ? CommitmentLevel.CONFIRMED : commitment === "processed" ? CommitmentLevel.PROCESSED : CommitmentLevel.FINALIZED;

const solanaConnection = new Connection(process.env.RPC_URL ?? "https://api.mainnet-beta.solana.com");

const client = new Client(
  process.env.ENDPOINT!,
  process.env.X_TOKEN,
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
  ping: undefined,
  commitment: commitmentLevelToUse,
};

let accumLatency = 0;
let count = 0;

const worker: fastq.worker<VersionedTransactionResponse> = async (transaction: VersionedTransactionResponse) => {
  // console.log("executing worker");
  if (!transaction?.blockTime) {
    console.log(
      "No blocktime found for: ",
      transaction?.transaction?.signatures[0]
    );
    return;
  }
  if (commitment === "processed")
    await new Promise((resolve) => setTimeout(resolve, 2000));

  const transactionData = await solanaConnection.getTransaction(
    transaction.transaction.signatures[0],
    {
      maxSupportedTransactionVersion: 0,
      commitment: (commitment === "confirmed" || commitment === "processed") ? "confirmed" : "finalized",
    }
  );
  if (!transactionData?.blockTime) {
    console.log(
      "No RPC blocktime found for: ",
      transaction?.transaction?.signatures[0]
    );
    return;
  }
  const transactionTime = transactionData?.blockTime as number * 1000;
  const receivedTime = transaction?.blockTime;
  REPORT_GEN.collectData(transactionTime, receivedTime);
  
    const difference = receivedTime - transactionTime;
    const actualDifference = difference > 0 ? difference : 0;
    //console.log("Current Latency: ", actualDifference);
  
    accumLatency += actualDifference;
    count++;
    console.log(
      "\nReceived: ",
      transaction.transaction.signatures[0],
      "\nReceived blocktime: ",
      receivedTime,
      " RPC Blocktime: ",
      transactionTime,
      " Latency: ",
      actualDifference,
      " ms"
    );
    console.log("Average Latency: ", accumLatency / count);
  
}

//@ts-ignore
const q = fastq.promise(worker, 100); 

async function handleStream(client: Client, args: SubscribeRequest) {
  console.log(`Subscribing and starting stream...`);
  const stream = await client.subscribe();
  console.log(`Streaming data and collecting info...`);
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

  stream.on("data", (data) => {
    if (data?.transaction) {
      const txn = TXN_FORMATTER.formTransactionFromJson(
        data.transaction,
        Date.now()
      );
      try {
        //console.log("Received: ",txn.transaction?.signatures[0]);
        q.push(txn);
        return;
      } catch (error) {
        console.error("parsing error: ", error, txn.transaction.signatures[0]);
      }
    }
  });

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

  setTimeout(() => {
    console.log("Ending stream now...");
    // stream.write(unsubRequest, (err: any) => {
    //   console.log(err);
    // });
    stream.end();
    stream.destroy();
    REPORT_GEN.generateReport();
    process.exit(0);
    
  }, RUN_TIME);

  await streamClosed;
}
  
  async function subscribeCommand(client: Client, args: SubscribeRequest) {
    await handleStream(client, args);
    // while (true) {
    //   try {

    //   } catch (error) {
    //     console.error("Stream error, restarting in 1 second...", error);
    //     await new Promise((resolve) => setTimeout(resolve, 1000));
    //   }
    // }
  }

  subscribeCommand(client, req);