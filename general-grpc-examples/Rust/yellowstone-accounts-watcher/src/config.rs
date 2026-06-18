use anyhow::{Context, Result};
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub grpc_endpoint: String,
    pub grpc_x_token: Option<String>,
    /// Named account groups. Each non-empty group gets its own gRPC stream.
    pub streams: Vec<StreamGroup>,
}

#[derive(Debug, Clone)]
pub struct StreamGroup {
    pub name: String,
    pub accounts: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let grpc_endpoint = env_require("GRPC_ENDPOINT")?;
        let grpc_x_token = env::var("GRPC_X_TOKEN").ok();

        let mut streams = Vec::new();
        for i in 1..=3 {
            let key = format!("ACCOUNT_INCLUDE_{i}");
            if let Ok(val) = env::var(&key) {
                let accounts: Vec<String> = val
                    .split(',')
                    .map(|s| s.trim().to_owned())
                    .filter(|s| !s.is_empty())
                    .collect();
                if !accounts.is_empty() {
                    streams.push(StreamGroup {
                        name: format!("stream_{i}"),
                        accounts,
                    });
                }
            }
        }

        if streams.is_empty() {
            return Err(anyhow::anyhow!(
                "No accounts configured — set at least one of ACCOUNT_INCLUDE_1, ACCOUNT_INCLUDE_2, ACCOUNT_INCLUDE_3"
            ));
        }

        Ok(Self {
            grpc_endpoint,
            grpc_x_token,
            streams,
        })
    }
}

fn env_require(key: &str) -> Result<String> {
    env::var(key).with_context(|| format!("environment variable `{key}` is required"))
}
