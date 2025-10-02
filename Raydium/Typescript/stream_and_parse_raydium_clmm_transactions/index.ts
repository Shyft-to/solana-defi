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
import raydiumClmmIdl from "./idls/raydium_clmm..json";
import { raydiumClmmFormatter } from "./utils/raydium-clmm-transaction-formatter";


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
  "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK"
);
const TOKEN_PROGRAM_ID = new PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
);
const RAYDIUM_CLMM_IX_PARSER = new SolanaParser([]);
RAYDIUM_CLMM_IX_PARSER.addParserFromIdl(
  RAYDIUM_CLMM_PROGRAM_ID.toBase58(),
  raydiumClmmIdl as Idl
);
const RAYDIUM_CLMM_EVENT_PARSER = new SolanaEventParser([], console);
RAYDIUM_CLMM_EVENT_PARSER.addParserFromIdl(
  RAYDIUM_CLMM_PROGRAM_ID.toBase58(),
  raydiumClmmIdl as Idl
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

      const parsedTxn = decodeRaydiumClmmTxn(txn);
      if (!parsedTxn) return;
      const raydiumClmmTransactions = raydiumClmmFormatter(parsedTxn, txn);
      if (!raydiumClmmTransactions) return;
      console.log(
        new Date(),
        ":",
        `New transaction https://translator.shyft.to/tx/${txn.transaction.signatures[0]} \n`,
        JSON.stringify(raydiumClmmTransactions, null, 2) + "\n"
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
  process.env.GRPC_URL!,
  process.env.X_TOKEN,
  undefined
);

const req: SubscribeRequest = {
  accounts: {},
  slots: {},
  transactions: {
    pumpAMM: {
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

function decodeRaydiumClmmTxn(tx: VersionedTransactionResponse) {
  if (tx.meta?.err) return;
   const hydratedTx = hydrateLoadedAddresses(tx);

  const paredIxs = RAYDIUM_CLMM_IX_PARSER.parseTransactionData(
    hydratedTx.transaction.message,
    hydratedTx.meta.loadedAddresses
  );
  const raydiumClmmIxs = paredIxs.filter((ix) =>
    ix.programId.equals(RAYDIUM_CLMM_PROGRAM_ID)  || 
        ix.programId.equals(TOKEN_PROGRAM_ID)
  );
  const parsedInnerIxs = RAYDIUM_CLMM_IX_PARSER.parseTransactionWithInnerInstructions(hydratedTx);

   let raydium_clmm_inner_ixs = parsedInnerIxs.filter((ix) =>
        ix.programId.equals(RAYDIUM_CLMM_PROGRAM_ID) || 
        ix.programId.equals(TOKEN_PROGRAM_ID)
   );

  if (raydiumClmmIxs.length === 0 && raydium_clmm_inner_ixs.length === 0) return;
  const events = RAYDIUM_CLMM_EVENT_PARSER.parseEvent(tx);
  const result = { 
   instructions: raydiumClmmIxs, 
   innerInstructions: raydium_clmm_inner_ixs,
   events 
 }; 
  bnLayoutFormatter(result);
  return result;
}



  function hydrateLoadedAddresses(tx: VersionedTransactionResponse): VersionedTransactionResponse {
    const loaded = tx.meta?.loadedAddresses;
    if (!loaded) return tx;

    function ensurePublicKey(arr: (Buffer | PublicKey)[]) {
      return arr.map(item =>
        item instanceof PublicKey ? item : new PublicKey(item)
      );
    }

    tx.meta.loadedAddresses = {
      writable: ensurePublicKey(loaded.writable),
      readonly: ensurePublicKey(loaded.readonly),
    };

    return tx;
  }
