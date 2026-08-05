import "dotenv/config";
import { LatencyChecker } from "@shyft-to/ladybug-sdk";

// Pump.fun program id (also the `address` field inside pump_fun.json)
const PUMP_FUN_PROGRAM_ID = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

const ENDPOINT = process.env.ENDPOINT!;
const X_TOKEN = process.env.X_TOKEN;
const TESTING_TIME_MS = process.env.TESTING_TIME_MS ? Number(process.env.TESTING_TIME_MS) : 30 * 1000;

if (!ENDPOINT) {
  throw new Error("Missing ENDPOINT env var (Yellowstone gRPC endpoint)");
}

// NOTE: deliberately NOT calling `latencyChecker.addParser(...)` here.
// In @shyft-to/ladybug-sdk@0.1.0, attaching a parser flips LatencyChecker into
// "parsing latency" mode, which has a bug in its signature-decoding fallback
// (node_modules/@shyft-to/ladybug-sdk/dist/index.js:917-921) — it reads
// `data.transaction.signature` where the bytes actually live one level deeper
// at `data.transaction.transaction.signature`, so it throws
// `TypeError: Expected Buffer` on every transaction. Skipping addParser keeps
// it on the (correctly wired) `transactionStatus` path instead.

async function main() {
  console.time("1. init LatencyChecker");
  const latencyChecker = new LatencyChecker(ENDPOINT, X_TOKEN);
  console.timeEnd("1. init LatencyChecker");

  console.time("2. addAddresses");
  await latencyChecker.addAddresses([PUMP_FUN_PROGRAM_ID]);
  console.timeEnd("2. addAddresses");

  console.time("3. setTestingTime");
  await latencyChecker.setTestingTime(TESTING_TIME_MS);
  console.timeEnd("3. setTestingTime");

  latencyChecker.onError((err) => console.error("Stream error:", err));
  latencyChecker.onEnd(() => console.log("Stream ended."));

  console.log(
    `Checking Pump.fun latency against ${ENDPOINT} for ${TESTING_TIME_MS}ms ...\n` +
      "(the SDK prints per-tx latency as data arrives, then a summary report when the test window ends)\n"
  );

  // start() blocks until `testingTime` elapses (or stop() is called), then
  // prints the aggregated latency distribution report automatically.
  console.time("4. latencyChecker.start (full test duration)");
  await latencyChecker.start();
  console.timeEnd("4. latencyChecker.start (full test duration)");
}

main().catch((err) => {
  console.error("Fatal error:", err);
  process.exit(1);
});
