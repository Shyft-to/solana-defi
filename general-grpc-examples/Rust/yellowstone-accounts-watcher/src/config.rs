use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Commitment {
    Confirmed,
    Finalized,
}

impl Commitment {
    fn parse(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "confirmed" => Commitment::Confirmed,
            _ => Commitment::Finalized,
        }
    }
}

impl std::fmt::Display for Commitment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Commitment::Confirmed => write!(f, "confirmed"),
            Commitment::Finalized => write!(f, "finalized"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Config {
    pub grpc_endpoint: String,
    pub grpc_x_token: Option<String>,
    pub rpc_endpoint: String,
    pub target_pubkey: String,
    /// Commitment level for the Yellowstone gRPC account-update subscription.
    /// Defaults to `confirmed` (lower latency). Does not affect slot trigger.
    pub grpc_commitment: Commitment,
    /// Commitment level for JSON-RPC calls (getAccountInfo).
    /// Defaults to `finalized` (safe, no rollback risk).
    pub rpc_commitment: Commitment,
    /// How many finalized slots of gRPC account data to keep in memory.
    /// Must be larger than the confirmed→finalized gap (~32 slots) plus some buffer.
    /// Defaults to 300.
    pub grpc_data_retain_slots: u64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let target_pubkey = require("TARGET_PUBKEY")?;
        if target_pubkey.is_empty() {
            return Err(anyhow::anyhow!("`TARGET_PUBKEY` must not be empty"));
        }
        let grpc_commitment = env::var("GRPC_COMMITMENT")
            .map(|s| Commitment::parse(&s))
            .unwrap_or(Commitment::Confirmed);
        let rpc_commitment = env::var("RPC_COMMITMENT")
            .map(|s| Commitment::parse(&s))
            .unwrap_or(Commitment::Finalized);
        let grpc_data_retain_slots = env::var("GRPC_DATA_RETAIN_SLOTS")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(300u64);
        Ok(Self {
            grpc_endpoint: require("GRPC_ENDPOINT")?,
            grpc_x_token: env::var("GRPC_X_TOKEN").ok(),
            rpc_endpoint: require("RPC_ENDPOINT")?,
            target_pubkey,
            grpc_commitment,
            rpc_commitment,
            grpc_data_retain_slots,
        })
    }
}

fn require(key: &str) -> Result<String> {
    env::var(key).with_context(|| format!("`{key}` env var is required"))
}
