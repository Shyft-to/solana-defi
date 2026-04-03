import "dotenv/config";
import Client, {
  CommitmentLevel,
  SubscribeRequest,
} from "@triton-one/yellowstone-grpc";
import { TransactionFormatter } from "./utils/transaction-formatter";
import { METEORA_DLMM_PROGRAM_ID } from "./utils/type";
import { MeteoraDlmmDecoder } from "./utils/decode-parser";
import { meteoraDlmmParsedTransaction } from "./utils/meteora-dlmm-transaction-parser";

const originalConsoleWarn = console.warn;
const originalConsoleLog = console.log;
const originalConsoleError = console.error;

console.warn = (message?: any, ...optionalParams: any[]) => {
  if (
    typeof message === "string" &&
    message.includes("Parser does not matching the instruction args")
  ) {
    return;
  }
  originalConsoleWarn(message, ...optionalParams); 
};

console.log = (message?: any, ...optionalParams: any[]) => {
  if (
    typeof message === "string" &&
    message.includes("Parser does not matching the instruction args")
  ) {
    return; 
  }
  originalConsoleLog(message, ...optionalParams); 
};

console.error = (message?: any, ...optionalParams: any[]) => {
  if (
    typeof message === "string" &&
    message.includes("Parser does not matching the instruction args")
  ) {
    return; 
  }
  originalConsoleError(message, ...optionalParams); 
};

const TXN_FORMATTER = new TransactionFormatter();
const METEORADLMM_PARSER = new MeteoraDlmmDecoder();

async function handleStream(client: Client, args: SubscribeRequest) {
  console.log("Streaming Meteora Dlmm Txn");
  const stream = await client.subscribe();

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
      
      try {
        const txn = TXN_FORMATTER.formTransactionFromJson(
          data.transaction,
          Date.now()
        );

        const decodedParsedTxn = METEORADLMM_PARSER.decodeTxn(txn);
        const parsedTxn = meteoraDlmmParsedTransaction(decodedParsedTxn, txn);
        if(!parsedTxn) return;
        
        console.log(
        new Date(),
         ":",
         `New transaction https://translator.shyft.to/tx/${txn.transaction.signatures[0]} \n`,
         JSON.stringify(parsedTxn, null, 2) + "\n"
       );
       console.log(
         "--------------------------------------------------------------------------------------------------"
       );
      } catch (error) {
        console.error("Error processing transaction:", error);
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
      await new Promise((resolve) => setTimeout(resolve, 1000));
    }
  }
}


const client = new Client(
  process.env.GRPC_URL,
  process.env.X_TOKEN,
  undefined,
);

const req: SubscribeRequest = {
  accounts: {},
  slots: {},
  transactions: {
    meteoraDlmm: {
      vote: false,
      failed: false,
      signature: undefined,
      accountInclude: [METEORA_DLMM_PROGRAM_ID.toBase58()],
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
  commitment: CommitmentLevel.CONFIRMED,
};
subscribeCommand(client, req).catch(console.error);