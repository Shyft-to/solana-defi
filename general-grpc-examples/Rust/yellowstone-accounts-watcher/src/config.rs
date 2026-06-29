use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub grpc_endpoint: String,
    pub grpc_x_token: Option<String>,
    pub rpc_endpoint: String,
    pub target_pubkeys: Vec<String>,
    /// When true, a transaction is only counted as "expected" if the target
    /// account's SOL balance actually changed (pre != post). Transactions where
    /// the pubkey was writable but the balance was unchanged are excluded.
    pub sol_bal_check: bool,
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
        let sol_bal_check = matches!(
            env::var("ENABLE_SOL_BAL_CHECK").as_deref(),
            Ok("1" | "true" | "yes")
        );
        Ok(Self {
            grpc_endpoint: require("GRPC_ENDPOINT")?,
            grpc_x_token: env::var("GRPC_X_TOKEN").ok(),
            rpc_endpoint: require("RPC_ENDPOINT")?,
            target_pubkeys,
            sol_bal_check,
        })
    }
}

fn require(key: &str) -> Result<String> {
    env::var(key).with_context(|| format!("`{key}` env var is required"))
}
