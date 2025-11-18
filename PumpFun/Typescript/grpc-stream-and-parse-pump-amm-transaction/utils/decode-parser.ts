import { PublicKey, VersionedTransactionResponse } from "@solana/web3.js";
import { bnLayoutFormatter } from "./bn-layout-formatter";
import { PUMP_AMM_EVENT_PARSER, PUMP_AMM_IX_PARSER, PUMP_AMM_PROGRAM_ID, TOKEN_PROGRAM_ID } from "./type";
import pumpSwapAmmIdl from "../idls/pump_amm_0.1.0.json";
import { BN } from "@coral-xyz/anchor";

export class PumpAmmDecoder {
  private idlMap: Map<string, string>;
  private idlInstructionsMap: Map<string, any>;

  constructor() {
    this.idlMap = new Map();
    this.idlInstructionsMap = new Map();
    this.initializeIdlMaps();
  }

  private initializeIdlMaps(): void {
    pumpSwapAmmIdl.instructions.forEach(inst => {
      const discHex = Buffer.from(inst.discriminator).toString('hex');
      this.idlMap.set(discHex, inst.name);
      this.idlInstructionsMap.set(inst.name, inst);
    });
  }

  public decodePumpAmmTxn(tx: VersionedTransactionResponse): any {
    const processedData = this.processPumpAmmInstructions(tx);
    
    if (!processedData) return null;

    const { instructions, inner_ixs, events } = processedData;

    const result = { 
      instructions: {
        pumpAmmIxs: instructions.map(ix => ({
          ...ix,
          accounts: this.formatAccounts(ix.accounts)
        })),
        events 
      }, 
      inner_ixs: inner_ixs.map(ix => ({
        ...ix,
        accounts: this.formatAccounts(ix.accounts)
      }))
    };
    
    bnLayoutFormatter(result);
    return result;
  }

  private processPumpAmmInstructions(tx: VersionedTransactionResponse): any {
    if (tx.meta?.err) return null;
    
    try {
      const paredIxs = PUMP_AMM_IX_PARSER.parseTransactionData(
        tx.transaction.message,
        tx.meta.loadedAddresses,
      );
      const hydratedTx = this.hydrateLoadedAddresses(tx);

      const pumpAmmIxs = paredIxs.filter((ix) =>
        ix.programId.equals(PUMP_AMM_PROGRAM_ID) || 
        ix.programId.equals(TOKEN_PROGRAM_ID)
      );

      const parsedInnerIxs = PUMP_AMM_IX_PARSER.parseTransactionWithInnerInstructions(hydratedTx);

      let pump_amm_inner_ixs = parsedInnerIxs.filter((ix) =>
        ix.programId.equals(PUMP_AMM_PROGRAM_ID) || 
        ix.programId.equals(TOKEN_PROGRAM_ID)
      );

      const { pumpFunIxs, pumpfun_inner_ixs } = this.decodeAndUpdatePumpInstructions(
        pumpAmmIxs,
        pump_amm_inner_ixs
      );

      if (pumpfun_inner_ixs.length === 0) return null;
      
      let parseEvents = PUMP_AMM_EVENT_PARSER.parseEvent(tx);
      let events = parseEvents.length === 0 ? PUMP_AMM_EVENT_PARSER.parseCpiEvent(tx) : parseEvents;
      
      const result = { instructions: pumpFunIxs, inner_ixs: pumpfun_inner_ixs, events };
      bnLayoutFormatter(result);

      return result;

    } catch(err) {
      console.error("Error processing Pump AMM instructions:", err);
      return null;
    }
  }

  // private decodeAndUpdatePumpInstructions(
  //   pumpAmmIxs: any[],
  //   pumpAmmInnerIxs: any[]
  // ): { pumpFunIxs: any[]; pumpfun_inner_ixs: any[] } {
    
  //   const decodeAndUpdateInstruction = (ix: any, index: number, array: any[]) => {
  //     let updatedIx = { ...ix };
      
  //     if (ix.name === 'unknown' && ix.args && typeof ix.args === 'object' && 'unknown' in ix.args) {
  //       const buf = (ix.args as { unknown: Buffer }).unknown;
  //       const discHex = buf.slice(0, 8).toString('hex');
        
  //       if (buf.length >= 24 && discHex === '66063d1201daebea') {        
  //         try {
  //           const baseAmountOut = buf.slice(8, 16).readBigUInt64LE();
  //           const maxQuoteAmountIn = buf.slice(16, 24).readBigUInt64LE();
            
  //           updatedIx = {
  //             ...ix,
  //             name: 'buy',
  //             args: {
  //               base_amount_out: new BN(baseAmountOut.toString()),
  //               max_quote_amount_in: new BN(maxQuoteAmountIn.toString()),
  //               track_volume: { __option: 'None' }
  //             }
  //           };
  //         } catch (err) {
  //           console.log("❌ Failed to decode buy instruction:", err);
  //         }
  //       }
  //       else if (this.idlMap.has(discHex)) {
  //         const instructionName = this.idlMap.get(discHex);
  //         console.log(`🔄 Updating unknown instruction to: ${instructionName}`);
  //         updatedIx = {
  //           ...ix,
  //           name: instructionName
  //         };
  //       }
  //     }
      
  //     if (updatedIx.name !== 'unknown') {
  //       const idlInstruction = this.idlInstructionsMap.get(updatedIx.name);
  //       if (idlInstruction) {
  //         updatedIx = this.mapAccountNames(updatedIx, idlInstruction);
  //       }
  //     }
      
  //     array[index] = updatedIx;
  //   };

  //   const pumpFunIxs = [...pumpAmmIxs];
  //   const pumpfun_inner_ixs = [...pumpAmmInnerIxs];

  //   pumpFunIxs.forEach((ix, index, array) => {
  //     decodeAndUpdateInstruction(ix, index, array);
  //   });
  //   pumpfun_inner_ixs.forEach((ix, index, array) => {
  //     decodeAndUpdateInstruction(ix, index, array);
  //   });

  //   return {
  //     pumpFunIxs,
  //     pumpfun_inner_ixs
  //   };
  // }
//   private decodeAndUpdatePumpInstructions(
//   pumpAmmIxs: any[],
//   pumpAmmInnerIxs: any[]
// ): { pumpFunIxs: any[]; pumpfun_inner_ixs: any[] } {

//   const decodeAndUpdateInstruction = (ix: any, index: number, array: any[]) => {
//     let updatedIx = { ...ix };

//     try {
//       if (ix.name === 'unknown' && ix.args && typeof ix.args === 'object' && 'unknown' in ix.args) {
//         const buf: Buffer = (ix.args as { unknown: Buffer }).unknown;
//         const lenBuf = buf.length;

//         // helpers for reading with bounds check
//         const readU64 = (offset: number) => offset + 8 <= lenBuf ? buf.readBigUInt64LE(offset) : null;
//         const readU32 = (offset: number) => offset + 4 <= lenBuf ? buf.readUInt32LE(offset) : null;
//         const readU16 = (offset: number) => offset + 2 <= lenBuf ? buf.readUInt16LE(offset) : null;

//         const tail = (n: number) => (lenBuf >= n ? buf.slice(lenBuf - n).toString() : '');
//         const hasBuyTrailer = tail(3) === 'buy';
//         const hasSellTrailer = tail(4) === 'sell';

//         const discHex = buf.slice(0, Math.min(8, lenBuf)).toString('hex');
//         const MATCH_OLD_BUY_DISC = discHex === '66063d1201daebea';

//         // -------------------
//         // BUY
//         // -------------------
//         if (hasBuyTrailer || (lenBuf >= 24 && MATCH_OLD_BUY_DISC)) {
//           try {
//             const baseAmountOut = readU64(8) ?? readU64(16);
//             const maxQuoteAmountIn = readU64(16) ?? readU64(24);

//             if (baseAmountOut && maxQuoteAmountIn) {
//               updatedIx = {
//                 ...ix,
//                 name: 'buy',
//                 args: {
//                   base_amount_out: new BN(baseAmountOut.toString()),
//                   max_quote_amount_in: new BN(maxQuoteAmountIn.toString()),
//                   track_volume: { __option: 'None' }
//                 }
//               };
//             }
//           } catch (err) {
//             console.error("❌ Failed to decode buy instruction:", err);
//           }
//         }

//         // -------------------
//         // SELL (mirror BUY layout)
//         // -------------------
//         else if (hasSellTrailer) {
//           try {
//             const tokenAmount = readU64(8);
//             const solAmount = readU64(16);
//             const timestamp = readU32(24);
//             const virtualToken = readU64(32);
//             const virtualSol = readU64(40);
//             const realToken = readU64(48);
//             const realSol = readU64(56);
//             const fee = readU64(64);
//             const creatorFeeBasis = readU16(72);
//             const creatorFee = readU64(80);

//             if (tokenAmount && solAmount) {
//               updatedIx = {
//                 ...ix,
//                 name: 'sell',
//                 args: {
//                   amount: new BN(tokenAmount.toString()),
//                   min_sol_output: new BN(solAmount.toString()),
//                   timestamp: timestamp ?? undefined,
//                   virtual_token_reserves: virtualToken ? new BN(virtualToken.toString()) : undefined,
//                   virtual_sol_reserves: virtualSol ? new BN(virtualSol.toString()) : undefined,
//                   real_token_reserves: realToken ? new BN(realToken.toString()) : undefined,
//                   real_sol_reserves: realSol ? new BN(realSol.toString()) : undefined,
//                   fee: fee ? new BN(fee.toString()) : undefined,
//                   creator_fee_basis_points: creatorFeeBasis ?? undefined,
//                   creator_fee: creatorFee ? new BN(creatorFee.toString()) : undefined,
//                   track_volume: { __option: 'None' }
//                 }
//               };
//             }
//           } catch (err) {
//             console.error("❌ Failed to decode sell instruction:", err);
//           }
//         }

//         // -------------------
//         // Fallback: IDL
//         // -------------------
//         else if (this.idlMap.has(discHex)) {
//           updatedIx = { ...ix, name: this.idlMap.get(discHex)! };
//         }
//       }

//       // Map account names if IDL exists
//       if (updatedIx.name !== 'unknown') {
//         const idlInstruction = this.idlInstructionsMap.get(updatedIx.name);
//         if (idlInstruction) updatedIx = this.mapAccountNames(updatedIx, idlInstruction);
//       }

//     } catch (err) {
//       console.error("❌ Error decoding instruction:", err);
//     }

//     array[index] = updatedIx;
//   };

//   const pumpFunIxs = [...pumpAmmIxs];
//   const pumpfun_inner_ixs = [...pumpAmmInnerIxs];

//   pumpFunIxs.forEach((ix, index, array) => decodeAndUpdateInstruction(ix, index, array));
//   pumpfun_inner_ixs.forEach((ix, index, array) => decodeAndUpdateInstruction(ix, index, array));

//   return { pumpFunIxs, pumpfun_inner_ixs };
// }
    private decodeAndUpdatePumpInstructions(
  pumpAmmIxs: any[],
  pumpAmmInnerIxs: any[]
): { pumpFunIxs: any[]; pumpfun_inner_ixs: any[] } {

  const decodeAndUpdateInstruction = (ix: any, index: number, array: any[]) => {
    let updatedIx = { ...ix };

    // Only handle 'unknown' instructions
    if (ix.name === 'unknown' && ix.args && typeof ix.args === 'object' && 'unknown' in ix.args) {
      const buf: Buffer = Buffer.from(ix.args.unknown, 'base64');
      const lenBuf = buf.length;

      // Helpers
      const readU64 = (offset: number) => offset + 8 <= lenBuf ? buf.readBigUInt64LE(offset) : null;
      const readU32 = (offset: number) => offset + 4 <= lenBuf ? buf.readUInt32LE(offset) : null;

      const discHex = buf.slice(0, Math.min(8, lenBuf)).toString('hex');

      // -------------------
      // SELL DECODING
      // -------------------
      // New sell layout: 256-byte instruction
      // Detect by discriminator length > old buy discriminator or known offset pattern
      if (lenBuf >= 120 && discHex !== '66063d1201daebea') {
        try {
          const amount = readU64(24);
          const minQuoteOut = readU64(32);
          const userBaseReserves = readU64(40);
          const userQuoteReserves = readU64(48);
          const poolBaseReserves = readU64(56);
          const poolQuoteReserves = readU64(64);
          const quoteAmountOut = readU64(72);

          const lpFeeBps = readU32(80);
          const lpFee = readU64(88);

          const protocolFeeBps = readU32(96);
          const protocolFee = readU64(104);

          const quoteOutNoLpFee = readU64(112); 
          const userQuoteOut = readU64(120);

          const timestamp = readU64(16);

          if (amount && minQuoteOut) {
            updatedIx = {
              ...ix,
              name: 'sell',
              args: {
                base_amount_in: new BN(amount.toString()),
                min_quote_amount_out: new BN(minQuoteOut.toString()),
                user_base_token_reserves: userBaseReserves ? new BN(userBaseReserves.toString()) : undefined,
                user_quote_token_reserves: userQuoteReserves ? new BN(userQuoteReserves.toString()) : undefined,
                pool_base_token_reserves: poolBaseReserves ? new BN(poolBaseReserves.toString()) : undefined,
                pool_quote_token_reserves: poolQuoteReserves ? new BN(poolQuoteReserves.toString()) : undefined,
                quote_amount_out: quoteAmountOut ? new BN(quoteAmountOut.toString()) : undefined,
                lp_fee: lpFee ? new BN(lpFee.toString()) : undefined,
                protocol_fee: protocolFee ? new BN(protocolFee.toString()) : undefined,
                lp_fee_basis_points: lpFeeBps ?? undefined,
                protocol_fee_basis_points: protocolFeeBps ?? undefined,
                quote_amount_out_without_lp_fee: quoteOutNoLpFee ? new BN(quoteOutNoLpFee.toString()) : undefined,
                user_quote_amount_out: userQuoteOut ? new BN(userQuoteOut.toString()) : undefined,
                timestamp: timestamp ? new BN(timestamp.toString()) : undefined,
                track_volume: { __option: 'None' }
              }
            };
          }
        } catch (err) {
          console.error("❌ Failed to decode sell instruction:", err);
        }
      }
    }

    // Map account names if IDL exists
    if (updatedIx.name !== 'unknown') {
      const idlInstruction = this.idlInstructionsMap.get(updatedIx.name);
      if (idlInstruction) updatedIx = this.mapAccountNames(updatedIx, idlInstruction);
    }

    array[index] = updatedIx;
  };

  const pumpFunIxs = [...pumpAmmIxs];
  const pumpfun_inner_ixs = [...pumpAmmInnerIxs];

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