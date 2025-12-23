import { SolanaParser } from "@shyft-to/solana-transaction-parser";
import {  BorshInstructionCoder, Idl } from "@coral-xyz/anchor";
import { SolanaEventParser } from "./event-parser";
import { PublicKey } from "@solana/web3.js";
import pumpSwapAmmIdl from "../idls/pump_amm_0.1.0.json";


export const PUMP_AMM_PROGRAM_ID = new PublicKey(
  "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA"
);
export const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");


export const PUMP_AMM_IX_PARSER = new SolanaParser([]);
PUMP_AMM_IX_PARSER.addParserFromIdl(
  PUMP_AMM_PROGRAM_ID.toBase58(),
  pumpSwapAmmIdl as Idl
);
export const coder = new BorshInstructionCoder(pumpSwapAmmIdl as Idl);
export const PUMP_AMM_EVENT_PARSER = new SolanaEventParser([], console);
PUMP_AMM_EVENT_PARSER.addParserFromIdl(
  PUMP_AMM_PROGRAM_ID.toBase58(),
  pumpSwapAmmIdl as Idl
);