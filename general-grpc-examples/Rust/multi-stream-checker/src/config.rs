use anyhow::{bail, Context, Result};
use std::env;
use yellowstone_grpc_proto::prelude::CommitmentLevel;

/// All runtime configuration for the latency checker.
#[derive(Debug, Clone)]
pub struct Config {
    /// Yellowstone gRPC endpoint, e.g. "https://grpc.example.com"
    pub grpc_endpoint: String,
    /// Region label for this deployment, included in startup and disconnect
    /// logs so multi-region runs can be told apart at a glance.
    pub region: String,
    /// Optional x-token header value required by some Yellowstone nodes
    pub grpc_x_token: Option<String>,
    /// Accounts watched by transaction stream 1
    pub account_include_1: Vec<String>,
    /// Accounts watched by transaction stream 2. `None` when
    /// `ACCOUNT_INCLUDE_2` is unset — stream 2 is then disabled entirely: no
    /// connection is opened for it and it is left out of every report.
    pub account_include_2: Option<Vec<String>>,
    /// Commitment level for transaction stream 1
    pub commitment_1: CommitmentLevel,
    /// Commitment level for transaction stream 2
    pub commitment_2: CommitmentLevel,
    /// Commitment level for the blocks_meta stream
    pub commitment_blocks_meta: CommitmentLevel,
    /// How many slots of transactions to hold while waiting for the matching
    /// blocks_meta update. Slots older than this are dropped unresolved.
    pub latency_buffer_slots: u64,
    /// Optional Slack incoming-webhook URL. When set, stream disconnects and
    /// reconnects are posted to Slack in addition to being logged locally.
    pub slack_webhook_url: Option<String>,
    /// When false, no per-transaction latency line is printed at all — only
    /// the periodic `[STATS]` report (if that is also enabled). For anyone
    /// who wants the aggregate view without the per-transaction firehose.
    pub log_transactions: bool,
    /// When true, each latency line includes the transaction signature. Off
    /// shortens the line to slot + timestamps + latency only. Ignored when
    /// `log_transactions` is false.
    pub log_signatures: bool,
    /// How often (seconds) to print p50/p95/p99 latency for each stream, over
    /// the samples recorded since the previous report. `0` disables it.
    pub stats_interval_secs: u64,
    /// When true, print the slot number of every blocks_meta update as it
    /// arrives. Off by default — mainly useful for confirming the
    /// blocks_meta stream is actually advancing.
    pub log_blocks_meta: bool,
    /// How long the blocks_meta stream may go without a single update before
    /// it is treated as stalled and force-reconnected. Unlike the
    /// transaction streams, blocks_meta should never legitimately go quiet
    /// for long, so — unlike them — it gets an idle timeout.
    pub blocks_meta_idle_timeout_secs: u64,
    /// Solana JSON-RPC endpoint polled for `getSlot`, independent of the
    /// Yellowstone gRPC endpoint. `None` when `RPC_URL` is unset — the
    /// poller then never starts.
    pub rpc_url: Option<String>,
    /// How often (seconds) to call `getSlot` against `rpc_url`. `0` (the
    /// default, including when `RPC_POLL_INTERVAL_SECS` is unset) disables
    /// the poller even if `rpc_url` is set.
    pub rpc_poll_interval_secs: u64,
    /// Commitment level passed to the `getSlot` poller (processed | confirmed
    /// | finalized). Defaults to `processed`, matching the gRPC-side
    /// `get_slot` health check — the plain Solana JSON-RPC default of
    /// `finalized` would otherwise report a much older slot.
    pub rpc_commitment: String,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        let account_include_1 = parse_accounts(&env_require("ACCOUNT_INCLUDE_1")?);

        // ACCOUNT_INCLUDE_2 is optional: when unset, stream 2 is disabled —
        // no connection is opened for it and it is left out of every report.
        // When set, it follows the same "at least one account" rule as stream 1.
        let account_include_2 = match env::var("ACCOUNT_INCLUDE_2").ok() {
            Some(raw) => {
                let accounts = parse_accounts(&raw);
                if accounts.is_empty() {
                    bail!("ACCOUNT_INCLUDE_2 must contain at least one account, or be unset to disable stream 2");
                }
                Some(accounts)
            }
            None => None,
        };

        // `COMMITMENT` sets the default for all three streams; each stream can
        // override it, which is what lets you time the same accounts at two
        // different commitment levels against one shared block_time.
        let default_commitment = commitment_from_env("COMMITMENT")?.unwrap_or(CommitmentLevel::Confirmed);

        // An empty account_include on a transactions filter means "every
        // transaction on the chain" — almost never what is intended here, and
        // it would bury the latency output. Fail loudly instead.
        if account_include_1.is_empty() {
            bail!("ACCOUNT_INCLUDE_1 must contain at least one account");
        }

        Ok(Self {
            grpc_endpoint: env_require("GRPC_ENDPOINT")?,
            region: env::var("REGION").unwrap_or_else(|_| "unknown".to_owned()),
            grpc_x_token: env::var("GRPC_X_TOKEN").ok(),
            account_include_1,
            account_include_2,
            commitment_1: commitment_from_env("COMMITMENT_1")?.unwrap_or(default_commitment),
            commitment_2: commitment_from_env("COMMITMENT_2")?.unwrap_or(default_commitment),
            commitment_blocks_meta: commitment_from_env("COMMITMENT_BLOCKS_META")?
                .unwrap_or(default_commitment),
            latency_buffer_slots: env::var("LATENCY_BUFFER_SLOTS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(300),
            slack_webhook_url: env::var("SLACK_WEBHOOK_URL").ok(),
            log_transactions: env::var("LOG_TRANSACTIONS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true),
            log_signatures: env::var("LOG_SIGNATURES")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(true),
            stats_interval_secs: env::var("STATS_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(10),
            log_blocks_meta: env::var("LOG_BLOCKS_META")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(false),
            blocks_meta_idle_timeout_secs: env::var("BLOCKS_META_IDLE_TIMEOUT_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(30),
            rpc_url: env::var("RPC_URL").ok(),
            rpc_poll_interval_secs: env::var("RPC_POLL_INTERVAL_SECS")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(0),
            rpc_commitment: rpc_commitment_from_env("RPC_COMMITMENT", "processed")?,
        })
    }

    /// Whether transaction stream 2 is active, i.e. `ACCOUNT_INCLUDE_2` was set.
    pub fn stream_2_enabled(&self) -> bool {
        self.account_include_2.is_some()
    }

    /// Whether the `getSlot` RPC poller should run, i.e. `RPC_URL` is set
    /// and `RPC_POLL_INTERVAL_SECS` is non-zero.
    pub fn rpc_poller_enabled(&self) -> bool {
        self.rpc_url.is_some() && self.rpc_poll_interval_secs > 0
    }
}

/// Read a commitment level from `key`, or `None` when the variable is unset.
///
/// An unrecognised value is a hard error rather than a silent fall back to the
/// default — a typo'd `COMMITMENT=finalised` would otherwise quietly measure
/// something other than what was asked for.
fn commitment_from_env(key: &str) -> Result<Option<CommitmentLevel>> {
    let raw = match env::var(key) {
        Ok(v) => v,
        Err(_) => return Ok(None),
    };

    let level = match raw.trim().to_ascii_lowercase().as_str() {
        "processed" => CommitmentLevel::Processed,
        "confirmed" => CommitmentLevel::Confirmed,
        "finalized" => CommitmentLevel::Finalized,
        other => bail!("{key}: unknown commitment `{other}` (expected processed|confirmed|finalized)"),
    };

    Ok(Some(level))
}

/// Read a commitment level from `key` for the plain Solana JSON-RPC poller,
/// falling back to `default` when unset. Unlike [`commitment_from_env`], this
/// returns the lowercase string the JSON-RPC `commitment` param expects
/// rather than yellowstone's protobuf enum.
fn rpc_commitment_from_env(key: &str, default: &str) -> Result<String> {
    let raw = match env::var(key) {
        Ok(v) => v,
        Err(_) => return Ok(default.to_owned()),
    };

    match raw.trim().to_ascii_lowercase().as_str() {
        v @ ("processed" | "confirmed" | "finalized") => Ok(v.to_owned()),
        other => bail!("{key}: unknown commitment `{other}` (expected processed|confirmed|finalized)"),
    }
}

/// Split a comma-separated account list, trimming whitespace and dropping blanks.
fn parse_accounts(raw: &str) -> Vec<String> {
    raw.split(',')
        .map(|s| s.trim().to_owned())
        .filter(|s| !s.is_empty())
        .collect()
}

fn env_require(key: &str) -> Result<String> {
    env::var(key).with_context(|| format!("environment variable `{key}` is required"))
}
