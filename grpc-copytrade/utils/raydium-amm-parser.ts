import * as base58 from "bs58";
import { struct, u8 } from "@solana/buffer-layout";
import { publicKey, u64 } from "@solana/buffer-layout-utils";
import { PublicKey, TransactionInstruction } from "@solana/web3.js";

type SwapBaseIn = {
  logType: number;
  amountIn: bigint;
  minimumOut: bigint;
  direction: bigint;
  userSource: bigint;
  poolCoin: bigint;
  poolPc: bigint;
  outAmount: bigint;
}

const SwapBaseInLayout = struct<SwapBaseIn>([
  u8("logType"),
  u64("amountIn"),
  u64("minimumOut"),
  u64("direction"),
  u64("userSource"),
  u64("poolCoin"),
  u64("poolPc"),
  u64("outAmount")
]);

type SwapBaseOut = {
  logType: number;
  maxIn: bigint;
  amountOut: bigint;
  direction: bigint;
  userSource: bigint;
  poolCoin: bigint;
  poolCoinTokenAccount:PublicKey;
  poolPc: bigint;
  directIn: bigint;
}

const SwapBaseOutLayout = struct<SwapBaseOut>([
  u8("logType"),
  u64("maxIn"),
  u64("amountOut"),
  u64("direction"),
  u64("userSource"),
  u64("poolCoin"),
  publicKey("poolCoinTokenAccount"),
  u64("poolPc"),
  u64("directIn")
]);

type RaydiumInitializeArgs = {
  nonce: number;
  openTime: bigint;
};
const RaydiumInitializeArgsLayout = struct<RaydiumInitializeArgs>([
  u8("nonce"),
  u64("openTime"),
]);


type RaydiumInitialize2Args = RaydiumInitializeArgs & {
  initPcAmount: bigint;
  initCoinAmount: bigint;
};
const RaydiumInitialize2ArgsLayout = struct<RaydiumInitialize2Args>([
  u8("nonce"),
  u64("openTime"),
  u64("initPcAmount"),
  u64("initCoinAmount"),
]);

export class RaydiumAmmParser {
  static PROGRAM_ID = new PublicKey(
    "675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8",
  );

  parseInstruction(instruction: TransactionInstruction) {
    const instructionData = instruction.data;
    const instructionType = u8().decode(instructionData);
    switch (instructionType) {
      case 0: {
        return this.parseRaydiumInitializeIx(instruction);
      }
      case 1: {
        return this.parseRaydiumInitialize2Ix(instruction);
      }
      case 9: {
        return this.parseRaydiumSwapIn(instruction);
      }
      case 11: {
        return this.parseRaydiumSwapOut(instruction);
      }
      default:
        return this.parseUnknownInstruction(instruction);
    }
  }
  private parseRaydiumSwapIn(instruction: TransactionInstruction){
    const accounts = instruction.keys;
    const instructionData = instruction.data;
    const args = SwapBaseInLayout.decode(instructionData);
    return {
      name : "swapIn",
     args: {
        minimum_in : args.minimumOut,
        swapIn: args.amountIn,
        swapOut: args.outAmount,
      },
      programId: instruction.programId,
    };
  }
  private parseRaydiumSwapOut(instruction: TransactionInstruction){
    const accounts = instruction.keys;
    const instructionData = instruction.data;
    const args = SwapBaseOutLayout.decode(instructionData);
    return {
      name : "swapOut",
     args: {
        maximum_in : args.maxIn,
        swapIn: args.directIn,
        swapOut: args.amountOut,
        poolCoinAccount : args.poolCoinTokenAccount
      },
      programId: instruction.programId,
    };
  }
  private parseRaydiumInitializeIx(instruction: TransactionInstruction) {
    const accounts = instruction.keys;
    const instructionData = instruction.data;
    const args = RaydiumInitializeArgsLayout.decode(instructionData);
    return {
      name: "raydiumInitialize",
      args: {
        amm: accounts[3].pubkey.toBase58(),
        amm_authority: accounts[4].pubkey.toBase58(),
        amm_open_orders: accounts[5].pubkey.toBase58(),
        lp_mint_address: accounts[6].pubkey.toBase58(),
        coin_mint_address: accounts[7].pubkey.toBase58(),
        pc_mint_address: accounts[8].pubkey.toBase58(),
        pool_coin_token_account: accounts[9].pubkey.toBase58(),
        pool_pc_token_account: accounts[10].pubkey.toBase58(),
        pool_withdraw_queue: accounts[11].pubkey.toBase58(),
        pool_target_orders_account: accounts[12].pubkey.toBase58(),
        user_lp_token_account: accounts[13].pubkey.toBase58(),
        pool_temp_lp_token_account: accounts[14].pubkey.toBase58(),
        serum_market: accounts[16].pubkey.toBase58(),
        user_wallet: accounts[17].pubkey.toBase58(),
        nonce: args.nonce,
        openTime: Number(args.openTime),
      },
      programId: instruction.programId,
    };
  }

  private parseRaydiumInitialize2Ix(instruction: TransactionInstruction) {
    const accounts = instruction.keys;
    const instructionData = instruction.data;
    const args = RaydiumInitialize2ArgsLayout.decode(instructionData);
    return {
      name: "raydiumInitialize2",
      args: {
        amm: accounts[4].pubkey.toBase58(),
        amm_authority: accounts[5].pubkey.toBase58(),
        amm_open_orders: accounts[6].pubkey.toBase58(),
        lp_mint_address: accounts[7].pubkey.toBase58(),
        coin_mint_address: accounts[8].pubkey.toBase58(),
        pc_mint_address: accounts[9].pubkey.toBase58(),
        pool_coin_token_account: accounts[10].pubkey.toBase58(),
        pool_pc_token_account: accounts[11].pubkey.toBase58(),
        pool_withdraw_queue: accounts[12].pubkey.toBase58(),
        amm_target_orders: accounts[13].pubkey.toBase58(),
        pool_temp_lp_token_account: accounts[14].pubkey.toBase58(),
        serum_market: accounts[16].pubkey.toBase58(),
        user_wallet: accounts[17].pubkey.toBase58(),
        user_token_coin: accounts[18].pubkey.toBase58(),
        user_token_pc: accounts[19].pubkey.toBase58(),
        user_lp_token_account: accounts[20].pubkey.toBase58(),
        nonce: args.nonce,
        openTime: Number(args.openTime),
        initPcAmount: Number(args.initPcAmount),
        initCoinAmount: Number(args.initCoinAmount),
      },
      programId: instruction.programId,
    };
  }

  private parseUnknownInstruction(instruction: TransactionInstruction) {
    return {
      name: "Unknown",
      args: { unknown: base58.encode(instruction.data) },
      programId: instruction.programId,
    };
  }
}
