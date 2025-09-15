import { SolanaParser } from "@shyft-to/solana-transaction-parser";
import { PublicKey, VersionedTransactionResponse } from "@solana/web3.js";
import { SolanaEventParser } from "./pump_interface/src/event/event-parser";
import pumpFunIdl from "../idls/pump_0.1.0.json";
import { bnLayoutFormatter } from "../utils/bn-layout-formatter";
import { Idl } from "@coral-xyz/anchor";
import { filtered_parsed_txn } from "./pump_interface/pumpfun-ix-resolver";
import { PUMP_FUN_PROGRAM_ID } from "../utils/type";

const PUMP_FUN_IX_PARSER = new SolanaParser([]);
PUMP_FUN_IX_PARSER.addParserFromIdl(
  PUMP_FUN_PROGRAM_ID.toBase58(),
  pumpFunIdl as Idl,
);
const PUMP_FUN_EVENT_PARSER = new SolanaEventParser([], console);
PUMP_FUN_EVENT_PARSER.addParserFromIdl(
  PUMP_FUN_PROGRAM_ID.toBase58(),
  pumpFunIdl as Idl,
);


export function decodePumpFunTxn(tx: VersionedTransactionResponse) {
  if (tx.meta?.err) return;
   try{
      const paredIxs = PUMP_FUN_IX_PARSER.parseTransactionData(
      tx.transaction.message,
      tx.meta.loadedAddresses,
    );
    const pumpFunIxs = paredIxs.filter((ix) =>
     ix.programId.equals(PUMP_FUN_PROGRAM_ID) || 
    ix.programId.equals(new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"))   ,
    );
    const hydratedTx = hydrateLoadedAddresses(tx);
    const parsedInnerIxs = PUMP_FUN_IX_PARSER.parseTransactionWithInnerInstructions(hydratedTx);

    const pump_inner_ixs = parsedInnerIxs.filter((ix) =>
      ix.programId.equals(PUMP_FUN_PROGRAM_ID) || 
      ix.programId.equals(new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"))
   || ix.programId.equals(new PublicKey("11111111111111111111111111111111"))
      ,
     );
   let pumpfun_inner_ixs = filtered_parsed_txn(pump_inner_ixs)
   let parseEvents = PUMP_FUN_EVENT_PARSER.parseEvent(tx);
   let event = parseEvents.length === 0? PUMP_FUN_EVENT_PARSER.parseCpiEvent(tx): parseEvents;
   const result = { instructions: pumpFunIxs, inner_ixs: pumpfun_inner_ixs, event };

   bnLayoutFormatter(result);

  return result;
  }catch(err){
    console.log(err)
  }
}
function hydrateLoadedAddresses(tx: VersionedTransactionResponse): VersionedTransactionResponse {
  const loaded = tx.meta?.loadedAddresses;
  if (!loaded) return tx;

  function ensurePublicKey(arr: (Buffer | PublicKey)[]) {
    return arr.map(item =>
      item instanceof PublicKey ? item : new PublicKey(item)
    );
  }

  tx.meta.loadedAddresses = {
    writable: ensurePublicKey(loaded.writable),
    readonly: ensurePublicKey(loaded.readonly),
  };

  return tx;
}