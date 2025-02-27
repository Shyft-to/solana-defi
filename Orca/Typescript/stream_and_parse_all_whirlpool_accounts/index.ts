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

import * as fs from 'fs';
import { BorshAccountsCoder } from "@project-serum/anchor";
import { bnLayoutFormatter } from "./utils/bn-layout-formatter";
import bs58 from 'bs58';

const WHIRLPOOL_PROGRAM_ID = 'whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc';

const program_idl = JSON.parse(fs.readFileSync('./Idl/whirlpool_idl.json', 'utf8'));

const accountCoder = new BorshAccountsCoder(program_idl);

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
  // Subscribe for events
const stream = await client.subscribe();

  // Create `error` / `end` handler
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

  // Handle updates
  stream.on("data", async (data) => {
    try{
      if(data?.account){
        
        const decodedData = accountCoder.decodeAny(data.account.account.data);
        if(!decodedData)
          return;

        bnLayoutFormatter(decodedData);

        const accountInfo = {
          pubkey: bs58.encode(data.account.account.pubkey),
          data: decodedData,
          owner: bs58.encode(data.account.account.owner),
          lamports: data.account.account.lamports,
          executable: data.account.account.executable,
          rentEpoch: data.account.account.rentEpoch,
          //slot: data.account.account.slot
        };
        
        console.log("Decoded Account Info for ", bs58.encode(data.account.account.pubkey));
        console.dir(accountInfo, {depth: null});
      }

}catch(error){
  if(error){
    console.log(error)
  }
}
});

  // Send subscribe request
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
    "owner": [WHIRLPOOL_PROGRAM_ID] // pumpfun program id to subscribe to
  }
},
"transactions": {},
"blocks": {},
"blocksMeta": {},
"accountsDataSlice": [],
"commitment": CommitmentLevel.PROCESSED, // Subscribe to processed blocks for the fastest updates
entry: {},
transactionsStatus: {}
}
subscribeCommand(client, req);
