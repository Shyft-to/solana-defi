# Real-Time PumpFun Token Mint Monitoring with Shredstream

This project provides a high-performance, real-time pipeline for streaming and decoding newly minted token transactions on PumpFun, using Shredstream with Rust. It captures and parses compiled Solana instructions, transforming raw blockchain data into structured, human-readable output — ideal for low-latency analytics, bots, or monitoring tools.

## Features
- 🪙 **Newly Minted Token Detection** : Automatically detects and processes transactions involving newly minted tokens on PumpFun, enabling real-time insights into token launches.

- 🔄 **Live Instruction Streaming** : Leverages Shredstream to subscribe to the Solana blockchain and stream compiled transaction instructions in real time.

- 🧠 **Instruction-Level Decoding** : Decodes raw PumpFun instructions using the Interface Definition Language (IDL), translating low-level bytecode into human-readable structures.

- 🔐 **Secure and Configurable Setup** : Supports secure credential handling and easily configurable shredstream endpoints for flexible deployment.

```
$ cargo run -- --shredstream-uri <uri>   --x-token  <token>  --pubkey 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
```

![screenshot](assets/screenshot-usage.png?raw=true "Screenshot")

## Notes

Shredstreaming client example in rust: [https://github.com/Shyft-to/shredstream-decode-examples]