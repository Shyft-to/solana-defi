# Stream and Decode Pumpswap  Amm Instructions via Shredstream
This project enables real-time streaming of compiled instructions from PumpSwap AMM transactions using Shredstream. It focuses on extracting and decoding raw PumpSwap AMM instructions into structured, human-readable data — optimized for low-latency processing.

## Features
- 🔄 **Real-time Instruction Streaming**: Uses Shredstream to subscribe to and process the **compiled instructions** in Solana transactions.
- 🧩 **Instruction-Level Decoding**: Decodes Pumpswap Amm  instructions using the corresponding Interface Definition Language (IDL) for clarity and structure.
- 🔐 **Secure Configuration**: Easily configure credentials and endpoints.


```
$ cargo run -- --shredstream-uri <uri>   --x-token  <token>  --pubkey pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA
```

![screenshot](assets/screenshot-usage.png?raw=true "Screenshot")

## Notes

Shredstreaming client example in rust: [https://github.com/Shyft-to/shredstream-decode-examples]