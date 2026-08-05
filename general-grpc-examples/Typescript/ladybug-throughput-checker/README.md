# check-lady-bug-sdk-latency

Scratch/test scripts for exercising [`@shyft-to/ladybug-sdk`](https://www.npmjs.com/package/@shyft-to/ladybug-sdk) against a Yellowstone gRPC endpoint — streaming Solana transactions, measuring throughput (tx/sec), and measuring latency (time from on-chain block to received-in-process).

```
Note: This is a test project for running various checks with ladybug SDK
```

## Setup

```bash
npm install
cp .env.example .env   # then fill in your own values
```

### Environment variables

| Variable              | Used by                                   | Description                                                                                          |
| ---------------------- | ------------------------------------------ | ------------------------------------------------------------------------------------------------------ |
| `ENDPOINT`             | all scripts                               | Yellowstone gRPC endpoint URL. **Required.**                                                          |
| `X_TOKEN`               | all scripts                               | Yellowstone gRPC auth token.                                                                            |
| `STREAM_1_ADDRESSES`   | `index.ts`                                | Comma-separated addresses stream 1 subscribes to. Leave blank/unset to disable stream 1.               |
| `STREAM_2_ADDRESSES`   | `index.ts`                                | Comma-separated addresses stream 2 subscribes to. Leave blank/unset to disable stream 2.               |
| `RUN_DURATION_MS`      | `index.ts`, `streamAll.ts`                | How long to stream before auto-stopping and printing the summary. Default `30000`. `0`/blank = run until Ctrl+C. |
| `TESTING_TIME_MS`      | `latencyChecker.ts`                       | How long the SDK's built-in latency test runs before printing its report. Default `30000`.              |

`.env` holds real secrets and is git-ignored — never commit it. Use `.env.example` as the template.

## Files

| File | What it does |
| ---- | ------------- |
| **`index.ts`** | Runs two independent `TransactionStreamer` streams in parallel (`STREAM_1_ADDRESSES` / `STREAM_2_ADDRESSES`), each with its own `Parser` loaded with all five IDLs below. Prints tx/sec once a second per stream, and on stop (after `RUN_DURATION_MS`, or Ctrl+C) prints a summary: total tx, average tx/sec, and peak tx/sec. A stream with no addresses configured is skipped rather than erroring. |
| **`streamAll.ts`** | Single unfiltered `TransactionStreamer` — no `Parser`, no address filter — subscribed to *every* non-vote, non-failed transaction on the network (an empty `accountInclude` filter matches everything, per Yellowstone gRPC semantics). Same tx/sec + avg/peak summary output as `index.ts`. Useful as a global throughput baseline since it isn't scoped to specific programs. |
| **`latencyChecker.ts`** | Uses the SDK's built-in `LatencyChecker` (not `TransactionStreamer`) to measure per-transaction latency (block time vs. received time) for Pump.fun, printing a distribution report after `TESTING_TIME_MS`. Runs in `transactionStatus` mode — see the code comment for why `addParser` isn't attached. |
| **`blockLatency.ts`** | A more manual latency measurement: a `TransactionStreamer` (with parser) records `{ slot -> [{ signature, receivedTime }] }` as Pump.fun transactions arrive, while a second, independent raw `@triton-one/yellowstone-grpc` connection subscribes only to `blocksMeta` (slot + blockTime). When a slot's blockMeta arrives, it's matched against any buffered transactions for that slot to compute `latency = receivedTime - blockTime`. |
| **`pump_fun.json`**, **`pump_amm_idl.json`**, **`raydium_clmm.json`**, **`whirlpool_idl.json`**, **`lbu_idl.json`** | Anchor IDLs for Pump.fun, Pump AMM, Raydium CLMM, Orca Whirlpool, and Meteora DLMM respectively. Loaded into each `Parser` via `addIDL` so it can decode those programs' instructions/events. |
| **`.env.example`** | Template for the environment variables above — copy to `.env` and fill in. |
| **`tsconfig.json`** / **`package.json`** | Standard TypeScript/Node project config. |

## Running

```bash
npm start                    # index.ts — dual-stream tx/sec
npm run start:stream-all     # streamAll.ts — unfiltered network-wide tx/sec
npm run start:latency-checker # latencyChecker.ts — SDK's latency distribution report
npm run start:block-latency  # blockLatency.ts — manual block-time-vs-received-time latency
```

## Known SDK issue

`TransactionStreamer.stop()` in `@shyft-to/ladybug-sdk@0.1.1` calls `this.stream?.cancel()` internally, but the underlying gRPC stream object doesn't expose a `.cancel` method — so `stop()` throws instead of tearing the connection down, and `onEnd` never fires as a result of calling it. `index.ts` and `streamAll.ts` work around this: `stop()` is called inside a try/catch (safe to ignore the throw), the run is finished and its summary printed on our own timer regardless of whether the SDK's teardown succeeded, and the process is force-exited (`process.exit(0)`) afterward to actually close the leftover connection.
