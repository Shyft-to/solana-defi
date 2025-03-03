# Solana gRPC transaction based latency checker

This is a solana latency checker based on Solana transactions. Transactions are continuously streamed from gRPC, and are pushed to a queue (fastq). Another service (worker) reads from the queue, and call getTransaction RPC method for each transaction, and compare the blocktimes. For best results, it is recommended this coded runs on the same geolocation of the gRPC we are streaming from.

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

RPC_URL=https://rpc.ny.shyft.to?api_key=<your-api-key>
# your Solana RPC url

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