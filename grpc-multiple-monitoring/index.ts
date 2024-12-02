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

  const tokens = [
    "AzAMgBCY7xM5FfJuVzb4dm46TJEfrmrar7XX9doEpump",
    "HmRpmbeGosahTzChmTmKzFHR5SmgSgXFAXKumajgpump",
    "65FTWaP4ERLYFkQ4QaKgFUeKncACdtx1dfEFyKJapump",
    "J5sHn79BS1n64SeT4RJXqYP1g1ZVSXjchYgXS848pump",
    "7S37Wv8v9BLQ7bCBTSmCgpCHEb7ae9ZVV7YGwDSepump",
    "AZZyEKLE288XgF795nkktMC9poBgsq93mkcZQxRrpump",
    "71DiBwJoSSY38t8TAK93o1bepYL8gpxoP9g7Jkg9pump",
    "HV4M7zRcxgArwFLxUgX7wpLF4qLmVemiFUiHUJ4Fpump",
    "5n2WeFEQbfV65niEP63sZc3VA7EgC4gxcTzsGGuXpump",
    "4oJh9x5Cr14bfaBtUsXN1YUZbxRhuae9nrkSyWGSpump",
    "GBpE12CEBFY9C74gRBuZMTPgy2BGEJNCn4cHbEPKpump",
    "oraim8c9d1nkfuQk9EzGYEUGxqL3MHQYndRw1huVo5h",
    "DTTLrCGbqn6fmNuKjGYqWFeQU5Hz153f5C3pNnxepump",
    "tqrRt3AsuGa3aYYSvABMDrPJEWVzpAnh3p6LxXGpump",
    "HuLHkBebCrvSnpKrMBL9yk4PH1Fu1J6es4cnDx6kcSpY",
    "uSuKePc3MotnSm4NeLMjcZofUqiAaBH8SNMJ3kwpump",
    "MVipSZ3kMJskEmY9dsiC4VAQVPJbKQL25wkyacKpump",
    "Au3i4Dh9UKb6SKg1ScVADcXwFdawesq72b52Mpmapump",
    "BKXQnRzZFq1DXyiJiVC6Y5EbUYMgYCMx9U89D5Yrpump",
    "3sy7mXKwRFrjUPayZy7yzWv8XRB82AWC97jN98wrpump",
    "97Mbx6Jym1iEkQdVfLf5PMWL5tARGnQ5na5jhR9Lpump",
    "8gxEGKvrJ4U7ygaoPopC4tpfbwTLyjPmZQzsjs2Ypump",
    "EGNhB85H9EGx5yxYPVcBSGhrmEsgWdeVmSY8Fvbtpump",
    "FexAz5fnPVUezWKDqjKMG5pz3GqCJMPNsmcWp5Srpump",
    "9C8FQFHxaxNng2bkdjLK4W9cLZ61bN3Ybgymk5sbpump",
    "Av8agXfDJZucEUb561JNP1SmktnWP6ZgN8Uum9z7pump",
    "9huESaYrXtsdx4gGLHxrSZ9Uta3sN2SKZUYux5ZVpump",
    "7FBXUry78ce5n1LLMAm4xnGWPFkQYZ5m3E8HFWL3UqwH",
    "8i51XNNpGaKaj4G4nDdmQh95v4FKAxw8mhtaRoKd9tE8",
    "AAdGJMGsJzfpERcYMpU4As1gNKKWrkKpQu9GCnvFpump",
    "J9z7yu57rvZwsKFibdPrjCzy1MBn6VS8oGtbdskFpump",
    "8QR8aemDXrXn5bRdCXLbYKbCKz9r6ivs3e2brPJ5pump"
  ];
  
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
        const quoteVault = accountKeys?.solVault;
        const baseVault = accountKeys?.tokenVault;
        if (!baseVault || !quoteVault) {
          console.error('Vaults are undefined or missing.');
           return;
          }
      //  console.log(`${baseVault} ${quoteVault}`)
          const tokenInfo = await getTokenInfo(info.ca)
          const quoteBal = await getSolBalance(quoteVault);
          const baseBal = await getTokenBalance(baseVault)/ 10 ** tokenInfo.decimal;
           const marketInfo = await getMarketInfo(baseBal,quoteBal,tokenInfo.currentSupply)
           const quoteBal$ = marketInfo.quote$ //sol = $241, you can change it in ...getMarketInfo()
           const price = marketInfo.price;
           const marketcap = marketInfo.marketcap;
           if(isNaN(price) && isNaN(marketcap)) return;
           const supply = marketInfo.currentSupply;
          const stringify : any = stringifyWithBigInt(createPoolIx.args);
          console.log(
            `Signature: ${txn.transaction.signatures[0]}
             CA : ${info.ca}
             BASE VAULT : ${baseVault}
             QUOTE VAULT : ${quoteVault}
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
      accountInclude: tokens,
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
  commitment: CommitmentLevel.PROCESSED,
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
