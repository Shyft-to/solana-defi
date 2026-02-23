use solana_client::rpc_client::RpcClient;
use solana_sdk::{
    commitment_config::CommitmentConfig,
    pubkey::Pubkey,
    signature::{Keypair, Signature, Signer},
    transaction::Transaction,
    compute_budget::ComputeBudgetInstruction,
    message::Message,
};
use spl_associated_token_account::get_associated_token_address;
use crate::{
    models::*,
    instruction::PumpInstructionBuilder,
    error::{Result, PumpFunError},
    constants::*,
    utils::{lamports_to_sol, get_explorer_url, load_rpc_url_with_network},
};
use log::info;
use std::time::Duration;
use std::str::FromStr;

const SPL_TOKEN_2022_PROGRAM_ID: &str = "TokenzQdBNbLqP5VEhdkAS6EPFLC1PHnBqCXEpPxuEb";
const SPL_ASSOCIATED_TOKEN_PROGRAM_ID: &str = "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL";

pub struct PumpFunClient {
    pub rpc_client: RpcClient,
    pub instruction_builder: PumpInstructionBuilder,
    pub network: Network,
    pub token_2022_program_id: Pubkey,
    pub associated_token_program_id: Pubkey,
}

impl PumpFunClient {
    pub fn new(rpc_url: &str, network: Network) -> Self {
        println!("🔧 Creating new PumpFunClient");
        println!("   Network: {:?}", network);
        println!("   RPC URL: {}", rpc_url);

        let token_2022_program_id = Pubkey::from_str(SPL_TOKEN_2022_PROGRAM_ID).unwrap();
        let associated_token_program_id = Pubkey::from_str(SPL_ASSOCIATED_TOKEN_PROGRAM_ID).unwrap();

        println!("   Token 2022 Program ID: {}", token_2022_program_id);
        println!("   Associated Token Program ID: {}", associated_token_program_id);

        Self {
            rpc_client: RpcClient::new_with_commitment(
                rpc_url.to_string(),
                CommitmentConfig::confirmed(),
            ),
            instruction_builder: PumpInstructionBuilder::new(network),
            network,
            token_2022_program_id,
            associated_token_program_id,
        }
    }

    pub fn devnet() -> Self {
        println!("🌐 Creating Devnet client");
        let network = Network::Devnet;
        let rpc_url = load_rpc_url_with_network("RPC_URL", &network);
        Self::new(&rpc_url, network)
    }

    pub fn mainnet() -> Self {
        println!("🌐 Creating Mainnet client");
        let network = Network::Mainnet;
        let rpc_url = load_rpc_url_with_network("RPC_URL", &network);
        Self::new(&rpc_url, network)
    }

    pub async fn check_balance(&self, wallet: &Pubkey) -> Result<f64> {
        println!("💰 Checking balance for wallet: {}", wallet);
        let balance = self.rpc_client.get_balance(wallet)?;
        let sol_balance = lamports_to_sol(balance);
        println!("   Balance: {} SOL ({} lamports)", sol_balance, balance);
        info!("💰 Wallet balance: {} SOL", sol_balance);
        Ok(sol_balance)
    }

    // ==================== CREATE (ORIGINAL) ====================

    pub async fn create_token(
        &self,
        creator_keypair: &Keypair,
        name: String,
        symbol: String,
        uri: String,
    ) -> Result<Pubkey> {
        println!("\n🪙 Creating token with original create...");
        println!("   Name: {}", name);
        println!("   Symbol: {}", symbol);
        println!("   URI: {}", uri);
        println!("   Creator: {}", creator_keypair.pubkey());

        let mint_keypair = Keypair::new();
        println!("   Mint (new keypair): {}", mint_keypair.pubkey());

        let create_ix = self.instruction_builder.build_create_instruction(
            &creator_keypair.pubkey(),
            &mint_keypair.pubkey(),
            name,
            symbol,
            uri,
        )?;

        let config = SwapConfig {
            priority_fee_microlamports: Some(5000),
            compute_unit_limit: Some(400_000),
            network: self.network,
            slippage_bps: 0,
            use_jito: false,
        };

        let result = self.send_transaction_with_signers(
            &[creator_keypair, &mint_keypair],
            vec![create_ix],
            &config,
        ).await?;

        let mint = mint_keypair.pubkey();

        println!("\n✅ Token created successfully!");
        println!("   Mint: {}", mint);
        println!("   Signature: {}", result.signature);
        println!("   Explorer: {}", result.explorer_url);

        let bonding_curve = Pubkey::find_program_address(
            &[b"bonding-curve", mint.as_ref()],
            &self.instruction_builder.program_id,
        ).0;

        println!("\n📝 Save these values:");
        println!("   MINT={}", mint);
        println!("   BONDING_CURVE={}", bonding_curve);

        Ok(mint)
    }

    // ==================== CREATE V2 ====================

    pub async fn create_token_v2(
        &self,
        creator_keypair: &Keypair,
        mint_keypair: &Keypair,
        name: String,
        symbol: String,
        uri: String,
        is_mayhem_mode: bool,
        is_cashback_enabled: bool,
    ) -> Result<Pubkey> {
        println!("\n🪙 Creating token with create_v2...");
        println!("   Name: {}", name);
        println!("   Symbol: {}", symbol);
        println!("   URI: {}", uri);
        println!("   Mint (new keypair): {}", mint_keypair.pubkey());
        println!("   Creator: {}", creator_keypair.pubkey());
        println!("   Mayhem mode: {}", is_mayhem_mode);
        println!("   Cashback enabled: {}", is_cashback_enabled);

        let create_ix = self.instruction_builder.build_create_v2_instruction(
            &creator_keypair.pubkey(),
            &mint_keypair.pubkey(),
            name,
            symbol,
            uri,
            is_mayhem_mode,
            is_cashback_enabled,
        )?;

        let config = SwapConfig {
            priority_fee_microlamports: Some(5000),
            compute_unit_limit: Some(400_000),
            network: self.network,
            slippage_bps: 0,
            use_jito: false,
        };

        let result = self.send_transaction_with_signers(
            &[creator_keypair, mint_keypair],
            vec![create_ix],
            &config,
        ).await?;

        let mint = mint_keypair.pubkey();

        println!("\n✅ Token created successfully!");
        println!("   Mint: {}", mint);
        println!("   Signature: {}", result.signature);
        println!("   Explorer: {}", result.explorer_url);

        let bonding_curve = Pubkey::find_program_address(
            &[b"bonding-curve", mint.as_ref()],
            &self.instruction_builder.program_id,
        ).0;

        println!("\n📝 Save these values:");
        println!("   MINT={}", mint);
        println!("   BONDING_CURVE={}", bonding_curve);

        Ok(mint)
    }

    // ==================== POOL INFO ====================

    pub async fn get_pool_info(&self, mint: &Pubkey) -> Result<PumpPoolInfo> {
        println!("\n📊 Fetching pool info for mint: {}", mint);

        let (bonding_curve, _) = Pubkey::find_program_address(
            &[b"bonding-curve", mint.as_ref()],
            &self.instruction_builder.program_id,
        );

        let associated_bonding_curve = get_associated_token_address(&bonding_curve, mint);

        let curve_account = self.rpc_client.get_account(&bonding_curve)?;

        let creator_bytes = &curve_account.data[41..73];
        let creator = Pubkey::try_from(creator_bytes)
            .map_err(|_| PumpFunError::InvalidAccountData("Failed to parse creator".to_string()))?;

        let complete = curve_account.data[8] != 0;
        let virtual_token_reserves = u64::from_le_bytes(curve_account.data[9..17].try_into()?);
        let virtual_sol_reserves = u64::from_le_bytes(curve_account.data[17..25].try_into()?);
        let token_total_supply = u64::from_le_bytes(curve_account.data[25..33].try_into()?);

        println!("✅ Pool found!");
        println!("   Bonding Curve: {}", bonding_curve);
        println!("   Creator: {}", creator);
        println!("   Complete: {}", complete);
        println!("   Virtual SOL: {} SOL", lamports_to_sol(virtual_sol_reserves));
        println!("   Virtual Tokens: {} tokens", lamports_to_sol(virtual_token_reserves));

        Ok(PumpPoolInfo {
            mint: *mint,
            bonding_curve,
            associated_bonding_curve,
            creator,
            virtual_token_reserves,
            virtual_sol_reserves,
            token_total_supply,
            complete,
            created_at: None,
        })
    }

    // ==================== TRANSACTION HELPERS ====================

    async fn send_transaction_with_signers(
        &self,
        signers: &[&Keypair],
        instructions: Vec<solana_sdk::instruction::Instruction>,
        config: &SwapConfig,
    ) -> Result<TransactionResult> {
        println!("\n   🔍 SENDING TRANSACTION");
        println!("   Instructions: {}", instructions.len());
        println!("   Signers: {}", signers.len());

        let mut final_instructions = Vec::new();

        if let Some(limit) = config.compute_unit_limit {
            final_instructions.push(ComputeBudgetInstruction::set_compute_unit_limit(limit));
        }
        if let Some(fee) = config.priority_fee_microlamports {
            final_instructions.push(ComputeBudgetInstruction::set_compute_unit_price(fee));
        }

        final_instructions.extend(instructions);

        let recent_blockhash = self.rpc_client.get_latest_blockhash()?;

        let message = Message::new(&final_instructions, Some(&signers[0].pubkey()));
        let mut transaction = Transaction::new_unsigned(message);
        transaction.sign(signers, recent_blockhash);

        let simulation = self.rpc_client.simulate_transaction(&transaction)?;
        if let Some(err) = simulation.value.err {
            return Err(PumpFunError::TransactionFailed(format!("Simulation failed: {:?}", err)));
        }

        let signature = self.send_with_retry(&transaction).await?;
        self.confirm_transaction(&signature).await?;

        let signature_str = signature.to_string();
        let explorer_url = get_explorer_url(&signature_str, &self.network);
        let slot = self.rpc_client.get_slot()?;

        Ok(TransactionResult {
            signature: signature_str,
            explorer_url,
            slot,
            block_time: None,
            fee: 5000,
        })
    }

    async fn send_with_retry(&self, transaction: &Transaction) -> Result<Signature> {
        let max_retries = 3;
        let mut last_error = None;

        for i in 0..max_retries {
            println!("   Send attempt {}/{}", i + 1, max_retries);
            match self.rpc_client.send_transaction(transaction) {
                Ok(sig) => {
                    println!("   ✅ Send successful");
                    return Ok(sig);
                }
                Err(e) => {
                    println!("   ⚠️ Send failed: {}", e);
                    last_error = Some(e);
                    if i < max_retries - 1 {
                        tokio::time::sleep(Duration::from_millis(500 * (i + 1))).await;
                    }
                }
            }
        }

        Err(PumpFunError::TransactionFailed(format!(
            "Failed after {} retries: {:?}",
            max_retries, last_error
        )))
    }

    async fn confirm_transaction(&self, signature: &Signature) -> Result<()> {
        println!("   Waiting for confirmation...");
        let timeout = Duration::from_secs(30);
        let start = std::time::Instant::now();

        while start.elapsed() < timeout {
            match self.rpc_client.get_signature_status(signature)? {
                Some(status) => {
                    if status.is_ok() {
                        println!("   ✅ Transaction confirmed");
                        return Ok(());
                    } else {
                        return Err(PumpFunError::TransactionFailed(
                            format!("Transaction failed: {:?}", status)
                        ));
                    }
                }
                None => {
                    tokio::time::sleep(Duration::from_millis(500)).await;
                }
            }
        }

        Err(PumpFunError::TransactionTimeout)
    }
}