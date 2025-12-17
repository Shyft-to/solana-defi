import "dotenv/config";
import Client, {
  CommitmentLevel,
  SubscribeRequest,
} from "@triton-one/yellowstone-grpc";
import { PublicKey, VersionedTransactionResponse } from "@solana/web3.js";
import { Idl } from "@coral-xyz/anchor";
import { SolanaParser } from "@shyft-to/solana-transaction-parser";
import { TransactionFormatter } from "./utils/transaction-formatter";
import meteoraDAMMIdl from "./idls/meteora_damm.json";
import { SolanaEventParser } from "./utils/event/event-parser";
import { bnLayoutFormatter } from "./utils/bn-layout-formatter";
import { meteoradammTransactionOutput } from "./utils/meteora_damm_transaction_output";


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
const METEORA_DAMM_v2_PROGRAM_ID = new PublicKey(
  "cpamdpZCGKUy5JxQXB4dcpGPiikHawvSWAd6mEn1sGG",
);
const METEORA_DAMM_IX_PARSER = new SolanaParser([]);
METEORA_DAMM_IX_PARSER.addParserFromIdl(
  METEORA_DAMM_v2_PROGRAM_ID.toBase58(),
  meteoraDAMMIdl as Idl,
);
const METEORA_DAMM_EVENT_PARSER = new SolanaEventParser([], console);
METEORA_DAMM_EVENT_PARSER.addParserFromIdl(
  METEORA_DAMM_v2_PROGRAM_ID.toBase58(),
  meteoraDAMMIdl as Idl,
);

async function handleStream(client: Client, args: SubscribeRequest) {
  console.log("Streaming Meteora Damm v2 Transactions")
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

  // Handle updates
  stream.on("data", (data) => {
    if (data?.transaction) {
      const txn = TXN_FORMATTER.formTransactionFromJson(
        data.transaction,
        Date.now(),
      );
      const parsedInstruction = decodeMeteoraDAMM(txn);
     // console.log(parsedInstruction)
      if (!parsedInstruction) return;
      const parsedMeteoraDamm = meteoradammTransactionOutput(parsedInstruction,txn)
     console.log(
        new Date(),
        ":",
        `New transaction https://translator.shyft.to/tx/${txn.transaction.signatures[0]} \n`,
        JSON.stringify(parsedMeteoraDamm, null, 2) + "\n"
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
  process.env.RABBITSTREAM_URL,
  process.env.X_TOKEN,
  undefined,
);
const req: SubscribeRequest = {
  accounts: {},
  slots: {},
  transactions: {
    Meteora_DAMM: {
      vote: false,
      failed: false,
      signature: undefined,
      accountInclude: [METEORA_DAMM_v2_PROGRAM_ID.toBase58()],
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



function decodeMeteoraDAMM(tx: VersionedTransactionResponse) {
  if (tx.meta?.err) return;
  try{
   const hydratedTx = hydrateLoadedAddresses(tx);
  const parsedInnerIxs = METEORA_DAMM_IX_PARSER.parseTransactionWithInnerInstructions(hydratedTx);
  const meteora_damm_inner_ixs = parsedInnerIxs.filter((ix) =>
    ix.programId.equals(METEORA_DAMM_v2_PROGRAM_ID)
    || ix.programId.equals(new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"))
    || ix.programId.equals(new PublicKey("ComputeBudget111111111111111111111111111111"))
  );
  if (meteora_damm_inner_ixs.length === 0) return;
  const events = METEORA_DAMM_EVENT_PARSER.parseEvent(tx);
  const result = { inner_ixs: {meteora_damm_inner_ixs, events} };
  bnLayoutFormatter(result);
  return result;
  }catch(err){
  }
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
