import { SolanaParser } from "@shyft-to/solana-transaction-parser";
import { BN, BorshCoder, BorshInstructionCoder, Idl } from "@coral-xyz/anchor";
import { SolanaEventParser } from "./event/event-parser";
import { PublicKey } from "@solana/web3.js";
import meteoraDlmmIdl from "../idls/meteora_dlmm.json";


export const METEORA_DLMM_PROGRAM_ID = new PublicKey(
  "LBUZKhRxPF3XUpBCjp4YzTKgLccjZhTSDM9YuVaPwxo"
);
export const TOKEN_PROGRAM_ID = new PublicKey("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");


export const METEORA_DLMM_IX_PARSER = new SolanaParser([]);
METEORA_DLMM_IX_PARSER.addParserFromIdl(
  METEORA_DLMM_PROGRAM_ID.toBase58(),
  meteoraDlmmIdl as Idl
);
export const coder = new BorshInstructionCoder(meteoraDlmmIdl as Idl);
export const METEORA_DLMM_EVENT_PARSER = new SolanaEventParser([], console);
METEORA_DLMM_EVENT_PARSER.addParserFromIdl(
  METEORA_DLMM_PROGRAM_ID.toBase58(),
  meteoraDlmmIdl as Idl
);

