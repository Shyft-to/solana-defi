use solana_client::rpc_client::RpcClient;
use solana_client::client_error::ClientError;
use anyhow::Context;


const API_KEY: &str = "api_key";

pub fn get_latest_slot() -> anyhow::Result<u64> {
    let rpc_url = format!("https://rpc.va.shyft.to?api_key={}", API_KEY);
    
    let client = RpcClient::new(rpc_url);
    
    client.get_slot().context("Failed to fetch latest slot")
}
