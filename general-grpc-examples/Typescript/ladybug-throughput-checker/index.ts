import "dotenv/config";
import { PublicKey } from "@solana/web3.js";
import { Idl } from "@coral-xyz/anchor";
import { Parser, TransactionStreamer } from "@shyft-to/ladybug-sdk";

import pumpFunIdl from "./pump_fun.json";
import pumpAmmIdl from "./pump_amm_idl.json";
import raydiumClmmIdl from "./raydium_clmm.json";
import whirlpoolIdl from "./whirlpool_idl.json";
import lbClmmIdl from "./lbu_idl.json";

// Program id + IDL pairs to teach every parser how to decode instructions/events.
// (programId is also the `address` field inside each *.json file.)
const IDLS: { programId: string; idl: Idl }[] = [
  { programId: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P", idl: pumpFunIdl as Idl }, // pump
  { programId: "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA", idl: pumpAmmIdl as Idl }, // pump_amm
  { programId: "CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK", idl: raydiumClmmIdl as Idl }, // raydium_clmm
  { programId: "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc", idl: whirlpoolIdl as Idl }, // whirlpool
  { programId: "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo", idl: lbClmmIdl as Idl }, // lb_clmm
];

const ENDPOINT = process.env.ENDPOINT!;
const X_TOKEN = process.env.X_TOKEN;
// How long each stream runs before it's stopped automatically. Set to 0/blank to run forever
// (until Ctrl+C), same as before.
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

// Each stream gets its own Parser instance (a Parser is stateful/single-owner once
// handed to a TransactionStreamer via addParser), but both are loaded with the same IDLs.
function buildParser(): Parser {
  const parser = new Parser();
  // Logging is enabled by default and prints every instruction the parser fails to
  // decode against the loaded IDLs (e.g. instructions from programs we didn't add an
  // IDL for) — noisy and irrelevant to a TPS measurement, so silence it.
  parser.enableLogging(false);
  for (const { programId, idl } of IDLS) {
    parser.addIDL(new PublicKey(programId), idl);
  }
  return parser;
}

// Counts transactions for one stream, prints a tx/sec figure once a second, and tracks
// enough (total count, peak window, start time) to report avg/peak tx/sec when stopped.
function makeTpsCounter(label: string) {
  let windowCount = 0;
  let totalCount = 0;
  let peakCount = 0;
  const startedAt = Date.now();

  const interval = setInterval(() => {
    if (windowCount > peakCount) peakCount = windowCount;
    console.log(`[${label}] ${windowCount} tx/sec (total: ${totalCount})`);
    windowCount = 0;
  }, 1000);
  interval.unref();

  return {
    tick: () => {
      windowCount++;
      totalCount++;
    },
    stop: () => {
      clearInterval(interval);
      if (windowCount > peakCount) peakCount = windowCount; // account for the trailing partial second
      const elapsedSec = (Date.now() - startedAt) / 1000;
      const avgTps = elapsedSec > 0 ? totalCount / elapsedSec : 0;
      console.log(
        `[${label}] summary — total: ${totalCount} tx over ${elapsedSec.toFixed(1)}s, ` +
          `avg: ${avgTps.toFixed(2)} tx/sec, peak: ${peakCount} tx/sec`
      );
    },
  };
}

async function runStream(label: string, addresses: string[]): Promise<void> {
  console.time(`[${label}] init parser + addIDLs`);
  const parser = buildParser();
  console.timeEnd(`[${label}] init parser + addIDLs`);

  console.time(`[${label}] init streamer`);
  const streamer = new TransactionStreamer(ENDPOINT, X_TOKEN);
  console.timeEnd(`[${label}] init streamer`);

  console.time(`[${label}] addParser`);
  await streamer.addParser(parser);
  console.timeEnd(`[${label}] addParser`);

  console.time(`[${label}] addAddresses`);
  await streamer.addAddresses(addresses);
  console.timeEnd(`[${label}] addAddresses`);

  const tps = makeTpsCounter(label);
  let ended = false;
  const finish = (reason: string) => {
    if (ended) return;
    ended = true;
    console.log(`[${label}] ${reason}`);
    tps.stop();
  };

  streamer.onData(() => tps.tick());
  streamer.onError((err) => console.error(`[${label}] Stream error:`, err));
  streamer.onEnd(() => finish("Stream ended."));

  const durationNote = RUN_DURATION_MS > 0 ? ` for ${RUN_DURATION_MS}ms` : "";
  console.log(`[${label}] streaming ${addresses.length} address(es)${durationNote}: ${addresses.join(", ")}`);

  // Not awaited directly — see below for why.
  const streamPromise = streamer.start().catch((err) => {
    console.error(`[${label}] start() error:`, err);
  });

  if (RUN_DURATION_MS <= 0) {
    // No duration set: run until the stream itself ends (Ctrl+C, fatal error, etc).
    await streamPromise;
    return;
  }

  // Race the stream's natural end against our duration timer. We deliberately don't just
  // `await streamer.start()` after calling stop() below, because TransactionStreamer.stop()
  // in @shyft-to/ladybug-sdk@0.1.1 has a bug — it calls `this.stream?.cancel()`, but the
  // underlying gRPC stream object has no `.cancel` method, so stop() throws instead of
  // tearing the stream down, and onEnd never fires. We work around it by finishing (printing
  // the summary) on the timer regardless, and force-exiting the process in main() afterward
  // to actually close the leftover connection.
  await new Promise<void>((resolve) => {
    const timer = setTimeout(() => {
      try {
        streamer.stop();
      } catch (err) {
        // Known SDK bug (see comment above) — safe to ignore, we finish manually below.
      }
      finish("duration elapsed — stopping stream.");
      resolve();
    }, RUN_DURATION_MS);
    timer.unref();

    streamPromise.then(() => {
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
  .then(() => process.exit(0)) // both streams stopped; force-exit in case the gRPC client leaves handles open
  .catch((err) => {
    console.error("Fatal error:", err);
    process.exit(1);
  });
