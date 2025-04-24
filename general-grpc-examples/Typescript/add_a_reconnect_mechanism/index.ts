require("dotenv").config();
import Client, { CommitmentLevel } from "@triton-one/yellowstone-grpc";
import { SubscribeRequest } from "@triton-one/yellowstone-grpc/dist/types/grpc/geyser";
import * as bs58 from "bs58";

const MAX_RETRY_WITH_LAST_SLOT = 30;
const RETRY_DELAY_MS = 1000;
const ADDRESS_TO_STREAM_FROM = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

type StreamResult = {
  lastSlot?: string;
  hasRcvdMSg: boolean;
};

async function handleStream(
  client: Client,
  args: SubscribeRequest,
  lastSlot?: string
): Promise<StreamResult> {
  const stream = await client.subscribe();
  let hasRcvdMSg = false;

  return new Promise((resolve, reject) => {
    stream.on("data", (data) => {
      const tx = data.transaction?.transaction?.transaction;
      if (tx?.signatures?.[0]) {
        const sig = bs58.encode(tx.signatures[0]);
        console.log("Got tx:", sig);
        lastSlot = data.transaction.slot;
        hasRcvdMSg = true;
      }
    });

    stream.on("error", (err) => {
      stream.end();
      reject({ error: err, lastSlot, hasRcvdMSg });
    });

    const finalize = () => resolve({ lastSlot, hasRcvdMSg });
    stream.on("end", finalize);
    stream.on("close", finalize);

    stream.write(args, (err: any) => {
      if (err) reject({ error: err, lastSlot, hasRcvdMSg });
    });
  });
}

async function subscribeCommand(client: Client, args: SubscribeRequest) {
  let lastSlot: string | undefined;
  let retryCount = 0;

  while (true) {
    try {
      if (args.fromSlot) {
        console.log("Starting stream from slot", args.fromSlot);
      }

      const result = await handleStream(client, args, lastSlot);
      lastSlot = result.lastSlot;
      if (result.hasRcvdMSg) retryCount = 0;
    } catch (err: any) {
      console.error(
        `Stream error, retrying in ${RETRY_DELAY_MS / 1000} second...`
      );
      await new Promise((resolve) => setTimeout(resolve, RETRY_DELAY_MS));

      lastSlot = err.lastSlot;
      if (err.hasRcvdMSg) retryCount = 0;

      if (lastSlot && retryCount < MAX_RETRY_WITH_LAST_SLOT) {
        console.log(
          `#${retryCount} retrying with last slot ${lastSlot}, remaining retries ${
            MAX_RETRY_WITH_LAST_SLOT - retryCount
          }`
        );
        args.fromSlot = lastSlot;
        retryCount++;
      } else {
        console.log("Retrying from latest slot (no last slot available)");
        delete args.fromSlot;
        retryCount = 0;
        lastSlot = undefined;
      }
    }
  }
}

const client = new Client(process.env.GRPC_URL!, process.env.X_TOKEN!, {
  "grpc.keepalive_permit_without_calls": 1,
  "grpc.keepalive_time_ms": 10000,
  "grpc.keepalive_timeout_ms": 1000,
  "grpc.default_compression_algorithm": 2,
});

const req: SubscribeRequest = {
  accounts: {},
  slots: {},
  transactions: {
    pumpFun: {
      vote: false,
      failed: false,
      accountInclude: [ADDRESS_TO_STREAM_FROM],
      accountExclude: [],
      accountRequired: [],
    },
  },
  transactionsStatus: {},
  blocks: {},
  blocksMeta: {},
  entry: {},
  accountsDataSlice: [],
  commitment: CommitmentLevel.CONFIRMED,
};

subscribeCommand(client, req);
