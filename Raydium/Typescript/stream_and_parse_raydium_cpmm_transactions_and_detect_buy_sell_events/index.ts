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
import { PublicKey, VersionedTransactionResponse } from "@solana/web3.js";
import { Idl } from "@project-serum/anchor";
import { SolanaParser } from "@shyft-to/solana-transaction-parser";
import { parsedTransactionOutput } from "./utils/parsedTransactionOutput";
import { TransactionFormatter } from "./utils/transaction-formatter";
import cpmmIDL from "./idls/cpmm_idl.json";

import { bnLayoutFormatter } from "./utils/bn-layout-formatter";

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
const CPMM_PROGRAM_ID = new PublicKey(
  "CPMMoo8L3F4NbTegBCKVNunggL7H1ZpdTHKxQB5qKP1C",
);
const TOKEN_PROGRAM_ID = new PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
)
const CPMM_IX_PARSER = new SolanaParser([]);
CPMM_IX_PARSER.addParserFromIdl(
  CPMM_PROGRAM_ID.toBase58(),
  cpmmIDL as Idl,
);


async function handleStream(client: Client, args: SubscribeRequest) {
  console.log("Listening to Buy and Sell Transactions...");
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

  stream.on("data", (data) => {
    if (data?.transaction) {
      const txn = TXN_FORMATTER.formTransactionFromJson(
        data.transaction,
        Date.now(),
      );

      console.log("Txn Received: ", txn.transaction.signatures[0]);

      const parsedTxn = decodeCpmmTxn(txn);
      if (!parsedTxn) return; 
      const formattedTxn = parsedTransactionOutput(parsedTxn,txn);
      console.log(JSON.stringify(formattedTxn));
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
      accountInclude: [CPMM_PROGRAM_ID.toBase58()],
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

function decodeCpmmTxn(tx: VersionedTransactionResponse) {
  if (tx.meta?.err) return;

  const paredIxs = CPMM_IX_PARSER.parseTransactionData(
    tx.transaction.message,
    tx.meta.loadedAddresses,
  );

  const parsedInnerIxs = CPMM_IX_PARSER.parseTransactionWithInnerInstructions(
    tx
  );

  const compiledIxs = paredIxs.filter((ix) =>
    ix.programId.equals(CPMM_PROGRAM_ID) || ix.programId.equals(TOKEN_PROGRAM_ID),
  );

  const parsedFilteredInnerIxs = parsedInnerIxs.filter((ix) =>
    ix.programId.equals(CPMM_PROGRAM_ID) || ix.programId.equals(TOKEN_PROGRAM_ID),
  );

  const result = { compiledInstructions: compiledIxs, innerInstructions: parsedFilteredInnerIxs };
  bnLayoutFormatter(result);
  return result;
}
