import "dotenv/config";
import Client, {
  CommitmentLevel,
  SubscribeRequestAccountsDataSlice,
  SubscribeRequestFilterAccounts,
  SubscribeRequestFilterBlocks,
  SubscribeRequestFilterBlocksMeta,
  SubscribeRequestFilterEntry,
  SubscribeRequestFilterSlots,
  SubscribeRequestFilterTransactions,
} from "@triton-one/yellowstone-grpc";
import { SubscribeRequestPing } from "@triton-one/yellowstone-grpc/dist/grpc/geyser";
import { decodeRaydiumLaunchpadTxnData } from "./utils/raydium-launchpad-transaction-processor";

const RAYDIUM_LAUNCHPAD_PROGRAM_ID = 'LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj';

interface SubscribeRequest {
  accounts: { [key: string]: SubscribeRequestFilterAccounts };
  slots: { [key: string]: SubscribeRequestFilterSlots };
  transactions: { [key: string]: SubscribeRequestFilterTransactions };
  transactionsStatus: { [key: string]: SubscribeRequestFilterTransactions };
  blocks: { [key: string]: SubscribeRequestFilterBlocks };
  blocksMeta: { [key: string]: SubscribeRequestFilterBlocksMeta };
  entry: { [key: string]: SubscribeRequestFilterEntry };
  commitment?: CommitmentLevel | undefined;
  accountsDataSlice: SubscribeRequestAccountsDataSlice[];
  ping?: SubscribeRequestPing | undefined;
}

async function handleStream(client: Client, args: SubscribeRequest) {
  console.log("Starting Stream...")
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
      try{
        const parsed_launchpad_account =await decodeRaydiumLaunchpadTxnData(data);
        if (!parsed_launchpad_account)return;
        console.log(parsed_launchpad_account);
       }catch(error){
       if(error){
        console.log(error);
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
  "pumpfun": {
    "account": [],
    "filters": [],
    "owner": [RAYDIUM_LAUNCHPAD_PROGRAM_ID] 
  }
},
"transactions": {},
"blocks": {},
"blocksMeta": {},
"accountsDataSlice": [],
"commitment": CommitmentLevel.PROCESSED, 
entry: {},
transactionsStatus: {}
}

subscribeCommand(client, req);
