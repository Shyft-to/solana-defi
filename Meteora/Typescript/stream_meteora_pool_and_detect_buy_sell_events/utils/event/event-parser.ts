import { ProgramInfoType } from "@shyft-to/solana-transaction-parser";
import {
  ParsedTransactionWithMeta,
  PublicKey,
  VersionedTransactionResponse,
} from "@solana/web3.js";
import { BorshCoder, EventParser, Idl } from "@project-serum/anchor";
import bs58 from 'bs58';
import { decodeEvtSwap } from "./event-layout";

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
     const logMessages = txn.meta?.logMessages ?? [];

    const programStack: string[] = [];
    const eventLogs: { programId: string; data: string }[] = [];

     for (const log of logMessages) {
       const invokeMatch = log.match(/^Program (\S+) invoke \[(\d+)\]/);
       if (invokeMatch) {
         const depth = parseInt(invokeMatch[2]);
         programStack[depth - 1] = invokeMatch[1];
         // Trim any stale deeper entries
         programStack.length = depth;
       }
 
       if (log.startsWith("Program data: ")) {
         // The emitting program is always the deepest one currently on the stack
         const ownerProgram = programStack[programStack.length - 1];
         const b64 = log.replace("Program data: ", "").trim();
         eventLogs.push({ programId: ownerProgram, data: b64 });
       }

       const successMatch = log.match(/^Program (\S+) (success|failed)/);
       if (successMatch) {
         // Pop back up one level when a program finishes
         if (programStack.length > 0) {
           programStack.pop();
         }
       }
     }

     const availableProgramIds = Array.from(this.eventDecoders.keys()).map(p =>
       p.toString()
     );

     for (const programId of availableProgramIds) {
       const matchingLogs = eventLogs.filter(e => e.programId === programId);

       for (const { data } of matchingLogs) {
         try {
           const buf = Buffer.from(data, "base64");
           const decoded = decodeEvtSwap(buf.toString("hex"));
           if (decoded) {
             events.push({
               source: programId,
               kind: "event",
               name: "TradeEvent",
               data: decoded,
             });
           }
         } catch (_) {
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