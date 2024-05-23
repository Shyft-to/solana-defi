import {
    Connection,
    Keypair,
    PublicKey,
    TransactionInstruction,
    TransactionMessage,
    VersionedTransaction,
  } from "@solana/web3.js";
  
  import { SearcherClient } from "jito-ts/dist/sdk/block-engine/searcher";
  import { Bundle } from "jito-ts/dist/sdk/block-engine/types";
  import { isError } from "jito-ts/dist/sdk/block-engine/utils";
  import { ClientReadableStream } from "@grpc/grpc-js";
  import { TxVersion, buildSimpleTransaction } from "@raydium-io/raydium-sdk";
  
  
  import { BundleResult } from "jito-ts/dist/gen/block-engine/bundle";
import { Wallet } from "@project-serum/anchor";
  
// define these
//export const blockEngineUrl = 'tokyo.mainnet.block-engine.jito.wtf';
export const blockEngineUrl = 'amsterdam.mainnet.block-engine.jito.wtf';

const privateKey = new Uint8Array([
 //your privatekey
]);
 const wallet = new Wallet(Keypair.fromSecretKey(privateKey));



 const rpc_https_url = "https://rpc.shyft.to?api_key=your api key";
 const connection = new Connection(rpc_https_url, "confirmed");

  const MEMO_PROGRAM_ID = "Memo1UhkJRfHyvLMcVucJwxXeuD728EqVDDwQDxFMNo";
  
  export async function build_bundle(
    search: SearcherClient,
    // accounts: PublicKey[],
    // regions: string[],
    bundleTransactionLimit: number,
    swap_ix,
    conn: Connection
  ) {
    const _tipAccount = (await search.getTipAccounts())[4];

    console.log("tip account:", _tipAccount);
    const tipAccount = new PublicKey(_tipAccount);
  
   const lookupTableCache= {}
   const connection = new Connection(rpc_https_url, "confirmed");
  const makeTxVersion = TxVersion.V0 // LEGACY
  const addLookupTableInfo = undefined // only mainnet. other = undefined
    let message1 = "First TXN";
    let message2 = "Second TXN";
  
    const bund = new Bundle([], bundleTransactionLimit);
    const resp = await connection.getLatestBlockhash("processed");
  
    const willSendTx1 = await buildSimpleTransaction({
      connection,
      makeTxVersion,
      payer: wallet.publicKey,
      innerTransactions: swap_ix,
      addLookupTableInfo: addLookupTableInfo,
    });
  
  
    if (willSendTx1[0] instanceof VersionedTransaction) {
      willSendTx1[0].sign([wallet.payer]);
      // txids.push(await connection.sendTransaction(iTx, options));
                bund.addTransactions(willSendTx1[0]);
    }
  
  
    // bund.addTransactions(
    //   buildMemoTransaction(LP_wallet_keypair, resp.blockhash, message1)
    // );
  
    // bund.addTransactions(
    //   buildMemoTransaction(swap_wallet_keypair, resp.blockhash, message2)
    // );
  
    let maybeBundle = bund.addTipTx(
      wallet.payer,
      2000000,
      tipAccount,
      resp.blockhash
    );
  
    if (isError(maybeBundle)) {
      throw maybeBundle;
    }
    console.log();
  
    try {
      const response_bund = await search.sendBundle(maybeBundle);
      console.log("response_bund:", response_bund);
    } catch (e) {
      console.error("error sending bundle:", e);
    }
  
    return maybeBundle;
  }
  
  
  
  
  
  
  export const onBundleResult = (c: SearcherClient): Promise<number> => {
    let first = 0;
    let isResolved = false; 
  
    return new Promise((resolve) => {
      // Set a timeout to reject the promise if no bundle is accepted within 5 seconds
      setTimeout(() => {
        resolve(first);
        isResolved = true
      }, 30000);
  
      c.onBundleResult(
        
  
        (result) => {
          
          if (isResolved) return first;
          // clearTimeout(timeout); // Clear the timeout if a bundle is accepted
  
  
          const bundleId = result.bundleId;
          const isAccepted = result.accepted;
          const isRejected = result.rejected;
          if (isResolved == false){
  
            if (isAccepted) {
              console.log(
                "bundle accepted, ID:",
                result.bundleId,
                " Slot: ",
                result.accepted.slot
              );
              first +=1;
              isResolved = true;
              resolve(first); // Resolve with 'first' when a bundle is accepted
            }
    
            if (isRejected) {
              console.log("bundle is Rejected:", result);
              // Do not resolve or reject the promise here
            }
  
          }
         
        },
        (e) => {
          console.error(e);
          // Do not reject the promise here
        }
      );
    });
  };
  
  
  
  
  export const buildMemoTransaction = (
    keypair: Keypair,
    recentBlockhash: string,
    message: string
  ): VersionedTransaction => {
    const ix = new TransactionInstruction({
      keys: [
        {
          pubkey: keypair.publicKey,
          isSigner: true,
          isWritable: true,
        },
      ],
      programId: new PublicKey(MEMO_PROGRAM_ID),
      data: Buffer.from(message),
    });
  
    const instructions = [ix];
  
    const messageV0 = new TransactionMessage({
      payerKey: keypair.publicKey,
      recentBlockhash: recentBlockhash,
      instructions,
    }).compileToV0Message();
  
    const tx = new VersionedTransaction(messageV0);
  
    tx.sign([keypair]);
  
    return tx;
  };