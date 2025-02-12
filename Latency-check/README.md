
## Underatanding gRPC Latency

First thing first, before testing gRPC latency, make sure you follow the rule of thumb.
```
Your server should be in the same region as the gRPC you are streaming from.
```

In simple words, if you are connecting to `grpc.ams.shyft.to`, your server should also be in Amsterdam. Same applies for all other regions. Now that we are connecting from the same region, lets answer the elephant in the room.

"Why am I getting 1s or more latency latency with gRPC. Isn't that slow?"

### Reason
Solana doesn’t store milliseconds in blocktime, which leads to loss of precision. A transaction happening at `07:46:46:900` would have a blocktime of `07:46:46`. If you receive it at `07:46:47:200`, latency would come as `07:46:47:200 - 07:46:46:000 = 1.2 seconds.` While in reality its around 300ms. So its better to approximate by taking the middle of the second. Hence subtract 500ms from your latency calculations.

[A more detailed explanation about latency](https://docs.shyft.to/solana-fast-grpc/decoding-grpc-latency)   
[gRPC Docs](https://docs.shyft.to/solana-fast-grpc/grpc-docs)   
[gRPC Replits](https://replit.com/@shyft-to/)  
