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
  // private decodeAndUpdatePumpInstructions(
  //  pumpAmmIxs: any[],
  //  pumpAmmInnerIxs: any[]
  //  ): { pumpFunIxs: any[]; pumpfun_inner_ixs: any[] } {
  // const decodeAndUpdateInstruction = (ix: any, index: number, array: any[]) => {
  //   let updatedIx = { ...ix };

  //   try {
  //     if (ix.name === "unknown" && ix.args && typeof ix.args === "object" && "unknown" in ix.args) {
  //       const buf: Buffer = (ix.args as { unknown: Buffer }).unknown;
  //       const lenBuf = buf.length;

  //       const readU64 = (offset: number): bigint | null =>
  //         offset + 8 <= lenBuf ? buf.readBigUInt64LE(offset) : null;
  //       const readU32 = (offset: number): number | null =>
  //         offset + 4 <= lenBuf ? buf.readUInt32LE(offset) : null;
  //       const readU16 = (offset: number): number | null =>
  //         offset + 2 <= lenBuf ? buf.readUInt16LE(offset) : null;

  //       const tail = (n: number) => (lenBuf >= n ? buf.slice(lenBuf - n).toString() : "");

  //       const hasBuyTrailer = lenBuf >= 3 && tail(3) === "buy";
  //       const hasSellTrailer = lenBuf >= 4 && tail(4) === "sell";
  //       const hasBuyExactTrailer = lenBuf >= 16 && tail(16) === "buy_exact_sol_in";

  //       const discHex = buf.slice(0, Math.min(8, lenBuf)).toString("hex");
  //       const MATCH_OLD_BUY_DISC = discHex === "66063d1201daebea";
  //       if (hasBuyTrailer) {
  //         const solAmount = readU64(48);
  //         const tokenAmount = readU64(56);
  //         const timestamp = readU32(97);
  //         const virtualSol = readU64(105);
  //         const virtualToken = readU64(113);
  //         const realSol = readU64(121);
  //         const realToken = readU64(129);
  //         const feeBasis = readU16(169);
  //         const fee = readU64(177);
  //         const creatorFeeBasis = readU16(217);
  //         const creatorFee = readU64(225);

  //         if (tokenAmount !== null && solAmount !== null) {
  //           updatedIx = {
  //             ...ix,
  //             name: "buy",
  //             args: {
  //               base_amount_out: new BN(tokenAmount.toString()),
  //               max_quote_amount_in: new BN(solAmount.toString()),
  //               timestamp: timestamp ?? undefined,
  //               virtual_sol_reserves: virtualSol ? new BN(virtualSol.toString()) : undefined,
  //               virtual_token_reserves: virtualToken ? new BN(virtualToken.toString()) : undefined,
  //               real_sol_reserves: realSol ? new BN(realSol.toString()) : undefined,
  //               real_token_reserves: realToken ? new BN(realToken.toString()) : undefined,
  //               fee_basis_points: feeBasis ?? undefined,
  //               fee: fee ? new BN(fee.toString()) : undefined,
  //               creator_fee_basis_points: creatorFeeBasis ?? undefined,
  //               creator_fee: creatorFee ? new BN(creatorFee.toString()) : undefined,
  //               track_volume: { __option: "None" }
  //             }
  //           };
  //         }
  //       }
  //       else if (lenBuf >= 24 && MATCH_OLD_BUY_DISC) {
  //         try {
  //           const baseAmountOut = buf.slice(8, 16).readBigUInt64LE();
  //           const maxQuoteAmountIn = buf.slice(16, 24).readBigUInt64LE();
  //           updatedIx = {
  //             ...ix,
  //             name: "buy",
  //             args: {
  //               base_amount_out: new BN(baseAmountOut.toString()),
  //               max_quote_amount_in: new BN(maxQuoteAmountIn.toString()),
  //               track_volume: { __option: "None" }
  //             }
  //           };
  //         } catch {}
  //       }
  //       else if (hasBuyExactTrailer) {
  //         const solAmount = readU64(48);
  //         const tokenAmount = readU64(56);
  //         const timestamp = readU32(97);

  //         if (solAmount && tokenAmount) {
  //           updatedIx = {
  //             ...ix,
  //             name: "buy_exact_sol_in",
  //             args: {
  //               spendable_sol_in: new BN(solAmount.toString()),
  //               min_tokens_out: new BN(tokenAmount.toString()),
  //               timestamp: timestamp ?? undefined,
  //               track_volume: { __option: "None" }
  //             }
  //           };
  //         }
  //       }
  //       else if (hasSellTrailer) {
  //         const minSolOutput = readU64(48);
  //         const amount = readU64(56);
  //         const timestamp = readU32(97);
  //         const virtualSol = readU64(105);
  //         const virtualToken = readU64(113);
  //         const realSol = readU64(121);
  //         const realToken = readU64(129);
  //         const fee = readU64(169);
  //         const creatorFeeBasis = readU16(217);
  //         const creatorFee = readU64(225);

  //         if (amount !== null && minSolOutput !== null) {
  //           updatedIx = {
  //             ...ix,
  //             name: "sell",
  //             args: {
  //               token_amount: new BN(amount.toString()),
  //               min_sol_output: new BN(minSolOutput.toString()),
  //               timestamp: timestamp ?? undefined,
  //               virtual_token_reserves: virtualToken ? new BN(virtualToken.toString()) : undefined,
  //               virtual_sol_reserves: virtualSol ? new BN(virtualSol.toString()) : undefined,
  //               real_token_reserves: realToken ? new BN(realToken.toString()) : undefined,
  //               real_sol_reserves: realSol ? new BN(realSol.toString()) : undefined,
  //               fee: fee ? new BN(fee.toString()) : undefined,
  //               creator_fee_basis_points: creatorFeeBasis ?? undefined,
  //               creator_fee: creatorFee ? new BN(creatorFee.toString()) : undefined,
  //               track_volume: { __option: "None" }
  //             }
  //           };
  //         }
  //       }
  //       else if (this.idlMap.has(discHex)) {
  //         updatedIx = { ...ix, name: this.idlMap.get(discHex)! };
  //       }

  //       if (updatedIx.name !== "unknown") {
  //         const idlInstruction = this.idlInstructionsMap.get(updatedIx.name);
  //         if (idlInstruction) updatedIx = this.mapAccountNames(updatedIx, idlInstruction);
  //       }
  //     }

  //     array[index] = updatedIx;
  //   } catch (err) {
  //     console.error("Error decoding Pump instruction:", err);
  //     array[index] = updatedIx;
  //   }
  // };

  // const pumpFunIxs = [...pumpAmmIxs];
  // const pumpfun_inner_ixs = [...pumpAmmInnerIxs];

  // pumpFunIxs.forEach((ix, index, array) => decodeAndUpdateInstruction(ix, index, array));
  // pumpfun_inner_ixs.forEach((ix, index, array) => decodeAndUpdateInstruction(ix, index, array));

  //   return { pumpFunIxs, pumpfun_inner_ixs };
  // }

  private decodeAndUpdatePumpInstructions(
   pumpAmmIxs: any[],
   pumpAmmInnerIxs: any[]
   ): { pumpFunIxs: any[]; pumpfun_inner_ixs: any[] } {
  
   const filteredPumpAmmInnerIxs = pumpAmmInnerIxs.filter(ix => {
  
    if (ix.name === "unknown" && 
        ix.args && 
        typeof ix.args === "object" && 
        "unknown" in ix.args) {
      
      const buf: Buffer = (ix.args as { unknown: Buffer }).unknown;
      const base64Data = buf.toString('base64');
      
      let isEventEmission = false;
      
      if (base64Data.startsWith('5EWlLlHLmh2')) {
        isEventEmission = true;
      }
      
      if (ix.accounts && ix.accounts.length === 1) {
        const accountPubkey = ix.accounts[0].pubkey;
        const eventAuthorityPubkey = 'Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1';
        
        if (accountPubkey && 
            (accountPubkey.toString() === eventAuthorityPubkey ||
             accountPubkey === eventAuthorityPubkey)) {
          isEventEmission = true;
        }
      }
      
      if (base64Data.length > 150) {
        const last100Chars = base64Data.slice(-100);
        if (last100Chars.includes('AAAAAAAA') || 
            last100Chars.includes('////////') ||
            last100Chars.includes('======')) {
          isEventEmission = true;
        }
      }
      
      if (isEventEmission) {
        return false; 
      }
    }
    return true;
   });
  
   const decodeAndUpdateInstruction = (ix: any, index: number, array: any[]) => {
    let updatedIx: any = {
      name: ix.name,
      programId: ix.programId,
      accounts: ix.accounts,
    };

    const propertiesToCopy = ['parentProgramId'];
    for (const key in ix) {
      if (!['name', 'programId', 'accounts', 'args', ...propertiesToCopy].includes(key)) {
        updatedIx[key] = ix[key];
      }
    }

    updatedIx.args = ix.args;

    if (ix.parentProgramId) {
      updatedIx.parentProgramId = ix.parentProgramId;
    }

    try {
      if (ix.name === "unknown" && ix.args && typeof ix.args === "object" && "unknown" in ix.args) {
        const buf: Buffer = (ix.args as { unknown: Buffer }).unknown;
        const lenBuf = buf.length;

        const readU64 = (offset: number): bigint | null =>
          offset + 8 <= lenBuf ? buf.readBigUInt64LE(offset) : null;
        const readU32 = (offset: number): number | null =>
          offset + 4 <= lenBuf ? buf.readUInt32LE(offset) : null;
        const readU16 = (offset: number): number | null =>
          offset + 2 <= lenBuf ? buf.readUInt16LE(offset) : null;

        const tail = (n: number) => (lenBuf >= n ? buf.slice(lenBuf - n).toString() : "");

        const hasBuyTrailer = lenBuf >= 3 && tail(3) === "buy";
        const hasSellTrailer = lenBuf >= 4 && tail(4) === "sell";
        const hasBuyExactTrailer = lenBuf >= 16 && tail(16) === "buy_exact_sol_in";

        const discHex = buf.slice(0, Math.min(8, lenBuf)).toString("hex");
        const MATCH_OLD_BUY_DISC = discHex === "66063d1201daebea";
        
        if (hasBuyTrailer) {
          const solAmount = readU64(48);
          const tokenAmount = readU64(56);
          const timestamp = readU32(97);
          const virtualSol = readU64(105);
          const virtualToken = readU64(113);
          const realSol = readU64(121);
          const realToken = readU64(129);
          const feeBasis = readU16(169);
          const fee = readU64(177);
          const creatorFeeBasis = readU16(217);
          const creatorFee = readU64(225);

          if (tokenAmount !== null && solAmount !== null) {
            updatedIx.name = "buy";
            updatedIx.args = {
              base_amount_out: new BN(tokenAmount.toString()),
              max_quote_amount_in: new BN(solAmount.toString()),
              timestamp: timestamp ?? undefined,
              virtual_sol_reserves: virtualSol ? new BN(virtualSol.toString()) : undefined,
              virtual_token_reserves: virtualToken ? new BN(virtualToken.toString()) : undefined,
              real_sol_reserves: realSol ? new BN(realSol.toString()) : undefined,
              real_token_reserves: realToken ? new BN(realToken.toString()) : undefined,
              fee_basis_points: feeBasis ?? undefined,
              fee: fee ? new BN(fee.toString()) : undefined,
              creator_fee_basis_points: creatorFeeBasis ?? undefined,
              creator_fee: creatorFee ? new BN(creatorFee.toString()) : undefined,
              track_volume: { __option: "None" }
            };
          }
        }
        else if (lenBuf >= 24 && MATCH_OLD_BUY_DISC) {
          try {
            const baseAmountOut = buf.slice(8, 16).readBigUInt64LE();
            const maxQuoteAmountIn = buf.slice(16, 24).readBigUInt64LE();
            updatedIx.name = "buy";
            updatedIx.args = {
              base_amount_out: new BN(baseAmountOut.toString()),
              max_quote_amount_in: new BN(maxQuoteAmountIn.toString()),
              track_volume: { __option: "None" }
            };
          } catch {}
        }
        else if (hasBuyExactTrailer) {
          const solAmount = readU64(48);
          const tokenAmount = readU64(56);
          const timestamp = readU32(97);

          if (solAmount && tokenAmount) {
            updatedIx.name = "buy_exact_sol_in";
            updatedIx.args = {
              spendable_sol_in: new BN(solAmount.toString()),
              min_tokens_out: new BN(tokenAmount.toString()),
              timestamp: timestamp ?? undefined,
              track_volume: { __option: "None" }
            };
          }
        }
        else if (hasSellTrailer) {
          const minSolOutput = readU64(48);
          const amount = readU64(56);
          const timestamp = readU32(97);
          const virtualSol = readU64(105);
          const virtualToken = readU64(113);
          const realSol = readU64(121);
          const realToken = readU64(129);
          const fee = readU64(169);
          const creatorFeeBasis = readU16(217);
          const creatorFee = readU64(225);

          if (amount !== null && minSolOutput !== null) {
            updatedIx.name = "sell";
            updatedIx.args = {
              token_amount: new BN(amount.toString()),
              min_sol_output: new BN(minSolOutput.toString()),
              timestamp: timestamp ?? undefined,
              virtual_token_reserves: virtualToken ? new BN(virtualToken.toString()) : undefined,
              virtual_sol_reserves: virtualSol ? new BN(virtualSol.toString()) : undefined,
              real_token_reserves: realToken ? new BN(realToken.toString()) : undefined,
              real_sol_reserves: realSol ? new BN(realSol.toString()) : undefined,
              fee: fee ? new BN(fee.toString()) : undefined,
              creator_fee_basis_points: creatorFeeBasis ?? undefined,
              creator_fee: creatorFee ? new BN(creatorFee.toString()) : undefined,
              track_volume: { __option: "None" }
            };
          }
        }
        else if (this.idlMap.has(discHex)) {
          updatedIx.name = this.idlMap.get(discHex)!;
        }

        if (updatedIx.name !== "unknown") {
          const idlInstruction = this.idlInstructionsMap.get(updatedIx.name);
          if (idlInstruction) {
            updatedIx.accounts = this.mapAccountNames(updatedIx, idlInstruction).accounts;
          }
        }
      }

      array[index] = updatedIx;
    } catch (err) {
      console.error("Error decoding Pump instruction:", err);
      array[index] = updatedIx;
     }
   };

   const pumpFunIxs = [...pumpAmmIxs];
   const pumpfun_inner_ixs = [...filteredPumpAmmInnerIxs]; 

   pumpFunIxs.forEach((ix, index, array) => decodeAndUpdateInstruction(ix, index, array));
   pumpfun_inner_ixs.forEach((ix, index, array) => decodeAndUpdateInstruction(ix, index, array));

   return { pumpFunIxs, pumpfun_inner_ixs };
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