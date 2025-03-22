import "dotenv/config";
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
import { PublicKey, VersionedTransactionResponse } from "@solana/web3.js";
import { Idl } from "@coral-xyz/anchor";
import { SolanaParser } from "@shyft-to/solana-transaction-parser";
import { SubscribeRequestPing } from "@triton-one/yellowstone-grpc/dist/types/grpc/geyser";
import { TransactionFormatter } from "./utils/transaction-formatter";
import { SolanaEventParser } from "./utils/event-parser";
import { bnLayoutFormatter } from "./utils/bn-layout-formatter";
import pumpFunAmmIdl from "./idls/pump_amm_0.1.0.json";
import { writeFileSync } from "fs";

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
const PUMP_FUN_AMM_PROGRAM_ID = new PublicKey(
  "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA"
);
const PUMP_FUN_IX_PARSER = new SolanaParser([]);
PUMP_FUN_IX_PARSER.addParserFromIdl(
  PUMP_FUN_AMM_PROGRAM_ID.toBase58(),
  pumpFunAmmIdl as Idl
);
const PUMP_FUN_EVENT_PARSER = new SolanaEventParser([], console);
PUMP_FUN_EVENT_PARSER.addParserFromIdl(
  PUMP_FUN_AMM_PROGRAM_ID.toBase58(),
  pumpFunAmmIdl as Idl
);

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
      const txn = TXN_FORMATTER.formTransactionFromJson(
        data.transaction,
        Date.now()
      );

      const parsedTxn = decodePumpFunTxn(txn);

      if (!parsedTxn) return;

      console.log(
        new Date(),
        ":",
        `New transaction https://translator.shyft.to/tx/${txn.transaction.signatures[0]} \n`,
        JSON.stringify(parsedTxn, null, 2) + "\n"
      );
      console.log(
        "--------------------------------------------------------------------------------------------------"
      );
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
  process.env.ENDPOINT!,
  process.env.X_TOKEN,
  undefined
);

const req: SubscribeRequest = {
  accounts: {},
  slots: {},
  transactions: {
    pumpFun: {
      vote: false,
      failed: false,
      signature: undefined,
      accountInclude: [PUMP_FUN_AMM_PROGRAM_ID.toBase58()],
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

function decodePumpFunTxn(tx: VersionedTransactionResponse) {
  if (tx.meta?.err) return;

  const paredIxs = PUMP_FUN_IX_PARSER.parseTransactionData(
    tx.transaction.message,
    tx.meta.loadedAddresses
  );

  const pumpFunIxs = paredIxs.filter((ix) =>
    ix.programId.equals(PUMP_FUN_AMM_PROGRAM_ID)
  );

  if (pumpFunIxs.length === 0) return;
  const events = PUMP_FUN_EVENT_PARSER.parseEvent(tx);
  const result = { instructions: pumpFunIxs, events };
  bnLayoutFormatter(result);
  return result;
}
