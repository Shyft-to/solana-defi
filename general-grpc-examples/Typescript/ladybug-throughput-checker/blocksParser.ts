import "dotenv/config";
import Client, { CommitmentLevel, SubscribeRequest } from "@triton-one/yellowstone-grpc";
import { Parser } from "@shyft-to/ladybug-sdk";

// Same two-stream setup as dualStream.ts (raw @triton-one/yellowstone-grpc Client,
// STREAM_1_ADDRESSES / STREAM_2_ADDRESSES from .env, Parser with only
// `useDefaultInstructionParsing(true)` — no addIDL() calls), but subscribed to
// `blocks` instead of `transactions`: each stream gets whole blocks (filtered by
// its address list via the blocks filter's accountInclude) and then parses every
// transaction inside each block using the ladybug SDK Parser.
//
// NOTE on the blocks filter's accountInclude semantics: unlike the transactions
// filter (which returns only the matching transactions), a block filter's
// accountInclude is a whole-block gate — a block is emitted if ANY of its
// transactions touches one of the given addresses, and (with
// includeTransactions: true) the block arrives with ALL of its transactions
// attached, not just the matching one(s). So "tx count" below is the size of
// every matched block, not just address hits.
//
// Two throughput stages are tracked per stream: "streamed" (transactions as they
// arrive inside a block, before parsing) and "parsed" (transactions the Parser
// has finished decoding). A block's transactions are parsed via Promise.all
// rather than sequential awaits, and block parsing is kicked off without
// blocking the stream's `data` handler, so a slow block's parsing can't stall
// the next block from arriving. Note this is *not* true multi-core parallelism —
// Node is single-threaded and Parser.parseTransaction/formatGrpcTransactionData
// are synchronous, so within a single block the transactions still parse one at
// a time on the event loop; what Promise.all + not-awaiting-inline buys you is
// that no block's parsing blocks another block's, and none of it blocks the
// stream's own I/O.

const ENDPOINT = process.env.ENDPOINT!;
const X_TOKEN = process.env.X_TOKEN;
// How long each stream runs before it's stopped automatically. Set to 0/blank to run forever
// (until Ctrl+C), same as index.ts / dualStream.ts.
const RUN_DURATION_MS = process.env.RUN_DURATION_MS ? Number(process.env.RUN_DURATION_MS) : 30_000;

if (!ENDPOINT) {
  throw new Error("Missing ENDPOINT env var (Yellowstone gRPC endpoint)");
}

// Empty/unset -> [] (caller decides whether that means "disabled").
function parseAddressList(raw: string | undefined): string[] {
  return (raw ?? "")
    .split(",")
    .map((a) => a.trim())
    .filter(Boolean);
}

// A stream with an empty address list is simply skipped (see main()), so you can leave
// either STREAM_n_ADDRESSES blank/unset to run with just one stream.
const STREAMS = [
  { label: "stream-1", envVar: "STREAM_1_ADDRESSES", addresses: parseAddressList(process.env.STREAM_1_ADDRESSES) },
  { label: "stream-2", envVar: "STREAM_2_ADDRESSES", addresses: parseAddressList(process.env.STREAM_2_ADDRESSES) },
];

// Each stream gets its own Parser instance. No addIDL() calls — default parsing
// only, so only System/Token/Token2022 instructions decode; anything else passes
// through as raw compiled instruction data.
function buildParser(): Parser {
  const parser = new Parser();
  parser.enableLogging(false);
  parser.useDefaultInstructionParsing(true);
  return parser;
}

// Tracks two throughput stages ("streamed" = arrived in a block, "parsed" =
// decoded by the Parser) for one stream, printing both tx/sec figures once a
// second, and both avg/peak summaries when stopped.
function makeStageCounters(label: string) {
  const streamed = { windowCount: 0, totalCount: 0, peakCount: 0 };
  const parsed = { windowCount: 0, totalCount: 0, peakCount: 0 };
  const startedAt = Date.now();

  const interval = setInterval(() => {
    if (streamed.windowCount > streamed.peakCount) streamed.peakCount = streamed.windowCount;
    if (parsed.windowCount > parsed.peakCount) parsed.peakCount = parsed.windowCount;
    console.log(
      `[${label}] streamed: ${streamed.windowCount} tx/sec (total: ${streamed.totalCount}) | ` +
        `parsed: ${parsed.windowCount} tx/sec (total: ${parsed.totalCount})`
    );
    streamed.windowCount = 0;
    parsed.windowCount = 0;
  }, 1000);
  interval.unref();

  return {
    tickStreamed: () => {
      streamed.windowCount++;
      streamed.totalCount++;
    },
    tickParsed: () => {
      parsed.windowCount++;
      parsed.totalCount++;
    },
    stop: () => {
      clearInterval(interval);
      // account for the trailing partial second
      if (streamed.windowCount > streamed.peakCount) streamed.peakCount = streamed.windowCount;
      if (parsed.windowCount > parsed.peakCount) parsed.peakCount = parsed.windowCount;
      const elapsedSec = (Date.now() - startedAt) / 1000;
      const avg = (total: number) => (elapsedSec > 0 ? total / elapsedSec : 0);
      console.log(
        `[${label}] summary — streamed: total ${streamed.totalCount} tx over ${elapsedSec.toFixed(1)}s, ` +
          `avg: ${avg(streamed.totalCount).toFixed(2)} tx/sec, peak: ${streamed.peakCount} tx/sec`
      );
      console.log(
        `[${label}] summary — parsed:   total ${parsed.totalCount} tx over ${elapsedSec.toFixed(1)}s, ` +
          `avg: ${avg(parsed.totalCount).toFixed(2)} tx/sec, peak: ${parsed.peakCount} tx/sec`
      );
    },
  };
}

async function runStream(label: string, addresses: string[]): Promise<void> {
  console.time(`[${label}] init parser`);
  const parser = buildParser();
  console.timeEnd(`[${label}] init parser`);

  console.time(`[${label}] init client + open stream`);
  const client = new Client(ENDPOINT, X_TOKEN, undefined);
  // @triton-one/yellowstone-grpc@5 requires an explicit connect() before subscribe()
  // (subscribe() throws "Client not connected. Call connect() first" otherwise).
  await client.connect();
  const stream = await client.subscribe();
  console.timeEnd(`[${label}] init client + open stream`);

  // A "blocks" filter (not "transactions"), scoped to `addresses` via
  // accountInclude. includeTransactions: true is required to get each block's
  // transaction list attached — without it, blocks arrive with slot/hash/etc.
  // but an empty transactions array.
  const request: SubscribeRequest = {
    accounts: {},
    slots: {},
    transactions: {},
    transactionsStatus: {},
    entry: {},
    blocks: {
      tracked: {
        accountInclude: [],
        includeTransactions: true,
        includeAccounts: false,
        includeEntries: false,
      },
    },
    blocksMeta: {},
    accountsDataSlice: [],
    // Unlike the transactions filter (PROCESSED works fine, see dualStream.ts),
    // this endpoint doesn't emit "blocks" updates at PROCESSED — confirmed by a
    // one-off test script (silent for 8s at PROCESSED, first block arrived
    // immediately at CONFIRMED). blockLatency.ts's raw blocksMeta subscription
    // already uses CONFIRMED for the same reason.
    commitment: CommitmentLevel.CONFIRMED,
  };

  const counters = makeStageCounters(label);
  // Parsing for each block runs in the background (see file header comment);
  // this tracks in-flight block-parse work so we can drain it before printing
  // the final summary, rather than under- or over-counting a partially parsed
  // trailing block.
  const pendingParses = new Set<Promise<void>>();

  let ended = false;
  let stoppingByUs = false; // set right before we call stream.destroy() ourselves
  const finish = async (reason: string) => {
    if (ended) return;
    ended = true;
    console.log(`[${label}] ${reason}`);
    await Promise.all(pendingParses);
    counters.stop();
  };

  const streamClosed = new Promise<void>((resolve) => {
    stream.on("data", (data: any) => {
      const block = data?.block;
      if (!block) return; // ignore pings / other update kinds (blocksMeta, ping, etc.)

      // Printed the moment the block arrives, before parsing starts — slot is
      // Yellowstone's own block identifier; blockHeight (when present) is the
      // more conventional "block number" (slots can be skipped, block height
      // only increments when a block is actually produced).
      const txInfos: any[] = block.transactions ?? [];
      console.log(
        `[${label}] block received — slot=${block.slot} height=${block.blockHeight?.blockHeight ?? "n/a"} txs=${txInfos.length}`
      );

      for (const _tx of txInfos) counters.tickStreamed();
      if (!txInfos.length) return;

      // Parse this block's transactions in parallel (Promise.all, not sequential
      // awaits) without blocking this data handler — see file header comment for
      // what "parallel" does and doesn't mean here.
      const parseBlock = async () => {
        await Promise.all(
          txInfos.map(async (txInfo) => {
            try {
              parser.parseTransaction(parser.formatGrpcTransactionData({ transaction: txInfo, slot: block.slot }, Date.now()));
              counters.tickParsed();
            } catch (err) {
              console.error(`[${label}] parse error (slot ${block.slot}):`, err);
            }
          })
        );
      };

      const parsePromise = parseBlock();
      pendingParses.add(parsePromise);
      parsePromise.finally(() => pendingParses.delete(parsePromise));
    });
    stream.on("error", (err: any) => {
      if (stoppingByUs) return; // expected — we destroyed the stream ourselves
      console.error(`[${label}] Stream error:`, err);
    });
    stream.on("end", () => {
      finish("Stream ended.").then(resolve);
    });
    stream.on("close", () => resolve());
  });

  console.time(`[${label}] send subscribe request`);
  await new Promise<void>((resolve, reject) => {
    stream.write(request, (err: Error | null | undefined) => (err ? reject(err) : resolve()));
  });
  console.timeEnd(`[${label}] send subscribe request`);

  const durationNote = RUN_DURATION_MS > 0 ? ` for ${RUN_DURATION_MS}ms` : "";
  console.log(`[${label}] streaming blocks for ${addresses.length} address(es)${durationNote}: ${addresses.join(", ")}`);

  if (RUN_DURATION_MS <= 0) {
    // No duration set: run until the stream itself ends (Ctrl+C, fatal error, etc).
    await streamClosed;
    return;
  }

  // Race the stream's natural end against our duration timer, same pattern as
  // dualStream.ts: `stream` is a Node `Duplex` (@triton-one/yellowstone-grpc@5's
  // native-backed ClientDuplexStream, not a @grpc/grpc-js stream — no
  // `.cancel()` on it), so `.destroy()` is the correct teardown.
  await new Promise<void>((resolve) => {
    const timer = setTimeout(() => {
      stoppingByUs = true;
      stream.destroy();
      finish("duration elapsed — stopping stream.").then(resolve);
    }, RUN_DURATION_MS);
    timer.unref();

    streamClosed.then(() => {
      clearTimeout(timer);
      resolve();
    });
  });
}

async function main() {
  const enabled = STREAMS.filter((s) => s.addresses.length > 0);
  const disabled = STREAMS.filter((s) => s.addresses.length === 0);

  if (!enabled.length) {
    throw new Error(
      "No addresses configured — set at least one of STREAM_1_ADDRESSES / STREAM_2_ADDRESSES " +
        "(comma-separated list of addresses to stream)."
    );
  }
  for (const s of disabled) {
    console.log(`[${s.label}] disabled — no addresses in ${s.envVar}.`);
  }

  await Promise.all(enabled.map((s) => runStream(s.label, s.addresses)));
}

main()
  .then(() => process.exit(0)) // both streams stopped; force-exit in case a gRPC client leaves handles open
  .catch((err) => {
    console.error("Fatal error:", err);
    process.exit(1);
  });
