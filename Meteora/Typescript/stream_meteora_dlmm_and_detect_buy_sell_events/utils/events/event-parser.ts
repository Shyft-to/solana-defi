import { ProgramInfoType } from "@shyft-to/solana-transaction-parser";
import {
  Message,
  MessageV0,
  ParsedTransactionWithMeta,
  PublicKey,
  VersionedTransactionResponse,
} from "@solana/web3.js";
import { BorshCoder, EventParser, Idl } from "@coral-xyz/anchor";
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { decodeSwap } from "./event-layout";

export class SolanaEventParser {
  private eventDecoders: Map<PublicKey | string, BorshCoder>;
  constructor(
    programInfos: ProgramInfoType[],
    private logger: Console,
  ) {
    this.eventDecoders = new Map();
    for (const programInfo of programInfos) {
      this.addParserFromIdl(
        new PublicKey(programInfo.programId),
        programInfo.idl as Idl,
      );
    }
  }

  addParserFromIdl(programId: PublicKey | string, idl: Idl) {
    if (idl?.events) {
      try {
        const coder = new BorshCoder(idl);
        this.eventDecoders.set(programId, coder);
      } catch (e) {
      }
    }
  }

  removeParser(programId: PublicKey | string) {
    this.eventDecoders.delete(programId);
  }

   parseEvent(txn: VersionedTransactionResponse | ParsedTransactionWithMeta) {
    const events: any[] = [];
    const programData = txn.meta.innerInstructions.map((innerInstruction) => innerInstruction.instructions);
    try {
     if (!programData) return events;

     const allProgramData = programData.flat().map(instr => instr.data)
     const availableProgramIds = Array.from(this.eventDecoders.keys()).map(p => p.toString());

     for (const programId of availableProgramIds) {
      const eventCoder = this.eventDecoders.get(programId);
      if (!eventCoder) continue;
      for (const log of allProgramData) {
        if (log.startsWith("yCGxBopjnVNQkNP5usq")) {
           const buf = Buffer.from(bs58.decode(log)); 
            let decoded: any = decodeSwap(buf.toString("hex"))
           if (decoded) {
               events.push({ source: programId, kind: "account", name: "SwapEvent", data: decoded });
               break;
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