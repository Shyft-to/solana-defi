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

import { bnLayoutFormatter } from "./utils/bn-layout-formatter";
import bs58 from 'bs58';
import { publicKey, seq, struct, u128, u64 } from './utils/decode_utils/marshmallow'

const RAYDIUM_PROGRAM_ID = '675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8';

// const accountCoder = new BorshAccountsCoder(program_idl);
const accountCoder = struct([
  u64('status'),
  u64('nonce'),
  u64('maxOrder'),
  u64('depth'),
  u64('baseDecimal'),
  u64('quoteDecimal'),
  u64('state'),
  u64('resetFlag'),
  u64('minSize'),
  u64('volMaxCutRatio'),
  u64('amountWaveRatio'),
  u64('baseLotSize'),
  u64('quoteLotSize'),
  u64('minPriceMultiplier'),
  u64('maxPriceMultiplier'),
  u64('systemDecimalValue'),
  u64('minSeparateNumerator'),
  u64('minSeparateDenominator'),
  u64('tradeFeeNumerator'),
  u64('tradeFeeDenominator'),
  u64('pnlNumerator'),
  u64('pnlDenominator'),
  u64('swapFeeNumerator'),
  u64('swapFeeDenominator'),
  u64('baseNeedTakePnl'),
  u64('quoteNeedTakePnl'),
  u64('quoteTotalPnl'),
  u64('baseTotalPnl'),
  u64('poolOpenTime'),
  u64('punishPcAmount'),
  u64('punishCoinAmount'),
  u64('orderbookToInitTime'),
  // u128('poolTotalDepositPc'),
  // u128('poolTotalDepositCoin'),
  u128('swapBaseInAmount'),
  u128('swapQuoteOutAmount'),
  u64('swapBase2QuoteFee'),
  u128('swapQuoteInAmount'),
  u128('swapBaseOutAmount'),
  u64('swapQuote2BaseFee'),
  // amm vault
  publicKey('baseVault'),
  publicKey('quoteVault'),
  // mint
  publicKey('baseMint'),
  publicKey('quoteMint'),
  publicKey('lpMint'),
  // market
  publicKey('openOrders'),
  publicKey('marketId'),
  publicKey('marketProgramId'),
  publicKey('targetOrders'),
  publicKey('withdrawQueue'),
  publicKey('lpVault'),
  publicKey('owner'),
  // true circulating supply without lock up
  u64('lpReserve'),
  seq(u64(), 3, 'padding'),
])

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
        
        const decodedData = accountCoder.decode(data.account.account.data);
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
    "owner": [RAYDIUM_PROGRAM_ID] // pumpfun program id to subscribe to
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
