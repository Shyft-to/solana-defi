import { PublicKey, VersionedTransactionResponse } from "@solana/web3.js";
import { bnLayoutFormatter } from "./bn-layout-formatter";
import {
  PUMP_AMM_EVENT_PARSER,
  PUMP_AMM_IX_PARSER,
  PUMP_AMM_PROGRAM_ID,
  TOKEN_PROGRAM_ID,
} from "./type";
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
    pumpSwapAmmIdl.instructions.forEach((inst) => {
      const discHex = Buffer.from(inst.discriminator).toString("hex");
      this.idlMap.set(discHex, inst.name);
      this.idlInstructionsMap.set(inst.name, inst);
    });

  }


  public decodePumpAmmTxn(tx: VersionedTransactionResponse): any {
    const processedData = this.processPumpAmmInstructions(tx);

    if (!processedData) {
      return null;
    }

    const { inner_ixs, events } = processedData;

    const result = {
      inner_ixs: inner_ixs.map((ix) => ({
        ...ix,
        accounts: this.formatAccounts(ix.accounts),
      })),
      events,
    };

    bnLayoutFormatter(result);
    return result;
  }


  private processPumpAmmInstructions(tx: VersionedTransactionResponse): any {
    if (tx.meta?.err) return null;

    try {
      const hydratedTx = this.hydrateLoadedAddresses(tx);

      let parsedInnerIxs: any[] = [];
    try {
    parsedInnerIxs = PUMP_AMM_IX_PARSER.parseTransactionWithInnerInstructions(hydratedTx);
    } catch {
     parsedInnerIxs = [];
    }

    if (!parsedInnerIxs.length && hydratedTx.transaction?.message?.compiledInstructions) {
      parsedInnerIxs = hydratedTx.transaction.message.compiledInstructions.map((ix: any) => {
      const programId = hydratedTx.transaction.message.staticAccountKeys[ix.programIdIndex];
      const programPubkey = new PublicKey(programId);
       const data = Buffer.from(ix.data);
      return {
       programId: programPubkey,
       name: "unknown",
       args: { unknown: data },
       accounts: ix.accountKeyIndexes.map((idx: number) => ({
         pubkey: hydratedTx.transaction.message.staticAccountKeys[idx],
         isSigner: false,
         isWritable: true,
       })),
      };
    });
    }

  if (!parsedInnerIxs.length) {
    return null;
  }


      if (!parsedInnerIxs.length) {
        console.log("❌ No parsed inner instructions found in this transaction.");
        return null;
      }

      let pump_amm_inner_ixs = parsedInnerIxs.filter(
        (ix) =>
          ix.programId.equals(PUMP_AMM_PROGRAM_ID) ||
          ix.programId.equals(TOKEN_PROGRAM_ID)
      );


      const { pumpfun_inner_ixs } =
        this.decodeAndUpdatePumpInstructions(pump_amm_inner_ixs);

      if (pumpfun_inner_ixs.length === 0) {
 
        return null;
      }

      let parseEvents: any[] = [];
      try {
        parseEvents = PUMP_AMM_EVENT_PARSER.parseEvent(tx);
      } catch {
        parseEvents = [];
      }

      const events = parseEvents.length === 0 ? null : parseEvents;
      const result = { inner_ixs: pumpfun_inner_ixs, events };
      bnLayoutFormatter(result);

      return result;
    } catch (e) {
      console.error("❌ Error while processing Pump AMM instructions:", e);
      return null;
    }
  }


  private decodeAndUpdatePumpInstructions(
    pumpAmmInnerIxs: any[]
  ): { pumpfun_inner_ixs: any[] } {
    const pumpfun_inner_ixs = [...pumpAmmInnerIxs];

    pumpfun_inner_ixs.forEach((ix, index, array) => {
      let updatedIx = { ...ix };

      if (
        ix.name === "unknown" &&
        ix.args &&
        typeof ix.args === "object" &&
        "unknown" in ix.args
      ) {
        const buf = (ix.args as { unknown: Buffer }).unknown;
        const discHex = buf.slice(0, 8).toString("hex");


        if (buf.length >= 24 && discHex === "66063d1201daebea") {
          try {
            const baseAmountOut = buf.slice(8, 16).readBigUInt64LE();
            const maxQuoteAmountIn = buf.slice(16, 24).readBigUInt64LE();

            updatedIx = {
              ...ix,
              name: "buy",
              args: {
                base_amount_out: new BN(baseAmountOut.toString()),
                max_quote_amount_in: new BN(maxQuoteAmountIn.toString()),
                track_volume: { __option: "None" },
              },
            };
          } catch (err) {
            console.log("❌ Failed to decode buy instruction:", err);
          }
        }

        else if (this.idlMap.has(discHex)) {
          const instructionName = this.idlMap.get(discHex)!;
          const idlInstruction = this.idlInstructionsMap.get(instructionName);

          try {
            if (instructionName === "buy_exact_quote_in" && buf.length >= 24) {
              const spendableQuoteIn = buf.slice(8, 16).readBigUInt64LE();
              const minBaseAmountOut = buf.slice(16, 24).readBigUInt64LE();

              updatedIx = {
                ...ix,
                name: instructionName,
                args: {
                  spendable_quote_in: new BN(spendableQuoteIn.toString()),
                  min_base_amount_out: new BN(minBaseAmountOut.toString()),
                  track_volume: { __option: "None" },
                },
              };
            } else {
              updatedIx = {
                ...ix,
                name: instructionName,
              };
            }
          } catch (err) {
            console.log(`⚠️ Failed to decode ${instructionName} args:`, err);
          }
        } else {
        }
      }

      if (updatedIx.name && updatedIx.name !== "unknown") {
        const idlInstruction = this.idlInstructionsMap.get(updatedIx.name);
        if (idlInstruction) {
          updatedIx = this.mapAccountNames(updatedIx, idlInstruction);
        }
      }

      array[index] = updatedIx;
    });

    return { pumpfun_inner_ixs };
  }

  private mapAccountNames(instruction: any, idlInstruction: any): any {
    if (!idlInstruction || !idlInstruction.accounts) {
      return instruction;
    }

    const accountsWithNames = instruction.accounts.map(
      (acc: any, index: number) => {
        if (index < idlInstruction.accounts.length) {
          const idlAccount = idlInstruction.accounts[index];
          return {
            ...acc,
            name: idlAccount.name || "unknown",
          };
        }
        return {
          ...acc,
          name: "unknown",
        };
      }
    );

    return {
      ...instruction,
      accounts: accountsWithNames,
    };
  }

  private formatAccounts(accounts: any[] | undefined): any[] {
    if (!accounts) return [];

    return accounts.map((acc) => {
      let pubkeyString: string;

      if (acc.pubkey && typeof acc.pubkey.toBase58 === "function") {
        pubkeyString = acc.pubkey.toBase58();
      } else if (typeof acc.pubkey === "string") {
        pubkeyString = acc.pubkey;
      } else if (acc.pubkey && acc.pubkey._bn) {
        try {
          pubkeyString = new PublicKey(acc.pubkey).toBase58();
        } catch {
          pubkeyString = "unknown";
        }
      } else {
        pubkeyString = "unknown";
      }

      return {
        ...acc,
        pubkey: pubkeyString,
        isSigner: acc.isSigner || false,
        isWritable: acc.isWritable || false,
      };
    });
  }
  private hydrateLoadedAddresses(
    tx: VersionedTransactionResponse
  ): VersionedTransactionResponse {
    if (!tx.meta) {
      tx.meta = {} as any;
    }

    if (!tx.meta.loadedAddresses) {
      tx.meta.loadedAddresses = {
        writable: [],
        readonly: [],
      };
      return tx;
    }

    const loaded = tx.meta.loadedAddresses;
    const ensurePublicKey = (arr?: any[]): PublicKey[] => {
      if (!arr) return [];
      return arr.map((item: any) => {
        try {
          if (item instanceof PublicKey) return item;
          if (Buffer.isBuffer(item) || item instanceof Uint8Array)
            return new PublicKey(item);
          if (typeof item === "string") {
            try {
              const buf = Buffer.from(item, "base64");
              if (buf.length === 32) return new PublicKey(buf);
            } catch {}
            return new PublicKey(item);
          }
          return new PublicKey(item);
        } catch {
          return new PublicKey(Buffer.alloc(32));
        }
      });
    };

    tx.meta.loadedAddresses = {
      writable: ensurePublicKey((loaded as any).writable),
      readonly: ensurePublicKey((loaded as any).readonly),
    };

    return tx;
  }
}
