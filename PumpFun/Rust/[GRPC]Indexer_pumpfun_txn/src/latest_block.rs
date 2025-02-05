use solana_client::rpc_client::RpcClient;
use solana_client::client_error::ClientError;
use anyhow::Context;


pub fn get_latest_slot() ->  anyhow::Result<u64>{
    let rpc_url = "https://rpc.va.shyft.to?api_key=YMyDOr87OBzT6TWr"; 
    let client = RpcClient::new(rpc_url.to_string());

    client.get_slot().context("Failed to fetch latest slot")

}
