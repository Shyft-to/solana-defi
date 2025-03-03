# Solana gRPC slot based latency checker

This project focuses on real-time analysis of Solana transaction latency. It uses Shyft gRPC to simultaneously stream both transaction updates and block metadata. By correlating the transaction's "received time" with the corresponding block's "block time" from the block metadata stream (using the slot field), the project calculates the latency experienced by each transaction. This approach eliminates the need for separate RPC calls to fetch block times, improving efficiency and reducing latency in the analysis itself.

Improvements and suggestions are welcome.  

## Prerequisites: `.env` details
You can just rename the `.env-sample` file to `.env` and add in your values, or just create an `.env` file with the following values.

```
COMMITMENT_LEVEL=processed
# This can be confirmed or finalized as well

ENDPOINT=https://grpc.us.shyft.to
# It can be connected to any yellowstone gRPC endpoint

X_TOKEN=add-your-token
# your x-token, if shyft user, you can find this in grpc section

CONCURRENCY=100
# no of items to be processed at a time by fastq

PUBLIC_KEY_TO_LISTEN=675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8
# address of the program you are receiving txns for

RUN_TIME=2
# how long you want to run the test, in minutes
```  
   
## Running the code
After the `.env` file is ready, you can run the code using the following command (in the project directory), 

```
$ npm run start
```
OR,

```
$ npx ts-node index.ts
```


## Notes

For more details on Shyft gRPC, checkout [gRPC Docs](https://docs.shyft.to/solana-grpc-shredstream/grpc-docs)
 and for detailed examples, checkout [Shyft Blogs](https://blogs.shyft.to/)!