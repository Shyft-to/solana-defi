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
| `STREAM_1_ADDRESSES`   | `index.ts`, `dualStream.ts`, `blocksParser.ts` | Comma-separated addresses stream 1 subscribes to. Leave blank/unset to disable stream 1.               |
| `STREAM_2_ADDRESSES`   | `index.ts`, `dualStream.ts`, `blocksParser.ts` | Comma-separated addresses stream 2 subscribes to. Leave blank/unset to disable stream 2.               |
| `RUN_DURATION_MS`      | `index.ts`, `streamAll.ts`, `dualStream.ts`, `blocksParser.ts` | How long to stream before auto-stopping and printing the summary. Default `30000`. `0`/blank = run until Ctrl+C. |
| `TESTING_TIME_MS`      | `latencyChecker.ts`                       | How long the SDK's built-in latency test runs before printing its report. Default `30000`.              |

`.env` holds real secrets and is git-ignored — never commit it. Use `.env.example` as the template.

## Files

| File | What it does |
| ---- | ------------- |
| **`index.ts`** | Runs two independent `TransactionStreamer` streams in parallel (`STREAM_1_ADDRESSES` / `STREAM_2_ADDRESSES`), each with its own `Parser` loaded with all five IDLs below. Prints tx/sec once a second per stream, and on stop (after `RUN_DURATION_MS`, or Ctrl+C) prints a summary: total tx, average tx/sec, and peak tx/sec. A stream with no addresses configured is skipped rather than erroring. |
| **`streamAll.ts`** | Single unfiltered `TransactionStreamer` — no `Parser`, no address filter — subscribed to *every* non-vote, non-failed transaction on the network (an empty `accountInclude` filter matches everything, per Yellowstone gRPC semantics). Same tx/sec + avg/peak summary output as `index.ts`. Useful as a global throughput baseline since it isn't scoped to specific programs. |
| **`dualStream.ts`** | Same dual-stream tx/sec measurement as `index.ts` (`STREAM_1_ADDRESSES` / `STREAM_2_ADDRESSES`), but without the ladybug SDK's `TransactionStreamer` — each stream opens its own raw `@triton-one/yellowstone-grpc` `Client`/`subscribe()` connection directly (same pattern `blockLatency.ts` uses for its blocksMeta stream). Each stream's `Parser` also skips per-program IDLs, relying only on `useDefaultInstructionParsing(true)` (System/Token/Token2022 decoding). `stream.destroy()` (not `.cancel()` — see "Known SDK issue" below) tears the connection down cleanly on `RUN_DURATION_MS`, no SDK workaround needed. |
| **`blocksParser.ts`** | Same raw-client dual-stream setup as `dualStream.ts`, but subscribed to `blocks` instead of `transactions` (`STREAM_1_ADDRESSES` / `STREAM_2_ADDRESSES` scope each stream's blocks filter via `accountInclude`, with `includeTransactions: true`), at `CommitmentLevel.CONFIRMED` — this endpoint doesn't emit `blocks` updates at `PROCESSED` (same reason `blockLatency.ts`'s raw blocksMeta subscription uses `CONFIRMED`). Prints `slot`/`blockHeight`/tx count the instant each block is received, before any parsing starts. Every transaction in the block is then parsed through the ladybug SDK `Parser` (same default-instruction-parsing-only setup as `dualStream.ts`), with each block's transactions parsed via `Promise.all` rather than sequential awaits, and block parsing kicked off without blocking the stream's `data` handler so one block's parsing can't stall the next block's arrival. Prints two tx/sec figures per stream each second — **streamed** (transactions as they arrive in a block) and **parsed** (transactions the `Parser` has finished decoding) — plus both summaries on stop. Note: a block filter's `accountInclude` gates the *whole block*, not individual transactions — a matching block arrives with all of its transactions attached, not just the one(s) touching the filtered address. |
| **`latencyChecker.ts`** | Uses the SDK's built-in `LatencyChecker` (not `TransactionStreamer`) to measure per-transaction latency (block time vs. received time) for Pump.fun, printing a distribution report after `TESTING_TIME_MS`. Runs in `transactionStatus` mode — see the code comment for why `addParser` isn't attached. |
| **`blockLatency.ts`** | A more manual latency measurement: a `TransactionStreamer` (with parser) records `{ slot -> [{ signature, receivedTime }] }` as Pump.fun transactions arrive, while a second, independent raw `@triton-one/yellowstone-grpc` connection subscribes only to `blocksMeta` (slot + blockTime). When a slot's blockMeta arrives, it's matched against any buffered transactions for that slot to compute `latency = receivedTime - blockTime`. |
| **`pump_fun.json`**, **`pump_amm_idl.json`**, **`raydium_clmm.json`**, **`whirlpool_idl.json`**, **`lbu_idl.json`** | Anchor IDLs for Pump.fun, Pump AMM, Raydium CLMM, Orca Whirlpool, and Meteora DLMM respectively. Loaded into each `Parser` via `addIDL` so it can decode those programs' instructions/events. |
| **`.env.example`** | Template for the environment variables above — copy to `.env` and fill in. |
| **`tsconfig.json`** / **`package.json`** | Standard TypeScript/Node project config. |

## Running

```bash
npm start                    # index.ts — dual-stream tx/sec
npm run start:stream-all     # streamAll.ts — unfiltered network-wide tx/sec
npm run start:dual-stream    # dualStream.ts — dual-stream tx/sec via raw yellowstone-grpc client
npm run start:blocks-parser  # blocksParser.ts — dual-stream blocks, parsed tx/sec vs streamed tx/sec
npm run start:latency-checker # latencyChecker.ts — SDK's latency distribution report
npm run start:block-latency  # blockLatency.ts — manual block-time-vs-received-time latency
```

## Known SDK issue

`TransactionStreamer.stop()` in `@shyft-to/ladybug-sdk@0.1.1` calls `this.stream?.cancel()` internally, but the underlying gRPC stream object doesn't expose a `.cancel` method — so `stop()` throws instead of tearing the connection down, and `onEnd` never fires as a result of calling it. `index.ts` and `streamAll.ts` work around this: `stop()` is called inside a try/catch (safe to ignore the throw), the run is finished and its summary printed on our own timer regardless of whether the SDK's teardown succeeded, and the process is force-exited (`process.exit(0)`) afterward to actually close the leftover connection.
