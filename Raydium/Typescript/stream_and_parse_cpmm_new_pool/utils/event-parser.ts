import { ProgramInfoType } from "@shyft-to/solana-transaction-parser";
import {
  Message,
  MessageV0,
  ParsedTransactionWithMeta,
  PublicKey,
  VersionedTransactionResponse,
} from "@solana/web3.js";
import { BorshCoder, EventParser, Idl } from "@project-serum/anchor";
import { intersection } from "lodash";

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
        (programId) => programId.toString(),
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
            eventCoder,
          );
          const eventsArray = Array.from(
            eventParser.parseLogs(txn?.meta?.logMessages as string[]),
          );
          events.push(...eventsArray);
        //  console.log(eventParser)
        }
        return events;
      } else {
        return [];
      }
    } catch (e) {
      return [];
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
