require('dotenv').config()
import Client, {
  CommitmentLevel,
  SubscribeRequest,
} from "@triton-one/yellowstone-grpc";
import { PublicKey, VersionedTransactionResponse } from "@solana/web3.js";
import { Idl } from "@project-serum/anchor";
import { SolanaParser } from "@shyft-to/solana-transaction-parser";
import { TransactionFormatter } from "./utils/transaction-formatter";
import meteoraPoolIdl from "./idls/meteora_pools.json";
import { SolanaEventParser } from "./utils/event/event-parser";
import { bnLayoutFormatter } from "./utils/bn-layout-formatter";
import { meteoraPoolTxn } from "./utils/meteora-pool-transaction";


const TXN_FORMATTER = new TransactionFormatter();
const METEORA_POOL_PROGRAM_ID = new PublicKey(
  "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB",
);
const METEORA_POOL_IX_PARSER = new SolanaParser([]);
METEORA_POOL_IX_PARSER.addParserFromIdl(
  METEORA_POOL_PROGRAM_ID.toBase58(),
  meteoraPoolIdl as Idl,
);
const METEORA_POOL_EVENT_PARSER = new SolanaEventParser([], console);
METEORA_POOL_EVENT_PARSER.addParserFromIdl(
  METEORA_POOL_PROGRAM_ID.toBase58(),
  meteoraPoolIdl as Idl,
);

async function handleStream(client: Client, args: SubscribeRequest) {
  console.log("Streaming Meteora Pool TXN...")
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
      const parsedInstructions = decodeMeteoraPool(txn);

      if (!parsedInstructions) return;
      const parsedTxn = meteoraPoolTxn(parsedInstructions,txn)
      if(!parsedTxn)return;
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
      accountInclude: [METEORA_POOL_PROGRAM_ID.toBase58()],
      accountExclude: [],
      accountRequired: [],
    },
  },
  entry: {},
  transactionsStatus: {},
  blocks: {},
  blocksMeta: {},
  accountsDataSlice: [],
  ping: undefined,
  commitment: CommitmentLevel.CONFIRMED,
};

subscribeCommand(client, req);

function decodeMeteoraPool(tx: VersionedTransactionResponse) {
  if (tx.meta?.err) return;

  const paredIxs = METEORA_POOL_IX_PARSER.parseTransactionData(
    tx.transaction.message,
    tx.meta.loadedAddresses,
  );

  const meteora_pool_Ixs = paredIxs.filter((ix) =>
    ix.programId.equals(METEORA_POOL_PROGRAM_ID) || ix.programId.equals(new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")),
  );

  const parsedInnerIxs = METEORA_POOL_IX_PARSER.parseTransactionWithInnerInstructions(tx);

  const meteora_pool_inner_ixs = parsedInnerIxs.filter((ix) =>
    ix.programId.equals(METEORA_POOL_PROGRAM_ID) || ix.programId.equals(new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA")),
  );


  if (meteora_pool_Ixs.length === 0) return;
  const events = METEORA_POOL_EVENT_PARSER.parseEvent(tx);
  const result = { instructions: meteora_pool_Ixs, inner_ixs: {meteora_pool_inner_ixs, events} };
  bnLayoutFormatter(result);
  return result;
}