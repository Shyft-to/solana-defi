import { PublicKey, VersionedTransactionResponse } from "@solana/web3.js";
import { bnLayoutFormatter } from "./bn-layout-formatter";
import { METEORA_DLMM_IX_PARSER, METEORA_DLMM_PROGRAM_ID, TOKEN_PROGRAM_ID, METEORA_DLMM_EVENT_PARSER } from "./type";
import meteoraDlmmIdl from "../idls/meteora_dlmm.json";

export class MeteoraDlmmDecoder {
  private idlMap: Map<string, string>;
  private idlInstructionsMap: Map<string, any>;

  constructor() {
    this.idlMap = new Map();
    this.idlInstructionsMap = new Map();
    this.initializeIdlMaps();
  }

  private initializeIdlMaps(): void {
    meteoraDlmmIdl.instructions.forEach(inst => {
      const discHex = Buffer.from(inst.discriminator).toString('hex');
      this.idlMap.set(discHex, inst.name);
      this.idlInstructionsMap.set(inst.name, inst);
    });
  }

  public decodeTxn(tx: VersionedTransactionResponse): any {
    const processedData = this.processInstructions(tx);
    
    if (!processedData) return null;

    const { instructions, inner_ixs, events } = processedData;
    const result = { 
      instructions: {
        meteoraDlmmIxs: instructions.map(ix => ({
          ...ix,
          accounts: this.formatAccounts(ix.accounts)
        })),
        events: events 
      }, 
      inner_ixs: inner_ixs.map(ix => ({
        ...ix,
        accounts: this.formatAccounts(ix.accounts)
      }))
    };
    
    bnLayoutFormatter(result);
    return result;
  }

  private processInstructions(tx: VersionedTransactionResponse): any {
    if (tx.meta?.err) return null;
    
    try {
      const parsedIxs = METEORA_DLMM_IX_PARSER.parseTransactionData(
        tx.transaction.message,
        tx.meta.loadedAddresses,
      );
      
      const hydratedTx = this.hydrateLoadedAddresses(tx);

      const meteoraDlmmIxs = parsedIxs.filter((ix) =>
        ix.programId.equals(METEORA_DLMM_PROGRAM_ID) || 
        ix.programId.equals(TOKEN_PROGRAM_ID)
      );

      const parsedInnerIxs = METEORA_DLMM_IX_PARSER.parseTransactionWithInnerInstructions(hydratedTx);

      let meteoraInnerIxs = parsedInnerIxs.filter((ix) =>
        ix.programId.equals(METEORA_DLMM_PROGRAM_ID) || 
        ix.programId.equals(TOKEN_PROGRAM_ID)
      );

      let events = METEORA_DLMM_EVENT_PARSER.parseEvent(tx);
      
      if (events.length === 0) {
        events = METEORA_DLMM_EVENT_PARSER.parseEvent(tx);
      }


      const result = { 
        instructions: meteoraDlmmIxs, 
        inner_ixs: meteoraInnerIxs, 
        events 
      };
      
      bnLayoutFormatter(result);

      return result;

    } catch(err) {
      console.error("Error processing Meteora DLMM instructions:", err);
      return null;
    }
  }

  private formatAccounts(accounts: any[] | undefined): any[] {
    if (!accounts) return [];
    
    return accounts.map(acc => {
      let pubkeyString: string;
      
      if (acc.pubkey && typeof acc.pubkey.toBase58 === 'function') {
        pubkeyString = acc.pubkey.toBase58();
      } else if (typeof acc.pubkey === 'string') {
        pubkeyString = acc.pubkey;
      } else if (acc.pubkey && acc.pubkey._bn) {
        try {
          pubkeyString = new PublicKey(acc.pubkey).toBase58();
        } catch {
          pubkeyString = 'unknown';
        }
      } else {
        pubkeyString = 'unknown';
      }
      
      return {
        ...acc, 
        pubkey: pubkeyString,
        isSigner: acc.isSigner || false,
        isWritable: acc.isWritable || false
      };
    });
  }

  private hydrateLoadedAddresses(tx: VersionedTransactionResponse): VersionedTransactionResponse {
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
}
