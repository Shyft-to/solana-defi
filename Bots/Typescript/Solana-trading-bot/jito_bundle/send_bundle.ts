

import {searcherClient} from 'jito-ts/dist/sdk/block-engine/searcher';
import {build_bundle, onBundleResult} from './build_bundle';
import { Wallet } from '@project-serum/anchor';
import { Connection, Keypair } from '@solana/web3.js';

 //export const blockEngineUrl = 'tokyo.mainnet.block-engine.jito.wtf';
export const blockEngineUrl = 'amsterdam.mainnet.block-engine.jito.wtf';
 const privateKey = new Uint8Array([
  //your api key
]);


 const wallet = new Wallet(Keypair.fromSecretKey(privateKey));



 const rpc_https_url = "https://rpc.shyft.to?api_key=your api key";
 const connection = new Connection(rpc_https_url, "confirmed");

export async function bull_dozer(swap_ix) {

  console.log('BLOCK_ENGINE_URL:', blockEngineUrl);
  const bundleTransactionLimit = parseInt('3');

  const search = searcherClient(blockEngineUrl, wallet.payer);


  await build_bundle(
    search,
    bundleTransactionLimit,
    swap_ix,
    connection
  );
 const bundle_result = await onBundleResult(search)
return bundle_result

// search.onBundleResult(
//   (bundle) => {
//     console.log(`JITO bundle result: ${JSON.stringify(bundle)}`);
//     return true;
//   },
//   (error) => {
//     console.log(`JITO bundle error: ${error}`);
//     return false;
//   }
// );

}
