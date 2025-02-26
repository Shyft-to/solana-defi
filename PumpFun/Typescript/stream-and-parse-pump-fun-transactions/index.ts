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
import pumpFunIdl from "./idls/pump_0.1.0.json";
import { SolanaEventParser } from "./utils/event-parser";
import { bnLayoutFormatter } from "./utils/bn-layout-formatter";
import { transactionOutput } from "./utils/transactionOutput";

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
const PUMP_FUN_PROGRAM_ID = new PublicKey(
  "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
);
const TOKEN_PROGRAM_ID = new PublicKey(
  "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
)
const PUMP_FUN_IX_PARSER = new SolanaParser([]);
PUMP_FUN_IX_PARSER.addParserFromIdl(
  PUMP_FUN_PROGRAM_ID.toBase58(),
  pumpFunIdl as Idl,
);

const GEN_IX_PARSER = new SolanaParser([]);

const PUMP_FUN_EVENT_PARSER = new SolanaEventParser([], console);
PUMP_FUN_EVENT_PARSER.addParserFromIdl(
  PUMP_FUN_PROGRAM_ID.toBase58(),
  pumpFunIdl as Idl,
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

      // console.log("txn from json: ");
      // console.dir(txn, { depth: null });

      console.log("signature: ", txn.transaction.signatures);

      const parsedTxn = decodePumpFunTxn(txn);

      const rpcTxnWithParsed = {
        ...txn,
        meta: {
          ...txn.meta,
          parsed: parsedTxn,
        },
      }

      // console.log("parsed Transaction: ");
      
      // // // console.dir(JSON.stringify(rpcTxnWithParsed), { depth: null });
      console.log(JSON.stringify(rpcTxnWithParsed));

      // if (!parsedTxn) return;
      // const tOutput = transactionOutput(parsedTxn)

      // console.log(
      //   `
      //   TYPE : ${tOutput.type}
      //   MINT : ${tOutput.mint}
      //   SIGNER : ${tOutput.user}
      //   TOKEN AMOUNT : ${tOutput.tokenAmount}
      //   SOL AMOUNT : ${tOutput.solAmount} SOL
      //   SIGNATURE : ${txn.transaction.signatures[0]}
      //   `
      // )
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

function decodePumpFunTxn(tx: VersionedTransactionResponse) {
  if (tx.meta?.err) return;

  // const paredIxs = PUMP_FUN_IX_PARSER.parseTransactionData(
  //   tx.transaction.message,
  //   tx.meta.loadedAddresses,
  // );

  const paredIxs = PUMP_FUN_IX_PARSER.parseTransactionWithInnerInstructions(
    tx
  );
  


  // console.log("txn from json: ");
  // console.dir(JSON.stringify(paredIxs), { depth: null });

  const pumpFunAndTokenIxs = paredIxs.filter((ix) =>
    ix.programId.equals(PUMP_FUN_PROGRAM_ID) || ix.programId.equals(TOKEN_PROGRAM_ID),
  );

  if (pumpFunAndTokenIxs.length === 0) return;
  
  const result = { instructions: pumpFunAndTokenIxs };
  bnLayoutFormatter(result);
  return result;
}
