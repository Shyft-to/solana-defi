<a id="readme-top"></a>
# Stream Orca Whirlpool transactions and parse and detect buy and sell using program IDL

This project leverages gRPC to stream real-time Orca Whirlpool buy and sell transactions on the Solana blockchain. Each transaction is parsed and decoded using the program's Interface Definition Language (IDL), enabling seamless integration with various DeFi applications such as trading bots, DEX aggregators, analytics tools, and more. Ideal for developers and platforms looking to gain actionable insights from on-chain data or automate trading strategies on Solana.

```
Whirlpool Program Id: whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc
```

![screenshot](assets/orca-buy-sell.png?raw=true "How to stream data")

## Getting Started
```
$ cargo run -- --endpoint <endpoint> --x-token <token>
```

## Related Links

- Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]  
- Start Streaming with Shyft: [https://shyft.to/solana-yellowstone-grpc]  
- Shyft Website: [https://shyft.to/]

<p align="right">(<a href="#readme-top">back to top</a>)</p>