import { PublicKey, VersionedTransactionResponse } from "@solana/web3.js";
import { bnLayoutFormatter } from "./bn-layout-formatter";
import { BN } from "@coral-xyz/anchor";
import pumpFunIdl from "../idls/pump_0.1.0.json";
import { PUMP_FUN_EVENT_PARSER, PUMP_FUN_IX_PARSER, PUMP_FUN_PROGRAM_ID, TOKEN_PROGRAM_ID } from "./type";


export class PumpFunDecoder {
  private idlMap: Map<string, string>;
  private idlInstructionsMap: Map<string, any>;

  constructor() {
    this.idlMap = new Map();
    this.idlInstructionsMap = new Map();
    this.initializeIdlMaps();
  }

  private initializeIdlMaps(): void {
    pumpFunIdl.instructions.forEach(inst => {
      const discHex = Buffer.from(inst.discriminator).toString('hex');
      this.idlMap.set(discHex, inst.name);
      this.idlInstructionsMap.set(inst.name, inst);
    });
  }

  public decodePumpFunTxn(tx: VersionedTransactionResponse): any {
    const processedData = this.processPumpAmmInstructions(tx);
    
    if (!processedData) return null;

    const { instructions, inner_ixs, events } = processedData;

    const result = { 
      instructions: {
        pumpAmmIxs: instructions.map(ix => ({
          ...ix,
          accounts: this.formatAccounts(ix.accounts)
        })) 
      }, 
      inner_ixs: inner_ixs.pumpfun_inner_ixs.map(ix => ({
        ...ix,
        accounts: this.formatAccounts(ix.accounts)
      })),
      events: inner_ixs.events 
    };
    
    bnLayoutFormatter(result);
    return result;
  }

  private processPumpAmmInstructions(tx: VersionedTransactionResponse): any {
    if (tx.meta?.err) return null;
    
    try {
      const paredIxs = PUMP_FUN_IX_PARSER.parseTransactionData(
        tx.transaction.message,
        tx.meta.loadedAddresses,
      );
      const hydratedTx = this.hydrateLoadedAddresses(tx);

      const pumpAmmIxs = paredIxs.filter((ix) =>
        ix.programId.equals(PUMP_FUN_PROGRAM_ID) || 
        ix.programId.equals(TOKEN_PROGRAM_ID)
      );

      const parsedInnerIxs = PUMP_FUN_IX_PARSER.parseTransactionWithInnerInstructions(hydratedTx);

      let pump_amm_inner_ixs = parsedInnerIxs.filter((ix) =>
        ix.programId.equals(PUMP_FUN_PROGRAM_ID) || 
        ix.programId.equals(TOKEN_PROGRAM_ID)
      );

      const { pumpFunIxs, pumpfun_inner_ixs } = this.decodeAndUpdatePumpInstructions(
        pumpAmmIxs,
        pump_amm_inner_ixs
      );

      if (pumpfun_inner_ixs.length === 0) return null;
      
      let parseEvents = PUMP_FUN_EVENT_PARSER.parseEvent(tx);
      let events = parseEvents.length === 0 ? PUMP_FUN_EVENT_PARSER.parseCpiEvent(tx) : parseEvents;
      const result = { instructions: pumpFunIxs, inner_ixs:{pumpfun_inner_ixs, events} };
      bnLayoutFormatter(result);

      return result;

    } catch(err) {
      console.error("Error processing Pump AMM instructions:", err);
      return null;
    }
  }

  private decodeAndUpdatePumpInstructions(
    pumpAmmIxs: any[],
    pumpAmmInnerIxs: any[]
  ): { pumpFunIxs: any[]; pumpfun_inner_ixs: any[] } {
    
    const decodeAndUpdateInstruction = (ix: any, index: number, array: any[]) => {
      let updatedIx = { ...ix };
      
      if (ix.name === 'unknown' && ix.args && typeof ix.args === 'object' && 'unknown' in ix.args) {
        const buf = (ix.args as { unknown: Buffer }).unknown;
        const discHex = buf.slice(0, 8).toString('hex');
        
        if (buf.length >= 24 && discHex === '66063d1201daebea') {        
          try {
            const baseAmountOut = buf.slice(8, 16).readBigUInt64LE();
            const maxQuoteAmountIn = buf.slice(16, 24).readBigUInt64LE();
            
            updatedIx = {
              ...ix,
              name: 'buy',
              args: {
                base_amount_out: new BN(baseAmountOut.toString()),
                max_quote_amount_in: new BN(maxQuoteAmountIn.toString()),
                track_volume: { __option: 'None' }
              }
            };
          } catch (err) {
            console.log("❌ Failed to decode buy instruction:", err);
          }
        }
        else if (this.idlMap.has(discHex)) {
          const instructionName = this.idlMap.get(discHex);
          updatedIx = {
            ...ix,
            name: instructionName
          };
        }
      }
      
      if (updatedIx.name !== 'unknown') {
        const idlInstruction = this.idlInstructionsMap.get(updatedIx.name);
        if (idlInstruction) {
          updatedIx = this.mapAccountNames(updatedIx, idlInstruction);
        }
      }
      
      array[index] = updatedIx;
    };

    const pumpFunIxs = [...pumpAmmIxs];
    const pumpfun_inner_ixs = [...pumpAmmInnerIxs];

    pumpFunIxs.forEach((ix, index, array) => {
      decodeAndUpdateInstruction(ix, index, array);
    });
    pumpfun_inner_ixs.forEach((ix, index, array) => {
      decodeAndUpdateInstruction(ix, index, array);
    });

    return {
      pumpFunIxs,
      pumpfun_inner_ixs
    };
  }

  private mapAccountNames(instruction: any, idlInstruction: any): any {
    if (!idlInstruction || !idlInstruction.accounts) {
      return instruction;
    }
    
    const accountsWithNames = instruction.accounts.map((acc: any, index: number) => {
      if (index < idlInstruction.accounts.length) {
        const idlAccount = idlInstruction.accounts[index];
        return {
          ...acc,
          name: idlAccount.name || 'unknown'
        };
      }
      return {
        ...acc,
        name: 'unknown'
      };
    });
    
    return {
      ...instruction,
      accounts: accountsWithNames
    };
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