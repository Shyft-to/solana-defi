# Solana gRPC vs WebSocket Latency Benchmark Tool

This tool allows you to compare stream data from a gRPC endpoint and a WebSocket in order to benchmark latency.

## Features

Measures and compares transaction delivery times between gRPC and WebSocket streams.
Provides total transaction counts, per-node received transactions, and average latency gains.
Easy-to-run CLI with configurable duration.

## Usage
```bash
cargo run -- \
  --endpoint https://grpc.sgp.shyft.to \
  --x-token <auth_token> \
  --ws-uri 'wss://rpc.sgp.shyft.to?api_key=<api_key>' \
  --account-include pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA \
  --duration 60
```

## Options

- --endpoint : gRPC endpoint URL (required)
- --x-token : Authorization token for gRPC (required)
- --ws-uri : WebSocket URL (required)
- --account-include : Only stream transactions for this account (required)
- --duration : Test duration in seconds (optional, default: 60)

## Sample Output
```
Final results:
----------  Total Transactions: 223 --------
GRPC, count: 211 (faster in 94.62% cases), avg_gain: 243 ms
WebSocket, count: 12 (faster in 5.38% cases), avg_gain: 18 ms

--- Per-node total received transactions ---
WebSocket received 223 transactions
GRPC received 265 transactions
```

## Notes

The tool reports which stream delivers transactions faster and by how much, on average.
Ideal for latency benchmarking and performance monitoring between real-time streaming endpoints.