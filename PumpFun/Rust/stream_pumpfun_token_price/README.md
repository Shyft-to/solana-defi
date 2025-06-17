# Stream token price update on pumpfun

![screenshot](assets/usage-screenshot.png?raw=true "Screenshot")

This project streams real-time token prices from the Pump.fun using the formula `virtualSolReserves / virtualTokenReserves`. It allows users to monitor live price movements and market capitalization. By tracking token prices prior to migration, the project delivers valuable insights into market trends and emerging investment opportunities.


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