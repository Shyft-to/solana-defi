use anyhow::{anyhow, Result};

pub struct Config {
    pub grpc_endpoint: String,
    pub grpc_x_token: Option<String>,
    pub slack_webhook_url: Option<String>,
    /// Solana JSON-RPC HTTP endpoint used for `getBlock` latency checks.
    /// When unset, block-latency measurement is disabled.
    pub rpc_url: Option<String>,
    /// How many slots to wait after a block arrives before calling `getBlock`,
    /// giving the RPC node time to index the slot. Converted to a delay at ~450ms/slot.
    pub latency_lag_slots: u64,
    /// Measure latency for every Nth block (1 = every block). Higher values reduce
    /// `getBlock` load / rate-limit pressure.
    pub latency_sample_every: u64,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let grpc_endpoint =
            std::env::var("GRPC_ENDPOINT").map_err(|_| anyhow!("GRPC_ENDPOINT is required"))?;

        let grpc_x_token = std::env::var("GRPC_X_TOKEN").ok();

        let slack_webhook_url = std::env::var("SLACK_WEBHOOK_URL").ok();

        let rpc_url = std::env::var("RPC_URL").ok().filter(|s| !s.is_empty());

        let latency_lag_slots = std::env::var("LATENCY_LAG_SLOTS")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(32u64);

        let latency_sample_every = std::env::var("LATENCY_SAMPLE_EVERY")
            .ok()
            .and_then(|v| v.parse().ok())
            .filter(|&n| n > 0)
            .unwrap_or(1u64);

        Ok(Self {
            grpc_endpoint,
            grpc_x_token,
            slack_webhook_url,
            rpc_url,
            latency_lag_slots,
            latency_sample_every,
        })
    }
}
