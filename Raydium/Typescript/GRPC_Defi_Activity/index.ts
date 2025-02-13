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
import { VersionedTransactionResponse } from "@solana/web3.js";
import { TransactionFormatter } from "./utils/transaction-formatter";
import { RaydiumAmmParser } from "./utils/raydium-amm-parser";
import { getPools } from "./utils/getPool";

const MONITORED_TOKENS = "8i51XNNpGaKaj4G4nDdmQh95v4FKAxw8mhtaRoKd9tE8";
let MONITORED_POOLS: string[] = [];

async function updateMonitoredPools(address: string) {
   MONITORED_POOLS = await getPools(address);
}


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

const TXN_FORMATTER = new TransactionFormatter();
const RAYDIUM_PARSER = new RaydiumAmmParser();
const RAYDIUM_PUBLIC_KEY = RaydiumAmmParser.PROGRAM_ID;

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
    if (data?.transaction) {
      const txn = TXN_FORMATTER.formTransactionFromJson(
        data.transaction,
        Date.now(),
      );
        console.log(
          `
          Filter: ${data.filters}
          Signature: ${txn.transaction.signatures[0]}
           CA : ${MONITORED_TOKENS}
          `
        );
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
      raydiumTransactions: {
        vote: false,
        failed: false,
        accountInclude: ["675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"],
        accountRequired: [MONITORED_TOKENS],
        accountExclude : []
      },
      poolTransaction: {
        vote: false,
        failed: false,
        accountInclude: [MONITORED_TOKENS], 
        accountRequired: MONITORED_POOLS,
        accountExclude: []
      },
    },
    transactionsStatus: {},
    entry: {},
    blocks: {},
    blocksMeta: {},
    accountsDataSlice: [],
    commitment: CommitmentLevel.PROCESSED,
  };
  updateMonitoredPools(MONITORED_TOKENS).then(() => {
    //console.log(MONITORED_POOLS)
    subscribeCommand(client, req);
});
// subscribeCommand(client, req);

function decodeRaydiumTxn(tx: VersionedTransactionResponse) {
  if (tx.meta?.err) return;
  
  const allIxs = TXN_FORMATTER.flattenTransactionResponse(tx);

  const raydiumIxs = allIxs.filter((ix) =>
    ix.programId.equals(RAYDIUM_PUBLIC_KEY),
  );

  const decodedIxs = raydiumIxs.map((ix) =>
    RAYDIUM_PARSER.parseInstruction(ix),
  );

  return decodedIxs;
}
function getVaults(tx:VersionedTransactionResponse){
  const accountKeys = tx.meta.loadedAddresses.writable;
  const solVault = accountKeys[2];
  const tokenVault = accountKeys[1];
  return {solVault, tokenVault,accountKeys};
}
function getMintToken(tx){
  const data : any[] = tx.transaction.transaction.meta.preTokenBalances;
  const filter = data.filter((t)=> t.mint !== "So11111111111111111111111111111111111111112")
  const ca = filter[0].mint;
  const signer = filter[0].owner;
   return {
    ca,
    signer
  };
}
function stringifyWithBigInt(obj: any): string {
  return JSON.stringify(obj, (key, value) => 
    typeof value === 'bigint' ? value.toString() : value);
}
