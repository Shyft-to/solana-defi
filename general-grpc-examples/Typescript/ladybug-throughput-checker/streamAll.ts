import "dotenv/config";
import { TransactionStreamer } from "@shyft-to/ladybug-sdk";

const ENDPOINT = process.env.ENDPOINT!;
const X_TOKEN = process.env.X_TOKEN;
// How long to stream before stopping automatically. Set to 0/blank to run forever (Ctrl+C to stop).
const RUN_DURATION_MS = process.env.RUN_DURATION_MS ? Number(process.env.RUN_DURATION_MS) : 30_000;

if (!ENDPOINT) {
  throw new Error("Missing ENDPOINT env var (Yellowstone gRPC endpoint)");
}

// Counts transactions, prints a tx/sec figure once a second, and tracks enough
// (total count, peak window, start time) to report avg/peak tx/sec when stopped.
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

async function main() {
  const label = "all-tx";

  console.time(`[${label}] init streamer`);
  const streamer = new TransactionStreamer(ENDPOINT, X_TOKEN);
  console.timeEnd(`[${label}] init streamer`);

  // Deliberately no addParser() call: with no addresses to filter on, we're just counting
  // arrivals, not decoding instructions, so no IDL/Parser is needed. An empty `accountInclude`
  // filter (what addAddresses([]) produces on the wire) matches every non-vote, non-failed
  // transaction on the network instead of only ones touching specific programs.
  console.time(`[${label}] addAddresses ([] = no filter = all transactions)`);
  await streamer.addAddresses([]);
  console.timeEnd(`[${label}] addAddresses ([] = no filter = all transactions)`);

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
  console.log(`[${label}] streaming ALL Solana transactions${durationNote}...`);

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
  // the summary) on the timer regardless, and force-exiting the process afterward to actually
  // close the leftover connection.
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

main()
  .then(() => process.exit(0)) // force-exit in case the gRPC client leaves handles open
  .catch((err) => {
    console.error("Fatal error:", err);
    process.exit(1);
  });
