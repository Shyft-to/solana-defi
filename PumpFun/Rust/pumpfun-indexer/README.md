# Building a Pumpfun Indexer with gRPC and PostgreSQL




This project subscribes to blocks, streams Pump.fun transactions in real-time, and parses both Pump.fun and Token Program instructions. All transaction data is then written to a PostgreSQL database. The goal is to demonstrate how to build a custom Pumpfun indexer using gRPC and PostgreSQL efficiently.



run with:
cargo run -- --endpoint https://grpc.ny.shyft.to --x-token<gRpc Token>



--PS: this code only works for Pumpfun Program
-- DON'T attempt to use the code for any other program
## Notes
 
  

gRPC client examples :https://github.com/Shyft-to/solana-defi
Blogs : blogs.shyft.to
Learn about shyft: https://shyft.to/
Discord: https://discord.gg/6bSmYuDa

