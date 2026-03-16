import "dotenv/config";
import Client, {
  CommitmentLevel,
  SubscribeRequest
} from "@triton-one/yellowstone-grpc";
import { VersionedTransactionResponse } from "@solana/web3.js";
import { SolanaParser } from "@shyft-to/solana-transaction-parser";
import { TransactionFormatter } from "./utils/transaction-formatter";
import { RaydiumAmmParser } from "./parsers/raydium-amm-parser";
import { LogsParser } from "./parsers/logs-parser";
import { bnLayoutFormatter } from "./utils/bn-layout-formatter";
import { raydiumTransactionOutput } from "./utils/raydium-formatted-txn";


const RAYDIUM_PUBLIC_KEY = RaydiumAmmParser.PROGRAM_ID;
const TXN_FORMATTER = new TransactionFormatter();
const raydiumAmmParser = new RaydiumAmmParser();
const IX_PARSER = new SolanaParser([]);
IX_PARSER.addParser(
  RaydiumAmmParser.PROGRAM_ID,
  raydiumAmmParser.parseInstruction.bind(raydiumAmmParser),
);

async function handleStream(client: Client, args: SubscribeRequest) {
  const stream = await client.subscribe();

  const streamClosed = new Promise<void>((resolve, reject) => {
    stream.on("error", (error) => {
      console.log("ERROR", error);
      reject(error);
      stream.end();
    });
    stream.on("end", () => { resolve(); });
    stream.on("close", () => { resolve(); });
  });

  stream.on("data", (data) => {
    if (data?.transaction) {
      let txn: VersionedTransactionResponse;
      try {
        txn = TXN_FORMATTER.formTransactionFromJson(data.transaction, Date.now());
      } catch (e) {
        console.error("formTransactionFromJson failed:", e);
        return;
      }

      const parsedTxn = decodeRaydiumTxn(txn);
      if (!parsedTxn) return;
      const formattedTxn = raydiumTransactionOutput(parsedTxn, txn)

      console.log(
        new Date(), ":",
        `New transaction https://translator.shyft.to/tx/${txn.transaction.signatures[0]} \n`,
        JSON.stringify(formattedTxn, null, 2) + "\n"
      );
    }
  });

  await new Promise<void>((resolve, reject) => {
    stream.write(args, (err: any) => {
      if (err === null || err === undefined) { resolve(); }
      else { reject(err); }
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
  process.env.RABBITSTREAM_URL!,
  process.env.X_TOKEN,
  undefined,
);

const req: SubscribeRequest = {
  accounts: {},
  slots: {},
  transactions: {
    raydiumLiquidityPoolV4: {
      vote: false,
      failed: false,
      signature: undefined,
      accountInclude: [RAYDIUM_PUBLIC_KEY.toBase58()],
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
  commitment: CommitmentLevel.PROCESSED,
};

subscribeCommand(client, req);

function decodeRaydiumTxn(tx: VersionedTransactionResponse) {
  if (tx.meta?.err) return;

  let parsedIxs;
  try {
    parsedIxs = IX_PARSER.parseTransactionWithInnerInstructions(tx);
  } catch (e: any) {
    return;
  }

  const programIxs = parsedIxs.filter((ix) =>
    ix.programId.equals(RAYDIUM_PUBLIC_KEY),
  );
  if (programIxs.length === 0) return;

  const result = { instructions: parsedIxs };
  bnLayoutFormatter(result);
  return result;
}