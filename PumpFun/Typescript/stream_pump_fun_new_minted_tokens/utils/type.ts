import { SolanaParser } from "@shyft-to/solana-transaction-parser";
import { BN, BorshCoder, BorshInstructionCoder, Idl } from "@coral-xyz/anchor";
import { SolanaEventParser } from "./event-parser";
import { PublicKey } from "@solana/web3.js";
import pumpFunIdl from "../idls/pump_0.1.0.json";


export const PUMP_FUN_PROGRAM_ID = new PublicKey(
  "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P",
);
export const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");
export const PUMPFUN_MINT_AUTHORITY = new PublicKey('TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM');


export const PUMP_FUN_IX_PARSER = new SolanaParser([]);
PUMP_FUN_IX_PARSER.addParserFromIdl(
  PUMP_FUN_PROGRAM_ID.toBase58(),
  pumpFunIdl as Idl,
);
export const PUMP_FUN_EVENT_PARSER = new SolanaEventParser([], console);
PUMP_FUN_EVENT_PARSER.addParserFromIdl(
  PUMP_FUN_PROGRAM_ID.toBase58(),
  pumpFunIdl as Idl,
);
export const coder = new BorshInstructionCoder(pumpFunIdl as Idl);
