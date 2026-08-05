import "dotenv/config";
import Client, { CommitmentLevel, SubscribeRequest } from "@triton-one/yellowstone-grpc";
import { PublicKey } from "@solana/web3.js";
import { Idl } from "@coral-xyz/anchor";
import { Parser, TransactionStreamer } from "@shyft-to/ladybug-sdk";

import pumpFunIdl from "./pump_fun.json";

// Pump.fun program id (also the `address` field inside pump_fun.json)
const PUMP_FUN_PROGRAM_ID = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

const ENDPOINT = process.env.ENDPOINT!;
const X_TOKEN = process.env.X_TOKEN;

if (!ENDPOINT) {
  throw new Error("Missing ENDPOINT env var (Yellowstone gRPC endpoint)");
}

const BLOCKS_META_FILTER_KEY = "pumpfun";
// Drop any slot's buffered txs if its blockMeta hasn't shown up within this
// many slots of the latest one seen — mirrors what the SDK's own
// LatencyChecker does internally, so a skipped/forked slot can't leak memory.
const MAX_SLOT_LAG = 20;

interface PendingTx {
  signature: string;
  receivedTime: number;
}

// slot -> [{ signature, receivedTime }, ...] — filled in by the transaction
// streamer, drained/matched against by the blockMeta stream.
const slotTxMap = new Map<string, PendingTx[]>();

async function main() {
  // 1. Parser: needed by the ladybug TransactionStreamer to decode Pump.fun txs
  console.time("1. init parser + addIDL");
  const parser = new Parser();
  parser.addIDL(new PublicKey(PUMP_FUN_PROGRAM_ID), pumpFunIdl as Idl);
  console.timeEnd("1. init parser + addIDL");

  // 2. ladybug SDK's TransactionStreamer — same pattern as index.ts. For each
  //    parsed tx received, record { slot -> [{ signature, receivedTime }] }.
  console.time("2. init + wire up TransactionStreamer");
  const txStreamer = new TransactionStreamer(ENDPOINT, X_TOKEN);
  await txStreamer.addParser(parser);
  await txStreamer.addAddresses([PUMP_FUN_PROGRAM_ID]);

  txStreamer.onData((tx: any) => {
    const receivedTime = Date.now();
    const slot = String(tx?.slot);
    const signature: string | undefined = tx?.transaction?.signatures?.[0];
    if (!signature) return;

    const entries = slotTxMap.get(slot) ?? [];
    entries.push({ signature, receivedTime });
    slotTxMap.set(slot, entries);
  });
  txStreamer.onError((err) => console.error("[TransactionStreamer] error:", err));
  txStreamer.onEnd(() => console.log("[TransactionStreamer] stream ended."));
  console.timeEnd("2. init + wire up TransactionStreamer");

  // start() only resolves once the stream ends/stops, so run it in the
  // background rather than awaiting it here — we still need to set up the
  // blockMeta stream below.
  txStreamer.start().catch((err) => console.error("[TransactionStreamer] fatal:", err));

  // 3. A second, independent connection using the raw @triton-one/yellowstone-grpc
  //    client (not the ladybug streamer), subscribed only to blocksMeta — just
  //    slot + blockTime, no transactions. This is what we correlate against
  //    the map the transaction streamer is filling above.
  console.time("3. init raw client + open blocksMeta stream");
  const grpcClient = new Client(ENDPOINT, X_TOKEN, undefined);
  const blocksMetaStream = await grpcClient.subscribe();
  console.timeEnd("3. init raw client + open blocksMeta stream");

  const blocksMetaRequest: SubscribeRequest = {
    accounts: {},
    slots: {},
    transactions: {},
    transactionsStatus: {},
    entry: {},
    blocks: {},
    blocksMeta: { [BLOCKS_META_FILTER_KEY]: {} },
    accountsDataSlice: [],
    commitment: CommitmentLevel.CONFIRMED,
  };

  console.time("4. send blocksMeta subscribe request");
  await new Promise<void>((resolve, reject) => {
    blocksMetaStream.write(blocksMetaRequest, (err: Error | null | undefined) =>
      err ? reject(err) : resolve()
    );
  });
  console.timeEnd("4. send blocksMeta subscribe request");

  blocksMetaStream.on("data", handleBlockMeta);
  blocksMetaStream.on("error", (err: any) => console.error("[blocksMeta] stream error:", err));
  blocksMetaStream.on("end", () => console.log("[blocksMeta] stream ended."));
  blocksMetaStream.on("close", () => console.log("[blocksMeta] stream closed."));

  console.log(`\nStreaming Pump.fun tx latency (SDK TransactionStreamer + raw blocksMeta) from ${ENDPOINT} ...\n`);
}

function handleBlockMeta(update: any) {
  const blockMeta = update.blockMeta;
  if (!blockMeta) return; // ignore pings / other update kinds

  const slot = String(blockMeta.slot);
  const blockTimeSec = blockMeta.blockTime?.timestamp as string | undefined;

  if (blockTimeSec !== undefined) {
    const blockTimeMs = Number(blockTimeSec) * 1000;
    const entries = slotTxMap.get(slot);

    if (entries?.length) {
      for (const { signature, receivedTime } of entries) {
        const latencyMs = receivedTime - blockTimeMs;
        console.log(
          `slot=${slot} sig=${signature} blockTime=${new Date(blockTimeMs).toISOString()} receivedAt=${new Date(
            receivedTime
          ).toISOString()} latency=${latencyMs}ms`
        );
      }
      slotTxMap.delete(slot); // matched — free it
    }
  }

  // Evict anything left too far behind the current slot (blockMeta that will
  // never arrive for it, e.g. a skipped/forked slot) so the map can't grow
  // unbounded.
  const currentSlotNum = Number(slot);
  for (const storedSlot of slotTxMap.keys()) {
    if (Number(storedSlot) < currentSlotNum - MAX_SLOT_LAG) {
      slotTxMap.delete(storedSlot);
    }
  }
}

main().catch((err) => {
  console.error("Fatal error:", err);
  process.exit(1);
});
