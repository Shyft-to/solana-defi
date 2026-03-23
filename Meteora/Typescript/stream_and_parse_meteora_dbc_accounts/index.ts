import "dotenv/config";
import Client, {
    CommitmentLevel,
    SubscribeRequest
  } from "@triton-one/yellowstone-grpc";
import { meteoraDbcParsedAccount } from "./utils/meteora-dbc-parsed-account";
 
   
  const METEORA_DBC_PROGRAM_ID = 'dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN';
    console.log("Streaming Meteora Dbc accounts updates...");
    async function handleStream(client: Client, args: SubscribeRequest) {

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
        const parsed_account = meteoraDbcParsedAccount(data);
        if (!parsed_account)return;
        console.log(parsed_account);
       }catch(error){
       if(error){
        console.log(error);
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
        const stream = await handleStream(client, args);
  
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
    "METEORA_DBC": {
      "account": [],
      "filters": [],
      "owner": [METEORA_DBC_PROGRAM_ID] 
    }
  },
  "transactions": {},
  "blocks": {},
  "blocksMeta": {
    "block": []
  },
  "accountsDataSlice": [],
  "commitment": CommitmentLevel.PROCESSED, // Subscribe to processed blocks for the fastest updates
  entry: {},
  transactionsStatus: {}
}
  subscribeCommand(client, req);
  
