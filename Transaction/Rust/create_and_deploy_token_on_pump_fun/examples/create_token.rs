use pumpfun_new_mint::{
    PumpFunClient,
    Network,
    utils::{load_keypair_from_env, lamports_to_sol},
};
use solana_sdk::{
    signature::{Keypair, Signer},
};
use std::str::FromStr;
use env_logger;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();
    
    println!("🚀 Pump.fun Token Creator Example");
    println!("==================================\n");

    let wallet = load_keypair_from_env("PRIVATE_KEY")
        .expect("❌ Please set PRIVATE_KEY in .env file");
    
    println!("📝 Creator Wallet: {}", wallet.pubkey());

    println!("\nSelect network:");
    println!("1. Devnet (testing)");
    println!("2. Mainnet (real)");
    print!("Choice: ");
    
    let mut network_choice = String::new();
    std::io::stdin().read_line(&mut network_choice)?;
    
    let client = match network_choice.trim() {
        "1" => {
            println!("🌐 Using Devnet");
            PumpFunClient::devnet()
        },
        "2" => {
            println!("🌐 Using Mainnet");
            PumpFunClient::mainnet()
        },
        _ => anyhow::bail!("❌ Invalid network choice"),
    };

    let balance = client.check_balance(&wallet.pubkey()).await?;
    println!("💰 Balance: {} SOL", balance);
    
    if balance < 0.1 {
        anyhow::bail!("❌ Need at least 0.1 SOL for creation");
    }

    println!("\nSelect create method:");
    println!("1. Create_v2");
    println!("2. Create");
    print!("Choice: ");
    
    let mut method_choice = String::new();
    std::io::stdin().read_line(&mut method_choice)?;

    println!("\n📋 Enter token details:");
    
    print!("Token name: ");
    let mut name = String::new();
    std::io::stdin().read_line(&mut name)?;
    let name = name.trim().to_string();
    
    print!("Token symbol: ");
    let mut symbol = String::new();
    std::io::stdin().read_line(&mut symbol)?;
    let symbol = symbol.trim().to_string();
    
    print!("Token URI (metadata): ");
    let mut uri = String::new();
    std::io::stdin().read_line(&mut uri)?;
    let uri = uri.trim().to_string();

    let result = match method_choice.trim() {
        "1" => {
            println!("\n🔑 Generating new mint keypair...");
            let mint = Keypair::new();
            println!("   Mint address: {}", mint.pubkey());
            
            println!("\n📤 Creating token with create_v2...");
           client.create_token_v2(
               &wallet,
               &mint,
               name,
               symbol,
               uri,
               false, 
                false, 
            ).await
        },
        "2" => {
            println!("\n📤 Creating token with original create (PDA mint)...");
            client.create_token(
                &wallet,
                name,
                symbol,
                uri,
            ).await
        },
        _ => anyhow::bail!("❌ Invalid method choice"),
    };

    match result {
        Ok(mint_pubkey) => {
            println!("\n✅✅✅ TOKEN CREATED SUCCESSFULLY! ✅✅✅");
            println!("   Mint: {}", mint_pubkey);
            
            println!("\n⏳ Fetching pool info to verify (waiting 3 seconds)...");
            tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;
            
            match client.get_pool_info(&mint_pubkey).await {
                Ok(pool) => {
                    println!("\n📊 Pool Information:");
                    println!("   Bonding Curve: {}", pool.bonding_curve);
                    println!("   Creator: {}", pool.creator);
                    println!("   Virtual SOL: {} SOL", lamports_to_sol(pool.virtual_sol_reserves));
                    println!("   Virtual Tokens: {} tokens", lamports_to_sol(pool.virtual_token_reserves));
                    println!("   Complete: {}", pool.complete);
                }
                Err(e) => {
                    println!("⚠️  Pool not yet available: {}", e);
                    println!("   Check back in a few seconds on Solscan.");
                }
            }
            
            println!("\n📝 SAVE THIS MINT ADDRESS:");
            println!("   TEST_TOKEN_MINT={}", mint_pubkey);
        }
        Err(e) => {
            println!("\n❌ Failed: {}", e);
        }
    }

    Ok(())
}