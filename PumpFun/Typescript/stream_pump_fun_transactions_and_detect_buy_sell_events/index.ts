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
import { SubscribeRequestPing } from "@triton-one/yellowstone-grpc/dist/types/grpc/geyser";
import { TransactionFormatter } from "./utils/transaction-formatter";
import { parseSwapTransactionOutput } from "./utils/pumpfun_formatted_txn";
import { PUMP_FUN_PROGRAM_ID } from "./utils/type";
import { PumpFunDecoder } from "./utils/decode-parser";

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
const pumpFunDecoder = new PumpFunDecoder();
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


async function handleStream(client: Client, args: SubscribeRequest) {
  console.log("Streaming Buy Sell on Pumpfun...")
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
      const txn = TXN_FORMATTER.formTransactionFromJson(
        data.transaction,
        Date.now(),
      );
      const parsedTxn = pumpFunDecoder.decodePumpFunTxn(txn);

      const parsedPumpfunTxn = parseSwapTransactionOutput(parsedTxn)
       console.log(
        new Date(),
        ":",
        `New transaction https://translator.shyft.to/tx/${txn.transaction.signatures[0]} \n`,
        JSON.stringify(parsedPumpfunTxn, null, 2) + "\n"
      );
      console.log(
        "--------------------------------------------------------------------------------------------------"
      );
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
  process.env.GRPC_URL,
  process.env.X_TOKEN,
  undefined,
);
const req: SubscribeRequest = {
  accounts: {},
  slots: {},
  transactions: {
    pumpFun: {
      vote: false,
      failed: false,
      signature: undefined,
      accountInclude: [PUMP_FUN_PROGRAM_ID.toBase58()],
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

subscribeCommand(client, req);