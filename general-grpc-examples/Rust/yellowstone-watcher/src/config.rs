use anyhow::{Context, Result};
use std::env;

/// All runtime configuration for the watcher.
#[derive(Debug, Clone)]
pub struct Config {
    /// Yellowstone gRPC endpoint, e.g. "https://grpc.example.com"
    pub grpc_endpoint: String,
    /// Optional x-token header value required by some Yellowstone nodes
    pub grpc_x_token: Option<String>,
    /// Solana JSON-RPC endpoint used for reconciliation
    pub rpc_endpoint: String,
    /// Accounts whose transactions we want to observe
    pub account_include: Vec<String>,
    /// How many slots to wait after a block is rooted before reconciling
    pub reconcile_lag_slots: u64,
    /// Maximum number of signatures fetched per `getSignaturesForAddress` page
    pub rpc_signatures_limit: usize,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            grpc_endpoint: env_require("GRPC_ENDPOINT")?,
            grpc_x_token: env::var("GRPC_X_TOKEN").ok(),
            rpc_endpoint: env::var("RPC_ENDPOINT")
                .unwrap_or_else(|_| "https://api.mainnet-beta.solana.com".into()),
            account_include: env_require("ACCOUNT_INCLUDE")?
                .split(',')
                .map(|s| s.trim().to_owned())
                .filter(|s| !s.is_empty())
                .collect(),
            reconcile_lag_slots: env::var("RECONCILE_LAG_SLOTS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(5),
            rpc_signatures_limit: env::var("RPC_SIGNATURES_LIMIT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(1000),
        })
    }
}

fn env_require(key: &str) -> Result<String> {
    env::var(key).with_context(|| format!("environment variable `{key}` is required"))
}
