use solana_client::rpc_client::RpcClient;
use solana_client::client_error::ClientError;
use anyhow::Context;
use solana_program::pubkey::Pubkey;
use std::str::FromStr;

const API_KEY: &str = "API_KEY";

pub fn get_sol_balance(token: &str) -> u64{
    let rpc_url = format!("https://rpc.va.shyft.to?api_key={}",API_KEY); 
    let client = RpcClient::new(rpc_url);
    client.get_account(&Pubkey::from_str(token).unwrap()).unwrap().lamports / 1000000000
}
