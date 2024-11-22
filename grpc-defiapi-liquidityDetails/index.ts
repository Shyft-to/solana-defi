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
import { PublicKey, VersionedTransactionResponse } from "@solana/web3.js";
import { tOutPut } from "./utils/transactionOutput";
import { LIQUIDITY_STATE_LAYOUT_V4 } from "@raydium-io/raydium-sdk";
import { getTokenInfo } from "./tools/getPooldetails";

const token = new PublicKey(
"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v",
); // token we are interested in

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
    try {
       if(!data.account.account) return;
        const poolInfo = await getTokenInfo(token.toString());
        if(poolInfo.fluxBeamLamports == undefined) return;
        // Extracting individual DEX data

        // Console logging each variable
        console.log("Raydium DEX Info:");
        console.log(`Token Account A: ${poolInfo?.raydiumTokenAccountA}`);
        console.log(`Token Account B: ${poolInfo?.raydiumTokenAccountB}`);
        console.log(`Mint A: ${poolInfo?.raydiumMintA}`);
        console.log(`Mint B: ${poolInfo?.raydiumMintB}`);

        console.log(`Lamports: ${poolInfo?.raydiumLamports}`);
        console.log(`LP Mint: ${poolInfo?.raydiumLpMint}`);
        console.log(`Public Key: ${poolInfo?.raydiumPublicKey}`);
        console.log(`Reserve: ${poolInfo?.reserve}`)

        console.log("\nFluxbeam DEX Info:");
        console.log(`Token Account A: ${poolInfo?.fluxBeamTokenAccountA}`);
        console.log(`Token Account B: ${poolInfo?.fluxBeamTokenAccountB}`);
        console.log(`Mint A: ${poolInfo?.fluxBeamMintA}`);
        console.log(`Mint B: ${poolInfo?.fluxBeamMintB}`);
        console.log(`Lamports: ${poolInfo?.fluxBeamLamports}`);
        console.log(`LP Mint: ${poolInfo?.fluxBeamLpMint}`);
        console.log(`Public Key: ${poolInfo?.fluxBeamPublicKey}`);

        console.log("\nOrca DEX Info:");
        console.log(`Token Account A: ${poolInfo?.orcaTokenAccountA}`);
        console.log(`Token Account B: ${poolInfo?.orcaTokenAccountB}`);
        console.log(`Mint A: ${poolInfo?.orcaMintA}`);
        console.log(`Mint B: ${poolInfo?.orcaMintB}`);
        console.log(`Liquidity: ${poolInfo?.orcaLiquidity}`);
        console.log(`Lamports: ${poolInfo?.orcaLamports}`);

        console.log(`Public Key: ${poolInfo?.orcaPublicKey}`);


        console.log("\nMeteora DEX Info:");
        console.log(`Token Account A: ${poolInfo?.meteoraTokenAccountA}`);
        console.log(`Token Account B: ${poolInfo?.meteoraTokenAccountB}`);
        console.log(`Mint A: ${poolInfo?.meteoraMintA}`);
        console.log(`Mint B: ${poolInfo?.meteoraMintB}`);
        console.log(`Lamports: ${poolInfo?.meteoraLamports}`);

        console.log(`Public Key: ${poolInfo?.meteoraPublicKey}`);
        console.log(`Vault LP A: ${poolInfo?.meteoraVaultLpA}`);
        console.log(`Vault LP B: ${poolInfo?.meteoraVaultLpB}`);

      
    } catch (error) {
        console.error("Error processing data:", error);
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
  slots: {},
  accounts: {
    usdc: {
      account: [],
      owner: ["TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"],
      filters: [
        {
          tokenAccountState: true,
        },
        {
          memcmp: {
            offset: String(0),
            base58:token.toString(),
          },
        },
      ],
    },
  },
  transactions: {},
  transactionsStatus: {},
  blocks: {},
  blocksMeta: {},
  entry: {},
  accountsDataSlice: [{ offset: String(32), length: String(40) }],
  commitment: CommitmentLevel.CONFIRMED,
};
function arbCalculation(amountA, amountB){
  if(amountA > amountB){
    const result = amountA - amountB
    return result;
  }else{
    return amountB - amountA
  }
}

subscribeCommand(client, req);

