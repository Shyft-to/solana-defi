<a id="readme-top"></a>

# Streaming Raydium CLMM transactions using Rabbitstream and parsing transactions

This project streams and parses Raydium CLMM transactions in real time using Rabbitstream. It decodes raw transaction data as it is received, extracts relevant information, and serializes it into structured formats for efficient processing. The architecture is optimized for scalability and high performance, enabling reliable real-time analytics and data-driven insights

```
Raydium CLMM Program Id: CAMMCzo5YL8w4VFF8KVHrK22GGUsp5VTaW7grrKgrWqK
```

![screenshot](assets/RaydiumCLMM_Screenshot.png?raw=true "Screenshot")

## Getting Started

1. **Clone the repository:**
   ```bash
   git clone https://github.com/Shyft-to/solana-defi.git
   cd Rabbitstream/Raydium/Typescript/stream_and_parse_raydium_clmm_transactions
   ```

2. **Install Dependencies:**

    ```bash
    # For example, if using npm
    npm i
    ```

3. **Run the script:**

    ```bash
    # To run the script
    npm run start
    ```

*Note: Please open `.env` and input your env details before running the script.*

## Related Links

Shyft gRPC Docs: [https://docs.shyft.to/solana-fast-grpc/grpc-docs]