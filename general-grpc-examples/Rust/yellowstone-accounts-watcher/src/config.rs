use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub grpc_endpoint: String,
    pub grpc_x_token: Option<String>,
    pub rpc_endpoint: String,
    pub target_pubkeys: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let raw = require("TARGET_PUBKEYS")?;
        let target_pubkeys: Vec<String> = raw
            .split(',')
            .map(|s| s.trim().to_owned())
            .filter(|s| !s.is_empty())
            .collect();
        if target_pubkeys.is_empty() {
            return Err(anyhow::anyhow!("`TARGET_PUBKEYS` must contain at least one pubkey"));
        }
        Ok(Self {
            grpc_endpoint: require("GRPC_ENDPOINT")?,
            grpc_x_token: env::var("GRPC_X_TOKEN").ok(),
            rpc_endpoint: require("RPC_ENDPOINT")?,
            target_pubkeys,
        })
    }
}

fn require(key: &str) -> Result<String> {
    env::var(key).with_context(|| format!("`{key}` env var is required"))
}
