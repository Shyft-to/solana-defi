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
import { PublicKey, VersionedTransactionResponse } from "@solana/web3.js";
import { Idl } from "@coral-xyz/anchor";
import { SolanaParser } from "@shyft-to/solana-transaction-parser";
import { TransactionFormatter } from "./utils/transaction-formatter";
import meteoraDAMMIdl from "./idls/meteora_damm.json";
import { SolanaEventParser } from "./utils/event-parser";
import { bnLayoutFormatter } from "./utils/bn-layout-formatter";
import { meteoradammTransactionOutput } from "./utils/meteora_damm_transaction_output";

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
        Date.now(),
      );
      const parsedInstruction = decodeMeteoraDAMM(txn);

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
  process.env.GRPC_URL,
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
  const paredIxs = METEORA_DAMM_IX_PARSER.parseTransactionData(
    tx.transaction.message,
    tx.meta.loadedAddresses,
  );

  const meteora_DAMM_Ixs = paredIxs.filter((ix) =>
    ix.programId.equals(METEORA_DAMM_v2_PROGRAM_ID) 
    || ix.programId.equals(new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"))
    || ix.programId.equals(new PublicKey("ComputeBudget111111111111111111111111111111"))

    ,
  );

  const parsedInnerIxs = METEORA_DAMM_IX_PARSER.parseTransactionWithInnerInstructions(tx);
  const meteroa_damm_inner_ixs = parsedInnerIxs.filter((ix) =>
    ix.programId.equals(METEORA_DAMM_v2_PROGRAM_ID)
    || ix.programId.equals(new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"))
    || ix.programId.equals(new PublicKey("ComputeBudget111111111111111111111111111111"))
    ,
  );


  if (meteora_DAMM_Ixs.length === 0) return;
  const events = METEORA_DAMM_EVENT_PARSER.parseEvent(tx);
  const result = { instructions: meteora_DAMM_Ixs, inner_ixs:  meteroa_damm_inner_ixs, events };
  bnLayoutFormatter(result);
  return result;
  }catch(err){
  }
}