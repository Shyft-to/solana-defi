import "dotenv/config";
import Client, {
  CommitmentLevel,
  SubscribeRequest
} from "@triton-one/yellowstone-grpc";
import {  PublicKey } from "@solana/web3.js";
import { parsedAccountData } from "./utils/accountStateParser";

const PUMP_SWAP_AMM_PROGRAM_ID = new PublicKey(
  "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA"
);

async function handleStream(client: Client, args: SubscribeRequest) {
  console.log("Stream Starting...")
  const stream = await client.subscribe();

  const streamClosed = new Promise<void>((resolve, reject) => {
    stream.on("error", (error) => {
      console.log("ERROR", error);
      reject(error);
      stream.end();
    });
    stream.on("end", () => {
      resolve();
    });
    stream.on("close", () => {
      resolve();
    });
  });

  stream.on("data", async (data) => {
    if(data.account) {
      try {
        const result = await parsedAccountData(data);
        console.log(result);
      } catch (error) {
        if (error) {
          console.log(error)
        }
      }
    }
    
  });

  await new Promise<void>((resolve, reject) => {
    stream.write(args, (err: any) => {
      if (err === null || err === undefined) {
        resolve();
      } else {
        reject(err);
      }
    });
  }).catch((reason) => {
    console.error(reason);
    throw reason;
  });

  await streamClosed;
}

async function subscribeCommand(client: Client, args: SubscribeRequest) {
  while (true) {
    try {
      await handleStream(client, args);
    } catch (error) {
      console.error("Stream error, restarting in 1 second...", error);
      await new Promise((resolve) => setTimeout(resolve, 1000));
    }
  }
}
const client = new Client(
  process.env.GRPC_URL,
  process.env.X_TOKEN,
  undefined,
);

const req: SubscribeRequest = {
  "slots": {},
  "accounts": {
    "pumpswap_amm": {
      "account": [],
      "filters": [],
      "owner": [PUMP_SWAP_AMM_PROGRAM_ID.toBase58()]
    }
  },
  "transactions": {},
  "blocks": {},
  "blocksMeta": {
    "block": []
  },
  "accountsDataSlice": [],
  "commitment": CommitmentLevel.PROCESSED,
  entry: {},
  transactionsStatus: {}
}
subscribeCommand(client, req);
