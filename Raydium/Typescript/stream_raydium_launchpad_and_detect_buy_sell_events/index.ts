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
import raydiumLaunchpadIdl from "./idls/raydium_launchpad.json";
import { writeFileSync } from "fs";
import { parsedTransactionOutput } from "./utils/rl-transaction-formatter";

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
const RAYDIUM_LAUNCHPAD_PROGRAM_ID = new PublicKey(
  "LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj"
);
const RAYDIUM_LAUNCHPAD_IX_PARSER = new SolanaParser([]);
RAYDIUM_LAUNCHPAD_IX_PARSER.addParserFromIdl(
  RAYDIUM_LAUNCHPAD_PROGRAM_ID.toBase58(),
  raydiumLaunchpadIdl as Idl
);
const RAYDIUM_LAUNCHPAD_EVENT_PARSER = new SolanaEventParser([], console);
RAYDIUM_LAUNCHPAD_EVENT_PARSER.addParserFromIdl(
  RAYDIUM_LAUNCHPAD_PROGRAM_ID.toBase58(),
  raydiumLaunchpadIdl as Idl
);

async function handleStream(client: Client, args: SubscribeRequest) {
  console.log("Starting Stream...")
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

      const parsedTxn = decodeRaydiumLaunchpad(txn);

      if (!parsedTxn) return;

      const rl_formatter = parsedTransactionOutput(parsedTxn,txn);

      console.log(
        new Date(),
        ":",
        `New transaction https://translator.shyft.to/tx/${txn.transaction.signatures[0]} \n`,
        JSON.stringify(rl_formatter.output, null, 2) + "\n",
        JSON.stringify(rl_formatter.transactionEvent) 
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
  process.env.GRPC_URL,
  process.env.X_TOKEN,
  undefined
);

const req: SubscribeRequest = {
  accounts: {},
  slots: {},
  transactions: {
    Raydium_Launchpad: {
      vote: false,
      failed: false,
      signature: undefined,
      accountInclude: [RAYDIUM_LAUNCHPAD_PROGRAM_ID.toBase58()],
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

function decodeRaydiumLaunchpad(tx: VersionedTransactionResponse) {
  if (tx.meta?.err) return;

  const paredIxs = RAYDIUM_LAUNCHPAD_IX_PARSER.parseTransactionData(
    tx.transaction.message,
    tx.meta.loadedAddresses
  );

  const raydiumLaunchpadIxs = paredIxs.filter((ix) =>
    ix.programId.equals(RAYDIUM_LAUNCHPAD_PROGRAM_ID)
  );

  if (raydiumLaunchpadIxs.length === 0) return;
  const events = RAYDIUM_LAUNCHPAD_EVENT_PARSER.parseEvent(tx);
  const result = events.length > 0?{ instructions: raydiumLaunchpadIxs, events }:{instructions : raydiumLaunchpadIxs};
  bnLayoutFormatter(result);
  return result;
}
