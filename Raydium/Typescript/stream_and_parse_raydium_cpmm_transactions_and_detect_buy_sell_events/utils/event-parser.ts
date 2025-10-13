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
import { bs58 } from "@coral-xyz/anchor/dist/cjs/utils/bytes";
import { decodeSwapEvent } from "./event-layout";

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
    try {
      let programIds: string[] = [];
      if (
        txn?.transaction.message instanceof Message ||
        txn?.transaction.message instanceof MessageV0
      ) {
        const accountKeys = txn.transaction.message.staticAccountKeys;
        txn.transaction.message.compiledInstructions.forEach((instruction) => {
          const programId = accountKeys[instruction.programIdIndex];
          if (programId) {
            programIds.push(programId.toBase58());
          }
        });
      } else {
        txn.transaction.message.instructions.forEach((instruction) => {
          programIds.push(instruction.programId.toBase58());
        });
      }
      const availableProgramIds = Array.from(this.eventDecoders.keys()).map(
        (programId) => programId.toString()
      );
      const commonProgramIds = intersection(availableProgramIds, programIds);
      if (commonProgramIds.length) {
        const events: any[] = [];
        for (const programId of commonProgramIds) {
          const eventCoder = this.eventDecoders.get(programId);
          if (!eventCoder) {
            continue;
          }

          const eventParser = new EventParser(
            new PublicKey(programId),
            eventCoder
          );
          const eventsArray = Array.from(
            eventParser.parseLogs(txn?.meta?.logMessages as string[])
          );
          events.push(...eventsArray);
        }
        return events;
      } else {
        return [];
      }
    } catch (e) {
      return [];
    }
  }
  parseCpiEvent(txn: VersionedTransactionResponse | ParsedTransactionWithMeta) {
    const events: any[] = [];
    try {
      if (!txn?.meta) return events;

    const availableProgramIds = Array.from(this.eventDecoders.keys()).map(p => p.toString());
    const programLogs =
      txn.meta.logMessages
        ?.filter(l => l.startsWith("Program data: "))
        .map(l => l.replace("Program data: ", "")) ?? [];

    if (programLogs.length === 0) return events;
    for (const programId of availableProgramIds) {
      const eventCoder = this.eventDecoders.get(programId);
      if (!eventCoder) continue;
      for (const data of programLogs) {
        if (data.startsWith("QMbN6CYIce")) {
          const buf = Buffer.from(data, "base64");
          const hex = buf.toString("hex");
          try{
          const decoded = decodeSwapEvent(hex, "hex");
          if (decoded) {
            events.push({
              source: programId,
              kind: "account",
              name: "SwapEvent",
              data: decoded,
            });
            break;
          }
        }catch(err){
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
