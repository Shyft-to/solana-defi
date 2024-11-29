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
import {  getSolBalance, getTokenBalance } from "./utils/walletInfo";
import { getMarketInfo } from "./utils/marketInfo";
import { getTokenInfo } from "./utils/tokenInfo";

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
      const decodedRaydiumIxs = decodeRaydiumTxn(txn);
      

      if (!decodedRaydiumIxs?.length) return;
      const createPoolIx = decodedRaydiumIxs.find((decodedRaydiumIx) => {
        if (
          decodedRaydiumIx.name === "swapIn" ||
          decodedRaydiumIx.name === "swapOut"
        ) {
          return decodedRaydiumIx;
        }
      });
       if (createPoolIx) {
        const info  = getMintToken(data);
        const accountKeys = getVaults(txn);
        const quoteVault = accountKeys.solVault;
        const baseVault = accountKeys.tokenVault;
        const tokenInfo = await getTokenInfo(info.ca)
        const quoteBal = await getSolBalance(quoteVault);
        const baseBal = await getTokenBalance(baseVault)/ 10 ** tokenInfo.decimal;
         const marketInfo = await getMarketInfo(baseBal,quoteBal,tokenInfo.currentSupply)
         const quoteBal$ = marketInfo.quote$ //sol = $241, you can change it in ...getMarketInfo()
         const price = marketInfo.price;
         const marketcap = marketInfo.marketcap;
         const supply = marketInfo.currentSupply;
        const stringify : any = stringifyWithBigInt(createPoolIx.args);
        console.log(
          `Signature: ${txn.transaction.signatures[0]}
           CA : ${info.ca}
           Price : $${price}
           MarketCap : ${marketcap}
           Supply : ${supply}
           Pool Info : ${stringify}
           Owner : ${info.signer}
          `
        );
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
    raydiumLiquidityPoolV4: {
      vote: false,
      failed: false,
      signature: undefined,
      accountInclude: ['bv88GZQfPHeTXHctPPxkbFCAKBCocXFSENYGYBppump','3FhZjjCn8KfDmd1ewG8p2gg9mLekhXT1V338ZJnKpump','PWGjTwYsKUnuavJS2a7irfqcZ3NF9YoiC53pF6KTedP'], //input wallet
      accountExclude: [],
      accountRequired: [RAYDIUM_PUBLIC_KEY.toBase58()],
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
  const solVault = accountKeys[0];
  const tokenVault = accountKeys[1];
  return {solVault, tokenVault};
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
