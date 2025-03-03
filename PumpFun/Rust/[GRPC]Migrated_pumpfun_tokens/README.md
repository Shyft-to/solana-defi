# Subscribe to Real-Time Updates for Migrated PumpFun Tokens Using gRPC

Gain a competitive edge by subscribing to real-time updates for tokens that complete their bonding curve and migrate from PumpFun to Raydium. Using gRPC, you can stream these critical updates directly into your application, ensuring you’re the first to know when a token transitions from its initial launch phase to a fully migrated state on Raydium.

This integration is particularly valuable for traders, developers, and analysts who need to stay informed about token migrations as they happen. By leveraging Rust for this implementation, you benefit from a high-performance, memory-safe, and efficient system capable of handling real-time data streams with ease.

Whether you're building automated trading strategies, monitoring market trends, or conducting in-depth analysis, this gRPC-based solution provides a reliable and scalable way to track token migrations and capitalize on emerging opportunities in the decentralized finance (DeFi) ecosystem.

```
 cargo run -- --endpoint https://grpc.ny.shyft.to --x-token<token>

--PS: this code only works for the program ID <address>
-- DON'T attempt to use the code for any other program
## Notes

gRPC client examples :https://github.com/Shyft-to/solana-defi
Blogs : blogs.shyft.to
Learn about shyft: https://shyft.to/
Discord: https://discord.gg/6bSmYuDa