# Yellowstone gRPC Throughput Comparator

This tool benchmarks and compares the **throughput** (messages per second) and **latency** (delay in receiving data) between multiple Yellowstone gRPC endpoints on Solana.  

It helps operators and developers identify which endpoint streams transactions/blocks faster, making it useful for trading bots, indexers, or validator infrastructure tuning.

---

## 🔧 Features
- Connects to **two or more Yellowstone gRPC endpoints** simultaneously.  
- Subscribes to transaction and block streams.  
- Measures:
  - **Messages per second (msg/s)** per endpoint.  
  - **Latency (ms)** between endpoints.  
- Prints a live comparison in the terminal.  
- Reports aggregate results at the end.  

---

## 📦 Setup

### Prerequisites
- [Rust](https://www.rust-lang.org/tools/install) (latest stable recommended)
- Access to at least two Yellowstone gRPC endpoints (with tokens if required)

### Clone & Build
```bash
git https://github.com/Shyft-to/solana-defi.git
cd solana-defi/grpc-latency-checker/Rust/grpc-throughput-compare
cargo build
```

## 🚀 Run
```bash
cargo run -- --endpoint-a 'https://grpc.va.shyft.to' --endpoint-b 'https://grpc.ny.shyft.to' --x-token-a 'X_TOKEN_B' --x-token-b 'X_TOKEN_A' --timeout-dur 60
```

or

```bash
cargo run -- --endpoint-a 'https://grpc.ny.shyft.to' --x-token-a 'X_TOKEN_A' --timeout-dur 10
```
---

## 📊 Example Output

```bash
Streaming from endpointA (https://grpc.va.shyft.to)...
Streaming from endpointB (https://grpc.ny.shyft.to)...

https://grpc.va.shyft.to: 53 msg/s (lat 66 ms)
https://grpc.va.shyft.to: 284 msg/s (lat 716 ms) | https://grpc.ny.shyft.to: 13 msg/s (lat 204 ms)
https://grpc.va.shyft.to: 321 msg/s (lat 1257 ms) | https://grpc.ny.shyft.to: 21 msg/s (lat 1377 ms)
https://grpc.va.shyft.to: 325 msg/s (lat 1884 ms) | https://grpc.ny.shyft.to: 39 msg/s (lat 2515 ms)
https://grpc.va.shyft.to: 708 msg/s (lat 2102 ms) | https://grpc.ny.shyft.to: 28 msg/s (lat 3526 ms)
...
```

## 📈 Use Cases

- Benchmark different gRPCs.
- Compare latency between geographically distributed endpoints.
- Validate throughput for Solana trading bots, MEV bots, and indexers.

---