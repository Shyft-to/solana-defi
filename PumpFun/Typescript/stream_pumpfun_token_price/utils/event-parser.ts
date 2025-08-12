import { ProgramInfoType } from "@shyft-to/solana-transaction-parser";
import {
  Message,
  MessageV0,
  ParsedTransactionWithMeta,
  PublicKey,
  VersionedTransactionResponse,
} from "@solana/web3.js";
import { BorshCoder, EventParser, Idl } from "@coral-xyz/anchor";
import { intersection } from "lodash";

export class SolanaEventParser {
  private eventDecoders: Map<PublicKey | string, BorshCoder>;
  constructor(programInfos: ProgramInfoType[], private logger: Console) {
    this.eventDecoders = new Map();
    for (const programInfo of programInfos) {
      this.addParserFromIdl(
        new PublicKey(programInfo.programId),
        programInfo.idl as Idl
      );
    }
  }

  addParserFromIdl(programId: PublicKey | string, idl: Idl) {
    if (idl?.events) {
      try {
        const coder = new BorshCoder(idl);
        this.eventDecoders.set(programId, coder);
      } catch (e) {
        this.logger.error({
          message: "SolanaEventParser.addParserFromIdl_error",
          data: { programId },
          error: e,
        });
      }
    }
  }

  removeParser(programId: PublicKey | string) {
    this.eventDecoders.delete(programId);
  }
  parseEvent(txn: VersionedTransactionResponse | ParsedTransactionWithMeta) {
    const events: any[] = [];
    try {
     if (!txn?.meta?.logMessages) return events;

     const allLogs = txn.meta.logMessages;
     const availableProgramIds = Array.from(this.eventDecoders.keys()).map(p => p.toString());

     for (const programId of availableProgramIds) {
      const eventCoder = this.eventDecoders.get(programId);
      if (!eventCoder) continue;

      const programKey = new PublicKey(programId);

      try {
        const anchorEvents = Array.from(new EventParser(programKey, eventCoder).parseLogs(allLogs));
        if (anchorEvents.length > 0) {
          events.push(...anchorEvents);
          continue;
        }
      } catch (err) {
        this.logger.warn({
          message: "Anchor event parse failed, trying raw decode",
          data: { programId },
          error: err,
        });
      }

      let insideProgram = false;

      for (const log of allLogs) {
        if (log.startsWith(`Program ${programId} invoke`)) {
          insideProgram = true;
          continue;
        }

        if (log.startsWith(`Program ${programId} success`) || log.startsWith(`Program ${programId} failed`)) {
          insideProgram = false;
          continue;
        }

        if (insideProgram && log.startsWith("Program data:")) {
          const base64Data = log.replace("Program data: ", "").trim();
          const buf = Buffer.from(base64Data, "base64");

          let decoded: any = null;

          if ((eventCoder as any).idl?.events) {
            for (const e of (eventCoder as any).idl.events) {
              try {
                decoded = eventCoder.types.decode(e.name, buf);
                if (decoded) {
                  events.push({ source: programId, kind: "event", name: e.name, data: decoded });
                  break;
                }
              } catch {}
            }
          }

          if (!decoded && (eventCoder as any).idl?.accounts) {
            for (const acc of (eventCoder as any).idl.accounts) {
              try {
                decoded = eventCoder.accounts.decode(acc.name, buf);
                if (decoded) {
                  events.push({ source: programId, kind: "account", name: acc.name, data: decoded });
                  break;
                }
              } catch {}
            }
          }

          if (!decoded && (eventCoder as any).idl?.types) {
            for (const t of (eventCoder as any).idl.types) {
              try {
                decoded = eventCoder.types.decode(t.name, buf);
                if (decoded) {
                  events.push({ source: programId, kind: "type", name: t.name, data: decoded });
                  break;
                }
              } catch {}
            }
          }

          if (!decoded) {
            events.push({
              source: programId,
              kind: "raw",
              data: buf.toString("hex"),
            });
          }
        }
      }
    }

     return events;
   } catch (e) {
    this.logger.error({ message: "SolanaEventParser.parseEvent_error", error: e });
    return events;
   }
  }

  parseProgramLogMessages(programId: string, rawLogs: string[]) {
    try {
      const eventCoder = this.eventDecoders.get(programId);
      if (!eventCoder) {
        return [];
      }
      const eventParser = new EventParser(new PublicKey(programId), eventCoder);
      return Array.from(eventParser.parseLogs(rawLogs));
    } catch (err) {
      this.logger.error({
        message: "SolanaEventParser.parseProgramLogMessages_error",
        data: { programId, rawLogs },
        error: err,
      });
      return [];
    }
  }

  getEventCoder(programId: string) {
    return this.eventDecoders.get(programId);
  }
}
