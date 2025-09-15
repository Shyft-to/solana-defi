import { PublicKey } from "@solana/web3.js";
import { decodeInstructionArgs } from "./layout-args";

export interface AccountMeta {
  name: string;
  isSigner: boolean;
  isWritable: boolean;
  pubkey: string;
}

export interface InstructionDiscriminator {
  name: string;
  discriminator: number[];
}

export interface InstructionArg {
  name: string;
  type: string | { defined: string };
}

export interface Instruction {
  name: string;
  docs?: string[];
  discriminator: number[];
  accounts: {
    name: string;
    isSigner?: boolean;
    isWritable?: boolean;
    pda?: any;
    address?: string;
  }[];
  args: InstructionArg[];
}

export interface PumpFunIDL {
  instructions: Instruction[];
  events: any[],
}

export const PUMP_FUN_IDL: PumpFunIDL = {
  instructions: [
    {
      name: "buy",
      docs: ["Buys tokens from a bonding curve."],
      discriminator: [102, 6, 61, 18, 1, 218, 235, 234],
      accounts: [
        { name: "global" },
        { name: "fee_recipient", isWritable: true },
        { name: "mint" },
        { name: "bonding_curve", isWritable: true },
        { name: "associated_bonding_curve", isWritable: true },
        { name: "associated_user", isWritable: true },
        { name: "user", isWritable: true, isSigner: true },
        { name: "system_program", address: "11111111111111111111111111111111" },
        { name: "token_program", address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" },
        { name: "creator_vault", isWritable: true },
        { name: "event_authority" },
        { name: "program", address: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P" },
        { name: "global_volume_accumulator", isWritable: true },
        { name: "user_volume_accumulator", isWritable: true },
        { name: "fee_config" },
        { name: "fee_program", address: "pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ" }
      ],
      args: [
        { name: "amount", type: "u64" },
        { name: "max_sol_cost", type: "u64" },
        { name: "track_volume", type: { defined: "OptionBool" } }
      ]
    },
    {
      name: "sell",
      docs: ["Sells tokens into a bonding curve."],
      discriminator: [51, 230, 133, 164, 1, 127, 131, 173],
      accounts: [
        { name: "global" },
        { name: "fee_recipient", isWritable: true },
        { name: "mint" },
        { name: "bonding_curve", isWritable: true },
        { name: "associated_bonding_curve", isWritable: true },
        { name: "associated_user", isWritable: true },
        { name: "user", isWritable: true, isSigner: true },
        { name: "system_program", address: "11111111111111111111111111111111" },
        { name: "creator_vault", isWritable: true },
        { name: "token_program", address: "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA" },
        { name: "event_authority" },
        { name: "program", address: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P" },
        { name: "fee_config" },
        { name: "fee_program", address: "pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ" }
      ],
      args: [
        { name: "amount", type: "u64" },
        { name: "min_sol_output", type: "u64" }
      ]
    },
  ],
  events: []
};
export const KNOWN_ADDRESSES = {
  SYSTEM_PROGRAM: '11111111111111111111111111111111',
  TOKEN_PROGRAM: 'TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA',
  PUMP_FUN_PROGRAM: '6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P',
  FEE_PROGRAM: 'pfeeUxB6jkeY1Hxd7CsFCAjcbHA9rWtchMGdZ6VojVZ'
};
export interface ParsedInstruction {
  name: string;
  accounts: AccountMeta[]; 
  args?: any;
  discriminator: number[];
}
export function mapAccounts(raw: any[], _args: any): ParsedInstruction | null {
  try {
    const normalizedAccounts = raw.map(acc => ({
      isSigner: acc.isSigner,
      isWritable: acc.isWritable,
      pubkey: typeof acc.pubkey === 'string' ? acc.pubkey : acc.pubkey.toString()
    }));

    const instruction = identifyInstruction(normalizedAccounts);
    
    if (!instruction) {
      return null;
    }

    const mappedAccounts = mapAccountsToInstruction(instruction, normalizedAccounts);
    
    if (!mappedAccounts) {
      return null;
    }
    const decodedArgs = decodeInstructionArgs(_args)

    return {
      name: instruction.name,
      accounts: mappedAccounts, 
      args: decodedArgs,
      discriminator: instruction.discriminator
    };

  } catch (error) {
    console.error("Error mapping accounts:", error);
    return null;
  }
}
function identifyInstruction(accounts: any[]): Instruction | null {
  if (accounts.length === 16) {
    if (accounts[7].pubkey === KNOWN_ADDRESSES.SYSTEM_PROGRAM && 
        accounts[8].pubkey === KNOWN_ADDRESSES.TOKEN_PROGRAM) {
      return PUMP_FUN_IDL.instructions.find(ix => ix.name === "buy") || null;
    }
  } else if (accounts.length === 14) {
    if (accounts[7].pubkey === KNOWN_ADDRESSES.SYSTEM_PROGRAM && 
        accounts[9].pubkey === KNOWN_ADDRESSES.TOKEN_PROGRAM) {
      return PUMP_FUN_IDL.instructions.find(ix => ix.name === "sell") || null;
    }
  }
  return null;
}

function mapAccountsToInstruction(instruction: Instruction, accounts: any[]): AccountMeta[] | null {
  const mapped: AccountMeta[] = [];
  
  instruction.accounts.forEach((accountDef, index) => {
    if (index >= accounts.length) return;
    
    const accountData = accounts[index];
    mapped.push({
      name: accountDef.name,
      isSigner: accountDef.isSigner || accountData.isSigner,
      isWritable: accountDef.isWritable || accountData.isWritable,
      pubkey: accountData.pubkey
    });
  });

  return mapped;
}
