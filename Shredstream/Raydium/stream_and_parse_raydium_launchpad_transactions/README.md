# Stream and Decode Raydium Launchpad Instructions via Shredstream

This project enables real-time streaming of Raydium Launchpad transaction **compiled instructions** using [Shredstream](https://github.com/Shyft-to/shredstream-decode-examples). It is focused on extracting and decoding Raydium Launchpad raw compiled instruction, providing structured and readable data optimized for low-latency.

## Features
- 🔄 **Real-time Instruction Streaming**: Uses Shredstream to subscribe to and process the **compiled instructions** in Solana transactions.
- 🧩 **Instruction-Level Decoding**: Decodes Raydium Launchpad instructions using the corresponding Interface Definition Language (IDL) for clarity and structure.
- 🔐 **Secure Configuration**: Easily configure credentials and endpoints.


```
$ cargo run -- --shredstream-uri <uri>   --x-token  <token>  --pubkey LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj
```

![screenshot](assets/screenshot-example.png?raw=true "Screenshot")

## Notes

Shredstreaming client example in rust: [https://github.com/Shyft-to/shredstream-decode-examples]