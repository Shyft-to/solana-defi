import {
    ParsedInstruction,
    parseLogs,
  } from "@shyft-to/solana-transaction-parser";
  import { Idl } from "@project-serum/anchor";
  import { RaydiumAmmParser } from "../../utils/raydium-amm-parser";
  import { RaydiumAmmLogsParser } from "./raydium-amm-logs-parser";
  
  export type LogEvent = {
    name: string;
    data: any;
  };
  
  const RAYDIUM_AMM_PROGRAM_ID = RaydiumAmmParser.PROGRAM_ID.toBase58();
  
  export class LogsParser {
    raydiumAmmLogsParser = new RaydiumAmmLogsParser();
    parse(
      actions: ParsedInstruction<Idl, string>[],
      logMessages: string[],
    ): LogEvent[] {
      if (!this.isValidIx(actions)) {
        return [];
      }
  
      const logs = parseLogs(logMessages);
  
      return actions
        .map((action, index) => {
          if ("info" in action) {
            return;
          } else {
            const programId = action.programId.toBase58();
            switch (programId) {
              case RAYDIUM_AMM_PROGRAM_ID: {
                return this.raydiumAmmLogsParser.parse(action, logs[index]);
              }
              default:
                return;
            }
          }
        })
        .filter((log) => Boolean(log)) as LogEvent[];
    }
  
    isValidIx(actions: ParsedInstruction<Idl, string>[]): boolean {
      return actions.some(
        (action) => action.programId.toBase58() === RAYDIUM_AMM_PROGRAM_ID,
      );
    }
  }
  