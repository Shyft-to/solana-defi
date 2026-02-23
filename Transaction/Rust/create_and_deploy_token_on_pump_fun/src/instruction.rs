use solana_program::{
    instruction::{AccountMeta, Instruction},
    pubkey::Pubkey,
    system_program,
    sysvar,
};
use spl_associated_token_account::{
    get_associated_token_address,
    get_associated_token_address_with_program_id,
};
use crate::{
    models::Network,
    constants::*,
    error::Result,
};
use std::str::FromStr;

const TOKEN_2022_PROGRAM_ID: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
const ASSOCIATED_TOKEN_PROGRAM_ID: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";
const MPL_TOKEN_METADATA_PROGRAM_ID: &str = "metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s";
const MAYHEM_PROGRAM_ID: &str = "MAyhSmzXzV1pTf7LsNkrNwkWKTo4ougAJ1PPg47MD4e";

const CREATE_DISCRIMINATOR: [u8; 8] = [24, 30, 200, 40, 5, 28, 7, 119];
const CREATE_V2_DISCRIMINATOR: [u8; 8] = [0xd6, 0x90, 0x4c, 0xec, 0x5f, 0x8b, 0x31, 0xb4];

pub struct PumpInstructionBuilder {
    pub program_id: Pubkey,
    token_2022_program_id: Pubkey,
    associated_token_program_id: Pubkey,
    mpl_token_metadata_program_id: Pubkey,
    mayhem_program_id: Pubkey,
}

impl PumpInstructionBuilder {
    pub fn new(network: Network) -> Self {
        println!("🔧 Creating PumpInstructionBuilder for {:?}", network);

        let program_id = Pubkey::from_str(PUMP_FUN_PROGRAM)
            .expect("Invalid program ID");

        let token_2022_program_id = Pubkey::from_str(TOKEN_2022_PROGRAM_ID)
            .expect("Invalid Token 2022 program ID");

        let associated_token_program_id = Pubkey::from_str(ASSOCIATED_TOKEN_PROGRAM_ID)
            .expect("Invalid Associated Token program ID");

        let mpl_token_metadata_program_id = Pubkey::from_str(MPL_TOKEN_METADATA_PROGRAM_ID)
            .expect("Invalid MPL Token Metadata program ID");

        let mayhem_program_id = Pubkey::from_str(MAYHEM_PROGRAM_ID)
            .expect("Invalid Mayhem program ID");

        println!("   Program ID: {}", program_id);
        println!("   Token 2022 Program ID: {}", token_2022_program_id);
        println!("   Associated Token Program ID: {}", associated_token_program_id);
        println!("   MPL Token Metadata Program ID: {}", mpl_token_metadata_program_id);
        println!("   Mayhem Program ID: {}", mayhem_program_id);

        Self {
            program_id,
            token_2022_program_id,
            associated_token_program_id,
            mpl_token_metadata_program_id,
            mayhem_program_id,
        }
    }


    pub fn get_mint_authority_pda(&self) -> Pubkey {
        Pubkey::find_program_address(&[b"mint-authority"], &self.program_id).0
    }

    pub fn get_global_pda(&self) -> Pubkey {
        Pubkey::find_program_address(&[b"global"], &self.program_id).0
    }

    pub fn get_bonding_curve_pda(&self, mint: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[b"bonding-curve", mint.as_ref()],
            &self.program_id,
        ).0
    }

    pub fn get_event_authority_pda(&self) -> Pubkey {
        Pubkey::find_program_address(&[b"__event_authority"], &self.program_id).0
    }

    pub fn get_metadata_pda(&self, mint: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[
                b"metadata",
                self.mpl_token_metadata_program_id.as_ref(),
                mint.as_ref(),
            ],
            &self.mpl_token_metadata_program_id,
        ).0
    }

    fn get_global_params_pda(&self) -> Pubkey {
        Pubkey::find_program_address(&[b"global-params"], &self.mayhem_program_id).0
    }

    fn get_sol_vault_pda(&self) -> Pubkey {
        Pubkey::find_program_address(&[b"sol-vault"], &self.mayhem_program_id).0
    }

    fn get_mayhem_state_pda(&self, mint: &Pubkey) -> Pubkey {
        Pubkey::find_program_address(
            &[b"mayhem-state", mint.as_ref()],
            &self.mayhem_program_id,
        ).0
    }

    pub fn build_create_instruction(
        &self,
        creator: &Pubkey,
        mint: &Pubkey,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<Instruction> {
        println!("\n🔧 Building original create instruction...");
        println!("   Mint (new keypair): {}", mint);

        let bonding_curve = self.get_bonding_curve_pda(mint);
        println!("   Bonding Curve: {}", bonding_curve);

        // Original create uses legacy SPL token ATA
        let associated_bonding_curve = get_associated_token_address(&bonding_curve, mint);
        println!("   Associated Bonding Curve: {}", associated_bonding_curve);

        let global = self.get_global_pda();
        println!("   Global: {}", global);

        let metadata = self.get_metadata_pda(mint);
        println!("   Metadata: {}", metadata);

        let mint_authority = self.get_mint_authority_pda();
        println!("   Mint Authority: {}", mint_authority);

        let event_authority = self.get_event_authority_pda();
        println!("   Event Authority: {}", event_authority);

        let mut data = Vec::new();
        data.extend_from_slice(&CREATE_DISCRIMINATOR);

        let name_bytes = name.as_bytes();
        data.extend_from_slice(&(name_bytes.len() as u32).to_le_bytes());
        data.extend_from_slice(name_bytes);
        println!("   Name: '{}' ({} bytes)", name, name_bytes.len());

        let symbol_bytes = symbol.as_bytes();
        data.extend_from_slice(&(symbol_bytes.len() as u32).to_le_bytes());
        data.extend_from_slice(symbol_bytes);
        println!("   Symbol: '{}' ({} bytes)", symbol, symbol_bytes.len());

        let uri_bytes = uri.as_bytes();
        data.extend_from_slice(&(uri_bytes.len() as u32).to_le_bytes());
        data.extend_from_slice(uri_bytes);
        println!("   URI: {} ({} bytes)", uri, uri_bytes.len());

        data.extend_from_slice(creator.as_ref());
        println!("   Creator: {}", creator);
        println!("   Total data length: {} bytes", data.len());

        Ok(Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(*mint, true),
                AccountMeta::new_readonly(mint_authority, false),
                AccountMeta::new(bonding_curve, false),
                AccountMeta::new(associated_bonding_curve, false),
                AccountMeta::new_readonly(global, false),
                AccountMeta::new_readonly(self.mpl_token_metadata_program_id, false),
                AccountMeta::new(metadata, false),
                AccountMeta::new(*creator, true),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(accounts::TOKEN_PROGRAM, false),
                AccountMeta::new_readonly(self.associated_token_program_id, false),
                AccountMeta::new_readonly(sysvar::rent::id(), false),
                AccountMeta::new_readonly(event_authority, false),
                AccountMeta::new_readonly(self.program_id, false),
            ],
            data,
        })
    }

    pub fn build_create_v2_instruction(
        &self,
        creator: &Pubkey,
        mint: &Pubkey,
        name: String,
        symbol: String,
        uri: String,
        is_mayhem_mode: bool,
        is_cashback_enabled: bool,
    ) -> Result<Instruction> {
        println!("\n🔧 Building create_v2 instruction...");
        println!("   Mint (new keypair): {}", mint);

        let (bonding_curve, _curve_bump) = Pubkey::find_program_address(
            &[b"bonding-curve", mint.as_ref()],
            &self.program_id,
        );
        println!("   Bonding Curve: {}", bonding_curve);

        let associated_bonding_curve = get_associated_token_address_with_program_id(
            &bonding_curve,
            mint,
            &self.token_2022_program_id,
        );
        println!("   Associated Bonding Curve: {}", associated_bonding_curve);

        let global = self.get_global_pda();
        println!("   Global: {}", global);

        let mint_authority = self.get_mint_authority_pda();
        println!("   Mint Authority: {}", mint_authority);

        let event_authority = self.get_event_authority_pda();
        println!("   Event Authority: {}", event_authority);

        let global_params = self.get_global_params_pda();
        println!("   Global Params: {}", global_params);

        let sol_vault = self.get_sol_vault_pda();
        println!("   Sol Vault: {}", sol_vault);

        let mayhem_state = self.get_mayhem_state_pda(mint);
        println!("   Mayhem State: {}", mayhem_state);

        let mayhem_token_vault = get_associated_token_address_with_program_id(
            &sol_vault,
            mint,
            &self.token_2022_program_id,
        );
        println!("   Mayhem Token Vault: {}", mayhem_token_vault);

        let mut data = Vec::new();
        data.extend_from_slice(&CREATE_V2_DISCRIMINATOR);

        let name_bytes = name.as_bytes();
        data.extend_from_slice(&(name_bytes.len() as u32).to_le_bytes());
        data.extend_from_slice(name_bytes);
        println!("   Name: '{}' ({} bytes)", name, name_bytes.len());

        let symbol_bytes = symbol.as_bytes();
        data.extend_from_slice(&(symbol_bytes.len() as u32).to_le_bytes());
        data.extend_from_slice(symbol_bytes);
        println!("   Symbol: '{}' ({} bytes)", symbol, symbol_bytes.len());

        let uri_bytes = uri.as_bytes();
        data.extend_from_slice(&(uri_bytes.len() as u32).to_le_bytes());
        data.extend_from_slice(uri_bytes);
        println!("   URI: {} ({} bytes)", uri, uri_bytes.len());

        data.extend_from_slice(creator.as_ref());
        println!("   Creator: {}", creator);

        data.push(if is_mayhem_mode { 1 } else { 0 });
        println!("   Mayhem mode: {}", is_mayhem_mode);

        data.push(if is_cashback_enabled { 2 } else { 0 });
        println!("   Cashback enabled: {}", is_cashback_enabled);

        println!("   Total data length: {} bytes", data.len());

        Ok(Instruction {
            program_id: self.program_id,
            accounts: vec![
                AccountMeta::new(*mint, true),
                AccountMeta::new_readonly(mint_authority, false),
                AccountMeta::new(bonding_curve, false),
                AccountMeta::new(associated_bonding_curve, false),
                AccountMeta::new_readonly(global, false),
                AccountMeta::new(*creator, true),
                AccountMeta::new_readonly(system_program::id(), false),
                AccountMeta::new_readonly(self.token_2022_program_id, false),
                AccountMeta::new_readonly(self.associated_token_program_id, false),
                AccountMeta::new(self.mayhem_program_id, false),
                AccountMeta::new_readonly(global_params, false),
                AccountMeta::new(sol_vault, false),
                AccountMeta::new(mayhem_state, false),
                AccountMeta::new(mayhem_token_vault, false),
                AccountMeta::new_readonly(event_authority, false),
                AccountMeta::new_readonly(self.program_id, false),
            ],
            data,
        })
    }
}

impl Default for PumpInstructionBuilder {
    fn default() -> Self {
        Self::new(Network::Mainnet)
    }
}