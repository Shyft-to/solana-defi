# Stream and Decode Raydium Launchpad Newly Minted Token via Shredstream

This project enables real-time streaming and decoding of newly minted tokens on the Raydium Launchpad by leveraging Shredstream. It focuses on extracting and interpreting compiled instructions specifically `initialize` from raw on-chain data, specifically targeting Raydium Launchpad transactions. The goal is to convert low-level blockchain data into structured, human-readable formats—optimized for low-latency applications such as analytics, token discovery, or automated monitoring systems.

## Features
-🔄 Real-time Instruction Streaming: Uses Shredstream to subscribe to and process the compiled instructions in Solana transactions.

-🧩 Instruction-Level Decoding: Decodes Raydium Launchpad instructions using the corresponding Interface Definition Language (IDL) for clarity and structure.

-🪙 Newly Minted Token Detection: Identifies and extracts metadata for newly minted tokens during Raydium Launchpad initialize instructions in real time.

- 🔐 Secure Configuration: Easily configure credentials and endpoints.

```
$ cargo run -- --shredstream-uri <uri>   --x-token  <token>  --pubkey LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj
```

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

## Notes

Shredstreaming client example in rust: [https://github.com/Shyft-to/shredstream-decode-examples]

