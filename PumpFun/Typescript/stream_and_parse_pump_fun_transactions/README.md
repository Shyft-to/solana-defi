# Stream and Parse Pump.fun Transactions in Real-time

This project streams Pump.fun transactions from the Solana blockchain via gRPC and decodes them in real time using the Ladybug SDK parser powered by the Shyft.

By decoding instructions at the program level, this project provides real-time visibility into Pump.fun market activity, allowing users to track token creation, buys, sells, swaps, and other DeFi-related events. This is useful for understanding market trends, trader behavior, and protocol usage within the Pump.fun ecosystem.

```
Pump.fun Program ID: 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
```
![screenshot](assets/usage_screenshot.png?raw=true "How to run project")

## How It Works
* Transactions are streamed in real time using Solana gRPC
* Instructions are decoded using @shyft-to/ladybug-sdk
* The IDL can be fetched dynamically from on-chain or provided manually as a local JSON file

This design allows users to choose between automatic IDL updates or stable, pinned decoding.

## Getting Started

### 1. Pre-requisites

This project works with any Solana gRPC provider.

The .env file requires the following values:

* ENDPOINT – Your Solana gRPC endpoint
* X_TOKEN – Access token (leave blank if not required)

### 2. Clone the Repository

```
git clone https://github.com/Shyft-to/solana-defi.git
cd PumpFun/Typescript/stream_and_parse_pump_fun_transactions
```

### 3. Install Dependencies

```
npm i
```

### 4. Run the Script

```
npm run start

# or
npx ts-node index.ts
```
This will stream all Pump.fun transactions and decode Pump.fun and Token Program instructions.

Note: Please rename the .env.sample file to .env and input your environment details before running the script.

## IDL Handling

This project supports two methods for loading the Pump.fun IDL.

On-chain IDL fetching uses @coral-xyz/anchor to retrieve the IDL directly from the blockchain. This ensures the parser stays up to date with program upgrades but may be unavailable if the IDL account is closed.
Manual IDL loading uses a locally stored JSON file. This approach is more stable and recommended for production systems, indexers, bots, and historical backfilling.

You can switch between these two options directly in the code.

## Things to Note

* On-chain IDLs can be removed if a program closes its IDL account
* Local IDLs provide deterministic and stable decoding
* Parsing accuracy depends on the correctness of the IDL
* This project is read-only and does not submit transactions
* Production systems should implement reconnection logic for gRPC streams

## Related Links

* Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs](https://docs.shyft.to/solana-fast-grpc/grpc-docs)
* Start Streaming with Shyft: [https://shyft.to/solana-yellowstone-grpc](https://shyft.to/solana-yellowstone-grpc)
* Ladybug SDK: [https://github.com/Shyft-to/ladybug-sdk](https://github.com/Shyft-to/ladybug-sdk)
* Shyft Website: [https://shyft.to/](https://shyft.to/)