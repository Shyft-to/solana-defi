import "dotenv/config";

import * as fastq from "fastq";
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
import { utils } from "@project-serum/anchor";

import { SlotMapper } from "./utils/slot-mapper";

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

interface TransactionUpdate {
  type: "transaction" | "slot";
  transactionSignature: string;
  createdAtReceived: number;
  blocktimeReceived: number;
  slot: number;
  gRPCBlocktime: number;
}

const PUBLIC_KEY_TO_LISTEN =
  process.env.PUBLIC_KEY_TO_LISTEN ??
  "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8";
const RUN_TIME = Number(process.env.RUN_TIME ?? 2) * 60 * 1000;
let slotMapper = new SlotMapper();

let commitment = process.env.COMMITMENT_LEVEL ?? "confirmed";
const commitmentLevelToUse =
  commitment === "confirmed"
    ? CommitmentLevel.CONFIRMED
    : commitment === "processed"
    ? CommitmentLevel.PROCESSED
    : CommitmentLevel.FINALIZED;

const client = new Client(
  process.env.ENDPOINT!,
  process.env.X_TOKEN,
  undefined
);

const req: SubscribeRequest = {
  accounts: {},
  slots: {},
  transactions: {},
  transactionsStatus: {
    raydiumLiquidityPoolV4: {
      vote: false,
      failed: false,
      signature: undefined,
      accountInclude: [PUBLIC_KEY_TO_LISTEN],
      accountExclude: [],
      accountRequired: [],
    },
  },
  entry: {},
  blocks: {},
  blocksMeta: {
    blockmetadata: {},
  },
  accountsDataSlice: [],
  ping: undefined,
  commitment: commitmentLevelToUse,
};

const worker: fastq.worker<TransactionUpdate> = async (
  txn_item: TransactionUpdate
) => {
  // console.log("executing worker");
  const {
    type,
    transactionSignature,
    createdAtReceived,
    blocktimeReceived,
    slot,
    gRPCBlocktime,
  } = txn_item;

  if (type === "slot") {
    slotMapper.addDatafromBlockMeta(slot.toString(), gRPCBlocktime);
  } else if (type === "transaction") {
    slotMapper.addDatafromTransaction(
      slot.toString(),
      transactionSignature,
      createdAtReceived,
      0,
      blocktimeReceived
    );
  } else {
    console.log("Unknown type: ", type);
    return;
  }
};

//@ts-ignore
const q = fastq.promise(worker, 100);

async function handleStream(client: Client, args: SubscribeRequest) {
  console.log(`Subscribing and starting stream...`);
  const stream = await client.subscribe();
  console.log(`Streaming data and collecting info...`);
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
    if (data?.blockMeta) {
      const slotReceived = data?.blockMeta?.slot;
      const blocktimeReceived = data?.blockMeta?.blockTime?.timestamp
        ? Number(data?.blockMeta?.blockTime?.timestamp) * 1000
        : 0;
      q.push({
        type: "slot",
        slot: slotReceived,
        gRPCBlocktime: blocktimeReceived,
        createdAtReceived: 0,
        blocktimeReceived: 0,
        transactionSignature: "",
      });
      return;
    }

    if (data?.transactionStatus) {
      const receivedTime = Date.now();
      const transactionSignature = data?.transactionStatus?.signature
        ? utils.bytes.bs58.encode(data?.transactionStatus?.signature)
        : "";

      const receivedSlot = data?.transactionStatus?.slot;

      let createdAt = 0;
      if (data?.transactionStatus?.createdAt) {
        createdAt = new Date(data?.transactionStatus?.createdAt).getTime();
      }

      try {
        q.push({
          type: "transaction",
          transactionSignature,
          createdAtReceived: createdAt,
          blocktimeReceived: receivedTime,
          slot: receivedSlot,
          gRPCBlocktime: 0,
        });
        return;
      } catch (error) {
        console.error("parsing error: ", error, transactionSignature);
      }
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

  setTimeout(() => {
    console.log("Ending stream now...");

    stream.end();
    stream.destroy();
    slotMapper.displayGroupedTxnData();
    process.exit(0);
  }, RUN_TIME);

  await streamClosed;
}

async function subscribeCommand(client: Client, args: SubscribeRequest) {
  await handleStream(client, args);
}

subscribeCommand(client, req);
