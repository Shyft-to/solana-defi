# Stream Pumpfun Transactions and Track Buy/Sell Events in Real-Time  
![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")


This project focuses on streaming Pump.fun transactions using gRPC and parsing buy/sell events in real-time. By decoding transaction data, it identifies key details such as the buyer/seller addresses and the transaction amounts. The implementation leverages Rust’s performance and type safety to efficiently process high-throughput transaction streams from Solana’s ecosystem. Structured for seamless integration, this solution extracts actionable insights from Pump.fun and Token Program instructions, enabling real-time analysis of market activity. Ideal for traders and developers, it provides a robust foundation for tracking and analyzing on-chain trading behavior on Pump.fun.


```
 cargo run -- --endpoint<gRpc endpoint url> --x-token<token>
````
## Notes
--PS: this code only works for the program ID <address>
-- DON'T attempt to use the code for any other program


gRPC client examples :https://github.com/Shyft-to/solana-defi
Blogs : blogs.shyft.to
Learn about shyft: https://shyft.to/
Discord: https://discord.gg/6bSmYuDa