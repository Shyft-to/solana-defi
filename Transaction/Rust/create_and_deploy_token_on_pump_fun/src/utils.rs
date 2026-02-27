use solana_sdk::pubkey::Pubkey;
use solana_sdk::signature::Keypair;
use std::str::FromStr;
use dotenv::dotenv;
use std::env;

pub fn load_keypair_from_env(var_name: &str) -> Option<Keypair> {
    dotenv().ok();
    
    if let Ok(private_key_str) = env::var(var_name) {
        if let Ok(private_key_bytes) = bs58::decode(&private_key_str).into_vec() {
            if private_key_bytes.len() == 64 {
                if let Ok(keypair) = Keypair::from_bytes(&private_key_bytes) {
                    return Some(keypair);
                }
            }
        }
        
        if private_key_str.starts_with('[') && private_key_str.ends_with(']') {
            let numbers: Vec<u8> = private_key_str
                .trim_start_matches('[')
                .trim_end_matches(']')
                .split(',')
                .filter_map(|s| s.trim().parse().ok())
                .collect();
            
            if numbers.len() == 64 {
                if let Ok(keypair) = Keypair::from_bytes(&numbers) {
                    return Some(keypair);
                }
            }
        }
        
        if let Ok(keypair) = read_keypair_from_file(&private_key_str) {
            return Some(keypair);
        }
    }
    
    None
}

pub fn load_rpc_url_from_env(var_name: &str) -> String {
    dotenv().ok();
    env::var(var_name).unwrap_or_else(|_| "https://api.devnet.solana.com".to_string())
}

pub fn load_rpc_url_with_network(var_name: &str, network: &crate::models::Network) -> String {
    dotenv().ok();
    
    match env::var(var_name) {
        Ok(url) => url,
        Err(_) => match network {
            crate::models::Network::Devnet => "https://api.devnet.solana.com".to_string(),
            crate::models::Network::Mainnet => "https://api.mainnet-beta.solana.com".to_string(),
            crate::models::Network::Localnet => "http://localhost:8899".to_string(),
        }
    }
}

fn read_keypair_from_file(path: &str) -> Result<Keypair, Box<dyn std::error::Error>> {
    let data = std::fs::read_to_string(path)?;
    let bytes: Vec<u8> = serde_json::from_str(&data)?;
    Ok(Keypair::from_bytes(&bytes)?)
}

pub fn parse_pubkey(s: &str) -> Result<Pubkey, String> {
    Pubkey::from_str(s).map_err(|e| e.to_string())
}

pub fn lamports_to_sol(lamports: u64) -> f64 {
    lamports as f64 / 1e9
}

pub fn sol_to_lamports(sol: f64) -> u64 {
    (sol * 1e9) as u64
}

pub fn wait_for_enter() {
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
}

pub fn get_explorer_url(signature: &str, network: &crate::models::Network) -> String {
    match network {
        crate::models::Network::Devnet => format!("https://explorer.solana.com/tx/{}?cluster=devnet", signature),
        crate::models::Network::Mainnet => format!("https://explorer.solana.com/tx/{}", signature),
        crate::models::Network::Localnet => format!("https://explorer.solana.com/tx/{}?cluster=custom", signature),
    }
}

pub fn format_number(num: f64) -> String {
    let mut parts = Vec::new();
    let num_str = format!("{:.2}", num);
    let num_parts: Vec<&str> = num_str.split('.').collect();
    let integer_part = num_parts[0];
    let decimal_part = if num_parts.len() > 1 { num_parts[1] } else { "00" };
    
    for (i, c) in integer_part.chars().rev().enumerate() {
        if i > 0 && i % 3 == 0 {
            parts.push(',');
        }
        parts.push(c);
    }
    
    parts.reverse();
    let formatted_integer: String = parts.into_iter().collect();
    
    format!("{}.{}", formatted_integer, decimal_part)
}

pub fn is_valid_pubkey(s: &str) -> bool {
    Pubkey::from_str(s).is_ok()
}

pub fn current_timestamp() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs() as i64
}