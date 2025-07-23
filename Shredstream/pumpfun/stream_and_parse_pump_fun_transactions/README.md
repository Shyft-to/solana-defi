# Stream and Decode Pumpfun launchpad Instructions via Shredstream
This project enables real-time streaming of compiled instructions from PumpFun transactions using Shredstream. It focuses on extracting and decoding raw PumpSwap AMM instructions into structured, human-readable data — optimized for low-latency processing.

## Features
- 🔄 **Real-time Instruction Streaming**: Uses Shredstream to subscribe to and process the **compiled instructions** in Solana transactions.
- 🧩 **Instruction-Level Decoding**: Decodes PumpFun  instructions using the corresponding Interface Definition Language (IDL) for clarity and structure.
- 🔐 **Secure Configuration**: Easily configure credentials and endpoints.


```
$ cargo run -- --shredstream-uri <uri>   --x-token  <token>  --pubkey 6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P
```

![screenshot](assets/screenshot-usage.png?raw=true "Screenshot")

## Notes

Shredstreaming client example in rust: [https://github.com/Shyft-to/shredstream-decode-examples]