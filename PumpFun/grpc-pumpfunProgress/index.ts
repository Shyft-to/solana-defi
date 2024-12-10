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
import { Connection, PublicKey, VersionedTransactionResponse } from "@solana/web3.js";
import { tOutPut } from "./utils/transactionOutput";
import { publicKey } from "@solana/buffer-layout-utils";
import { getTokenBalance } from "./utils/token";
import { LIQUIDITY_STATE_LAYOUT_V4 } from "@raydium-io/raydium-sdk";
import { Boolean } from "@solana/buffer-layout";
import { getBondingCurveAddress } from "./utils/getBonding";
const pumpfun = '6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P';


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
    const result = await tOutPut(data);
    const bondingDetails = await getBondingCurveAddress(result.meta.postTokenBalances);
    const Ca = result.meta.postTokenBalances[0].mint
    const bondingCurve = bondingDetails.bondingCurve?bondingDetails.bondingCurve?.toString():"";
    const PoolValue = bondingDetails.solBalance/1000000000
    const poolStandard = 84;
    const progress = (PoolValue/poolStandard) * 100;
    console.log(`
      BONDING CURVE PROGRESS
      Ca : ${Ca}
      Bonding Curve Address : ${bondingCurve}
      Pool Value : ${Number(PoolValue).toFixed(2)} SOL
      PROGRESS : ${Number(progress).toFixed(1)}% to Completion
   `)
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
  'gRPC REGION URL',
  'gRPC TOKEN',
  undefined,
);
const req: SubscribeRequest = {
  accounts: {},
  slots: {},
  transactions: {
    pumpFun: {
      vote: false,
      failed: false,
      signature: undefined,
      accountInclude: [],//["Fhg5yuuNN7SSG6XrvQKnBmwe32uwvbua6GYKiLyfpump",'D2BXEnw5Ufsns8mfFvs3qaAAuH8gPQuE9bMFvqTSpump'],
      accountExclude: [],
      accountRequired: [pumpfun],
    },
  },
  transactionsStatus: {},
  entry: {},
  blocks: {},
  blocksMeta: {},
  accountsDataSlice: [],
  ping: undefined,
  commitment: CommitmentLevel.CONFIRMED,
};

subscribeCommand(client, req);
