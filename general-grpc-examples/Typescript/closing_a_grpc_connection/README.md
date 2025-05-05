# Closing a gRPC Connection: Yellowstone Client

Once the subscription stream is active, we cancel it gracefully after a set duration using stream.cancel(). This tells the server that the client is intentionally ending the stream.

When stream.cancel() is called:

The stream emits an error event with a message like "Cancelled" or error code 1.

We listen for this event and check if it’s a user-initiated cancellation.

If so, we treat it as a normal shutdown, not an error.

We also listen for the close event to confirm that the stream has fully closed on the client side.

This clean cancellation helps release server-side resources and ensures that any connection tracking (like tokens or sessions) can be properly cleaned up in middleware or proxy layers.

## Getting Started
Clone the repository:

```bash
git clone https://github.com/Shyft-to/solana-defi.git
cd general-grpc-examples/Typescript/closing_a_grpc_connection
```

## Install Dependencies:

```
# For example, if using npm

npm i
```
## Run the script:

```
# To run the script
npx ts-node index.ts
```

Note: Please add your grpc endpoint and xtoken in the variables defined in the start of the file.