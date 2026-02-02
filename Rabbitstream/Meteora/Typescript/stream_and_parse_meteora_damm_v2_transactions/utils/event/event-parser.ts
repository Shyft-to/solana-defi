import { ProgramInfoType } from "@shyft-to/solana-transaction-parser";
import {
  ParsedTransactionWithMeta,
  PublicKey,
  VersionedTransactionResponse,
  VersionedMessage,
} from "@solana/web3.js";
import { BorshCoder, EventParser, Idl } from "@coral-xyz/anchor";
import bs58 from "bs58";
import { decodeEvtSwap } from "./event-layout";

export class SolanaEventParser {
  private eventDecoders: Map<string, BorshCoder>;

  constructor(programInfos: ProgramInfoType[], private logger: Console) {
    this.eventDecoders = new Map();

    for (const programInfo of programInfos) {
      this.addParserFromIdl(
        programInfo.programId,
        programInfo.idl as Idl
      );
    }
  }

  addParserFromIdl(programId: PublicKey | string, idl: Idl) {
    if (!idl?.events) return;

    try {
      const coder = new BorshCoder(idl);
      this.eventDecoders.set(programId.toString(), coder);
    } catch (e) {
      this.logger.error({
        message: "SolanaEventParser.addParserFromIdl_error",
        data: { programId },
        error: e,
      });
    }
  }

  removeParser(programId: PublicKey | string) {
    this.eventDecoders.delete(programId.toString());
  }
  parseEvent(
    txn: VersionedTransactionResponse | ParsedTransactionWithMeta
  ) {
    const events: any[] = [];

    const message = txn.transaction.message;

    if (!("compiledInstructions" in message)) {
      return events;
    }

    const compiledMessage = message as VersionedMessage;

    const instructionData: any[] =
      compiledMessage.compiledInstructions.map((ix) => ix.data);

    if (instructionData.length === 0) return events;

    try {
      for (const [programId, _coder] of this.eventDecoders) {
        for (const data of instructionData) {

          try {
            const buf = Buffer.from(bs58.decode(data));
            const decoded = decodeEvtSwap(buf.toString("hex"));

            if (decoded) {
              events.push({
                source: programId,
                kind: "instruction-data",
                name: "TradeEvent",
                data: decoded,
              });
              break;
            }
          } catch {
          }
        }
      }

      return events;
    } catch (e) {
      this.logger.error({
        message: "SolanaEventParser.parseEvent_error",
        error: e,
      });
      return events;
    }
  }
  parseProgramLogMessages(programId: string, rawLogs: string[]) {
    try {
      const eventCoder = this.eventDecoders.get(programId);
      if (!eventCoder) return [];

      const eventParser = new EventParser(
        new PublicKey(programId),
        eventCoder
      );

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
