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
import { TransactionFormatter } from "./utils/transaction-formatter";
import raydiumClmmIdl from "./idls/raydium_clmm.json";
import { SolanaEventParser } from "./utils/event-parser";
import { bnLayoutFormatter } from "./utils/bn-layout-formatter";
import { parsedTransactionOutput } from "./utils/parsedTransactionOutput";

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
const RAYDIUM_CLMM_PROGRAM_ID = new PublicKey(
  "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK",
);
const RAYDIUM_CLMM_IX_PARSER = new SolanaParser([]);
RAYDIUM_CLMM_IX_PARSER.addParserFromIdl(
  RAYDIUM_CLMM_PROGRAM_ID.toBase58(),
  raydiumClmmIdl as Idl,
);
const RAYDIUM_CLMM_EVENT_PARSER = new SolanaEventParser([], console);
RAYDIUM_CLMM_EVENT_PARSER.addParserFromIdl(
  RAYDIUM_CLMM_PROGRAM_ID.toBase58(),
  raydiumClmmIdl as Idl,
);

async function handleStream(client: Client, args: SubscribeRequest) {
  // Subscribe for events
  console.log("Streaming Started...")
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
      const parsedInstructions = decodeRaydiumClmm(txn);

      if (!parsedInstructions) return;
      const formattedTxn = parsedTransactionOutput(parsedInstructions,txn)
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
    Meteora_Pool: {
      vote: false,
      failed: false,
      signature: undefined,
      accountInclude: [RAYDIUM_CLMM_PROGRAM_ID.toBase58()],
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

function decodeRaydiumClmm(tx: VersionedTransactionResponse) {
  if (tx.meta?.err) return;

  const paredIxs = RAYDIUM_CLMM_IX_PARSER.parseTransactionData(
    tx.transaction.message,
    tx.meta.loadedAddresses,
  );

  const raydium_clmm_Ixs = paredIxs.filter((ix) =>
    ix.programId.equals(RAYDIUM_CLMM_PROGRAM_ID) || ix.programId.equals(new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")),
  );

  const parsedInnerIxs = RAYDIUM_CLMM_IX_PARSER.parseTransactionWithInnerInstructions(tx);

  const raydium_clmm_inner_ixs = parsedInnerIxs.filter((ix) =>
    ix.programId.equals(RAYDIUM_CLMM_PROGRAM_ID) || ix.programId.equals(new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")),
  );


  if (raydium_clmm_Ixs.length === 0) return;
  const events = RAYDIUM_CLMM_EVENT_PARSER.parseEvent(tx);
  const result = { instructions: raydium_clmm_Ixs, inner_ixs: raydium_clmm_inner_ixs, events };
  bnLayoutFormatter(result);
  return result;
}