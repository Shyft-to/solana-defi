# Stream Newly Created Pools on Pumpswap Amm  via 
This project provides a high-performance, real-time pipeline for streaming and decoding newly created pools on Pumpswap Amm, using Shredstream with Rust. It captures and parses compiled Solana instructions, transforming raw blockchain data into structured, human-readable output — ideal for low-latency analytics, bots, or monitoring tools.

## Features
- 🪙 **Newly Created Pool Detection** : Automatically detects and processes transactions involving newly created pools on pumpswap Amm by listening to the `create_pool` instruction.
- 🔄 **Real-time Instruction Streaming**: Uses Shredstream to subscribe to and process the **compiled instructions** in Solana transactions.
- 🧩 **Instruction-Level Decoding**: Decodes Pumpswap Amm  instructions using the corresponding Interface Definition Language (IDL) for clarity and structure.
- 🔐 **Secure Configuration**: Easily configure credentials and endpoints.


```
$ cargo run -- --shredstream-uri <uri>   --x-token  <token>  --pubkey pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
```

![screenshot](assets/screenshot-usage.png?raw=true "Screenshot")

## Notes
Shredstreaming client example in rust: [https://github.com/Shyft-to/shredstream-decode-examples]
Shyft : [https://shyft.to]