use {
    clap::Parser,
    futures::{sink::SinkExt, stream::StreamExt},
    log::{debug, error, info, warn},
    race::RaceTracker,
    std::{
        collections::{HashMap, HashSet},
        sync::{
            atomic::{AtomicU64, Ordering},
            Arc, Mutex,
        },
        time::{Duration, Instant},
    },
    tonic::transport::channel::ClientTlsConfig,
    yellowstone_grpc_client::GeyserGrpcClient,
    yellowstone_grpc_proto::prelude::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
        SubscribeRequestFilterAccounts, SubscribeRequestFilterTransactions,
        SubscribeRequestPing,
    },
};

mod race;
mod slack;

type TxnFilterMap = HashMap<String, SubscribeRequestFilterTransactions>;
type AccountsFilterMap = HashMap<String, SubscribeRequestFilterAccounts>;

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(long, env = "ENDPOINT", help = "gRPC endpoint for the transaction stream (e.g. Rabbitstream)")]
    endpoint: String,

    #[clap(long, env = "X_TOKEN", help = "X-Token for ENDPOINT")]
    x_token: String,

    #[clap(
        long,
        env = "ENDPOINT_NAME",
        help = "Optional display name for ENDPOINT in logs and the summary (defaults to the endpoint URL)"
    )]
    endpoint_name: Option<String>,

    #[clap(
        long,
        env = "ACCOUNTS_ENDPOINT",
        help = "gRPC endpoint for the account-update stream (Rabbitstream itself typically only streams transactions, so this is usually a different provider)"
    )]
    accounts_endpoint: String,

    #[clap(long, env = "ACCOUNTS_X_TOKEN", help = "X-Token for ACCOUNTS_ENDPOINT")]
    accounts_x_token: String,

    #[clap(
        long,
        env = "ACCOUNTS_ENDPOINT_NAME",
        help = "Optional display name for ACCOUNTS_ENDPOINT in logs and the summary (defaults to the endpoint URL)"
    )]
    accounts_endpoint_name: Option<String>,

    #[clap(
        long,
        env = "ACCOUNT_INCLUDE",
        value_delimiter = ',',
        help = "Comma-separated program/account addresses to watch on both streams"
    )]
    account_include: Vec<String>,

    #[clap(
        long,
        env = "LOG_SIG",
        default_value = "true",
        help = "Print transaction signatures as they arrive"
    )]
    log_sig: bool,

    #[clap(
        long,
        env = "RUN_DURATION_MINS",
        help = "Auto-stop after this many minutes (omit or set 0 to run forever)"
    )]
    run_duration_mins: Option<u64>,

    #[clap(
        long,
        env = "STATS_INTERVAL_SECS",
        default_value = "5",
        help = "How often to print throughput stats for both streams (seconds)"
    )]
    stats_interval_secs: u64,

    #[clap(
        long,
        env = "MATCH_TIMEOUT_SECS",
        default_value = "10",
        help = "How long to wait for a signature to show up on the other stream before counting it as stream-exclusive"
    )]
    match_timeout_secs: u64,

    #[clap(
        long,
        env = "WARMUP_SECS",
        default_value = "5",
        help = "Grace period (seconds) after startup during which a one-sided signature is treated as connection-startup skew, not a genuine miss (the two streams connect independently and don't backfill, so whichever comes up first can briefly see activity the other can't)"
    )]
    warmup_secs: u64,

    #[clap(
        long,
        env = "PRINT_MISSED_SIGS",
        default_value = "true",
        help = "Print every signature seen on the account stream but never on the tx stream, at the end of the report"
    )]
    print_missed_sigs: bool,

    #[clap(
        long,
        env = "REGION",
        default_value = "unknown",
        help = "Deployment region tag, prefixed on error/metrics logs"
    )]
    region: String,

    #[clap(
        long,
        env = "SLACK_URL",
        help = "Slack incoming webhook URL for disconnection alerts (omit to only log to console)"
    )]
    slack_url: Option<String>,
}

async fn connect_geyser(endpoint: &str, x_token: &str) -> anyhow::Result<GeyserGrpcClient> {
    GeyserGrpcClient::build_from_shared(endpoint.to_owned())?
        .x_token(Some(x_token.to_owned()))?
        .connect_timeout(Duration::from_secs(10))
        .timeout(Duration::from_secs(10))
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .max_decoding_message_size(1024 * 1024 * 1024)
        .connect()
        .await
        .map_err(Into::into)
}

fn build_tx_subscribe_request(account_include: &[String]) -> SubscribeRequest {
    let mut transactions: TxnFilterMap = HashMap::new();

    transactions.insert(
        "client".to_owned(),
        SubscribeRequestFilterTransactions {
            vote: Some(false),
            failed: Some(false),
            account_include: account_include.to_vec(),
            account_exclude: vec![],
            account_required: vec![],
            signature: None,
        },
    );

    SubscribeRequest {
        transactions,
        commitment: Some(CommitmentLevel::Processed as i32),
        ..Default::default()
    }
}

fn build_accounts_subscribe_request(account_include: &[String]) -> SubscribeRequest {
    let mut accounts: AccountsFilterMap = HashMap::new();

    accounts.insert(
        "client".to_owned(),
        SubscribeRequestFilterAccounts {
            account: vec![],
            owner: account_include.to_vec(),
            filters: vec![],
            // Startup snapshot updates carry no txn_signature and can't be
            // correlated with a transaction, so skip them server-side.
            nonempty_txn_signature: Some(true),
        },
    );

    SubscribeRequest {
        accounts,
        commitment: Some(CommitmentLevel::Processed as i32),
        ..Default::default()
    }
}

/// Runs one Yellowstone gRPC subscription with auto-reconnect (constant
/// backoff, idle-timeout detection, ping/pong keepalive, Slack alerts on
/// disconnect), dispatching every update that isn't a ping/pong to
/// `on_update`. Shared by the transaction stream and the account-update
/// stream — all stream-specific logic (verification, counting, race
/// tracking) lives in the `on_update` closure the caller provides.
async fn run_stream(
    region: String,
    label: &'static str,
    endpoint: String,
    x_token: String,
    subscribe_request: SubscribeRequest,
    slack_url: Option<String>,
    mut on_update: impl FnMut(UpdateOneof),
) -> anyhow::Result<()> {
    // A closure returning a future that borrows `on_update` mutably across
    // reconnect attempts doesn't satisfy the borrow checker (no stable
    // "async closures" here), so reconnection is a plain loop instead of
    // `backoff::retry_notify`: constant 2s backoff, notifying Slack on each
    // disconnect, same as before.
    let reconnect_delay = Duration::from_secs(2);

    loop {
        let Err(e) =
            run_stream_once(&region, label, &endpoint, &x_token, &subscribe_request, &mut on_update)
                .await
        else {
            unreachable!("run_stream_once only returns on error");
        };

        let msg = format!(
            "[{region}] 🐇 [{label}] Disconnected ({e}) — reconnecting in {:.0}s",
            reconnect_delay.as_secs_f64()
        );
        error!("{msg}");

        let slack_url = slack_url.clone();
        tokio::spawn(async move {
            slack::report(&slack_url, &msg).await;
        });

        tokio::time::sleep(reconnect_delay).await;
    }
}

/// One connect-subscribe-consume attempt; returns `Err` on any disconnect
/// (idle timeout, stream closed, or stream error) for `run_stream` to retry.
async fn run_stream_once(
    region: &str,
    label: &'static str,
    endpoint: &str,
    x_token: &str,
    subscribe_request: &SubscribeRequest,
    on_update: &mut impl FnMut(UpdateOneof),
) -> anyhow::Result<()> {
    info!("[{label}] Connecting to {endpoint}");

    let mut client = connect_geyser(endpoint, x_token).await.inspect_err(|e| {
        error!("[{region}] 🐇 [{label}] Connect failed: {e}");
    })?;

    let (mut sink, mut stream) = client
        .subscribe_with_request(Some(subscribe_request.clone()))
        .await
        .map_err(|e| {
            error!("[{region}] 🐇 [{label}] Subscribe failed: {e}");
            anyhow::anyhow!(e)
        })?;

    info!("[{label}] Subscribed — waiting for updates...");

    let mut ping_id: i32 = 0;
    let idle_timeout = Duration::from_secs(30);

    loop {
        match tokio::time::timeout(idle_timeout, stream.next()).await {
            Err(_) => {
                warn!(
                    "[{region}] [{label}] No messages received for {}s — reconnecting",
                    idle_timeout.as_secs()
                );
                return Err(anyhow::anyhow!("stream idle timeout"));
            }
            Ok(None) => {
                warn!("[{region}] [{label}] Stream closed by server — reconnecting");
                return Err(anyhow::anyhow!("stream ended unexpectedly"));
            }
            Ok(Some(Err(e))) => {
                error!("[{region}] [{label}] Stream error: {e} — reconnecting");
                return Err(anyhow::anyhow!(e));
            }
            Ok(Some(Ok(update))) => match update.update_oneof {
                Some(UpdateOneof::Ping(_)) => {
                    ping_id += 1;
                    sink.send(SubscribeRequest {
                        ping: Some(SubscribeRequestPing { id: ping_id }),
                        ..Default::default()
                    })
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?;
                }
                Some(UpdateOneof::Pong(_)) => {}
                Some(other) => on_update(other),
                None => {}
            },
        }
    }
}

#[allow(clippy::too_many_arguments)]
async fn run_tx_stream(
    region: String,
    endpoint: String,
    x_token: String,
    subscribe_request: SubscribeRequest,
    slack_url: Option<String>,
    log_sig: bool,
    stats_interval: Duration,
    run_start: Instant,
    target_addresses: Arc<HashSet<Vec<u8>>>,
    tracker: Arc<RaceTracker>,
    stats_history: Arc<Mutex<Vec<(f32, f32)>>>,
    received_count: Arc<AtomicU64>,
) -> anyhow::Result<()> {
    let log_region = region.clone();
    let mut tx_count: u64 = 0;
    let mut window_start = Instant::now();

    run_stream(
        region,
        "tx",
        endpoint,
        x_token,
        subscribe_request,
        slack_url,
        move |update| {
            let UpdateOneof::Transaction(tx) = update else {
                return;
            };

            // Rabbitstream's server-side account_include filtering should
            // guarantee every transaction we receive references one of these
            // addresses, but its payloads omit `meta` (no
            // loaded_writable/readonly_addresses for ALT lookups), so we can
            // only verify against the transaction's static
            // `message.account_keys`.
            let inner = tx.transaction.as_ref();
            let message = inner
                .and_then(|t| t.transaction.as_ref())
                .and_then(|t| t.message.as_ref());

            let sig = inner
                .and_then(|t| t.transaction.as_ref())
                .and_then(|t| t.signatures.first())
                .map(|b| bs58::encode(b).into_string())
                .unwrap_or_else(|| "<unknown>".to_string());

            let matched = message
                .is_some_and(|m| m.account_keys.iter().any(|k| target_addresses.contains(k)));

            // If the account_include address isn't among the static
            // account_keys, it may still have been loaded via an Address
            // Lookup Table — meta.loaded_writable/readonly_addresses would
            // normally resolve that, but Rabbitstream's payload omits meta,
            // so an ALT-using tx can't be conclusively judged either way.
            let has_alt_lookup = message.is_some_and(|m| !m.address_table_lookups.is_empty());

            if log_sig {
                info!("{}", sig);
            }

            received_count.fetch_add(1, Ordering::Relaxed);
            if !matched {
                if has_alt_lookup {
                    debug!(
                        "[{log_region}] transaction not found in static account_keys, but uses an address lookup table — can't verify without meta: {sig}"
                    );
                } else {
                    warn!(
                        "[{log_region}] ⚠️ transaction does not include any account_include address (no ALT lookups either): {sig}"
                    );
                }
            }

            tracker.record_tx(&sig, Instant::now());

            tx_count += 1;
            let elapsed = window_start.elapsed();
            if elapsed >= stats_interval {
                let tps = tx_count as f64 / elapsed.as_secs_f64();
                let total = received_count.load(Ordering::Relaxed);
                info!("[{log_region}] -----> throughput: {:.1} tx/s | total transactions: {} <------\n", tps, total);
                stats_history
                    .lock()
                    .unwrap()
                    .push((run_start.elapsed().as_secs_f32(), tps as f32));
                tx_count = 0;
                window_start = Instant::now();
            }
        },
    )
    .await
}

#[allow(clippy::too_many_arguments)]
async fn run_accounts_stream(
    region: String,
    endpoint: String,
    x_token: String,
    subscribe_request: SubscribeRequest,
    slack_url: Option<String>,
    stats_interval: Duration,
    run_start: Instant,
    tracker: Arc<RaceTracker>,
    stats_history: Arc<Mutex<Vec<(f32, f32)>>>,
    received_account_updates: Arc<AtomicU64>,
) -> anyhow::Result<()> {
    let log_region = region.clone();
    let mut upd_count: u64 = 0;
    let mut window_start = Instant::now();

    run_stream(
        region,
        "accounts",
        endpoint,
        x_token,
        subscribe_request,
        slack_url,
        move |update| {
            let UpdateOneof::Account(acc) = update else {
                return;
            };
            let Some(info) = acc.account else {
                return;
            };
            // Filtered server-side via nonempty_txn_signature, but guard
            // anyway in case that filter isn't honored end-to-end.
            let Some(sig_bytes) = info.txn_signature else {
                return;
            };

            let sig = bs58::encode(&sig_bytes).into_string();
            received_account_updates.fetch_add(1, Ordering::Relaxed);
            tracker.record_account(&sig, Instant::now());

            upd_count += 1;
            let elapsed = window_start.elapsed();
            if elapsed >= stats_interval {
                let ups = upd_count as f64 / elapsed.as_secs_f64();
                let total = received_account_updates.load(Ordering::Relaxed);
                info!("[{log_region}] -----> account throughput: {:.1} upd/s | total account updates: {} <------\n", ups, total);
                stats_history
                    .lock()
                    .unwrap()
                    .push((run_start.elapsed().as_secs_f32(), ups as f32));
                upd_count = 0;
                window_start = Instant::now();
            }
        },
    )
    .await
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Args::parse();
    let tx_label = endpoint_label(&args.endpoint_name, &args.endpoint);
    let accounts_label = endpoint_label(&args.accounts_endpoint_name, &args.accounts_endpoint);

    info!("Watching accounts: {}", args.account_include.join(", "));
    info!("Tx stream      : {tx_label}");
    info!("Account stream : {accounts_label}");

    let target_addresses: HashSet<Vec<u8>> = args
        .account_include
        .iter()
        .map(|addr| {
            bs58::decode(addr)
                .into_vec()
                .map_err(|e| anyhow::anyhow!("invalid account_include address {addr}: {e}"))
        })
        .collect::<anyhow::Result<_>>()?;
    let target_addresses = Arc::new(target_addresses);

    if let Some(mins) = args.run_duration_mins {
        if mins > 0 {
            info!("Will auto-stop after {} minutes", mins);
        }
    }

    let region = args.region.clone();
    let stats_history: Arc<Mutex<Vec<(f32, f32)>>> = Arc::new(Mutex::new(Vec::new()));
    let accounts_stats_history: Arc<Mutex<Vec<(f32, f32)>>> = Arc::new(Mutex::new(Vec::new()));
    let received_count = Arc::new(AtomicU64::new(0));
    let received_account_updates = Arc::new(AtomicU64::new(0));
    let tracker = Arc::new(RaceTracker::new(Duration::from_secs(args.warmup_secs)));
    let stats_interval = Duration::from_secs(args.stats_interval_secs);
    let run_start = Instant::now();

    let tx_handle = tokio::spawn(run_tx_stream(
        region.clone(),
        args.endpoint.clone(),
        args.x_token.clone(),
        build_tx_subscribe_request(&args.account_include),
        args.slack_url.clone(),
        args.log_sig,
        stats_interval,
        run_start,
        Arc::clone(&target_addresses),
        Arc::clone(&tracker),
        Arc::clone(&stats_history),
        Arc::clone(&received_count),
    ));

    let accounts_handle = tokio::spawn(run_accounts_stream(
        region.clone(),
        args.accounts_endpoint.clone(),
        args.accounts_x_token.clone(),
        build_accounts_subscribe_request(&args.account_include),
        args.slack_url.clone(),
        stats_interval,
        run_start,
        Arc::clone(&tracker),
        Arc::clone(&accounts_stats_history),
        Arc::clone(&received_account_updates),
    ));

    let match_timeout = Duration::from_secs(args.match_timeout_secs);
    let sweep_tracker = Arc::clone(&tracker);
    let sweep_handle = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(500));
        loop {
            interval.tick().await;
            sweep_tracker.sweep(match_timeout);
        }
    });

    let tx_abort = tx_handle.abort_handle();
    let accounts_abort = accounts_handle.abort_handle();
    let both = futures::future::try_join(tx_handle, accounts_handle);

    match args.run_duration_mins {
        Some(mins) if mins > 0 => {
            let deadline = Duration::from_secs(mins * 60);
            match tokio::time::timeout(deadline, both).await {
                Ok(joined) => {
                    let (tx_res, accounts_res) = joined?;
                    tx_res?;
                    accounts_res?;
                }
                Err(_) => {
                    info!("Run duration of {} minute(s) reached — stopping", mins);
                    tx_abort.abort();
                    accounts_abort.abort();
                }
            }
        }
        _ => {
            let (tx_res, accounts_res) = both.await?;
            tx_res?;
            accounts_res?;
        }
    }

    sweep_handle.abort();
    // Run is over: finalize any signature still waiting on its counterpart
    // as a stream-exclusive miss, regardless of the match timeout.
    tracker.flush_all();

    let tx_history = stats_history.lock().unwrap();
    let accounts_history = accounts_stats_history.lock().unwrap();
    print_report(
        &tx_label,
        &accounts_label,
        &args.account_include,
        &tx_history,
        received_count.load(Ordering::Relaxed),
        &accounts_history,
        received_account_updates.load(Ordering::Relaxed),
        &tracker.summary(),
        args.print_missed_sigs,
    );

    Ok(())
}

/// Per-window (elapsed_secs, rate) history -> (avg, peak, min) rate, or
/// `None` if fewer than two stats windows elapsed.
fn throughput_stats(history: &[(f32, f32)]) -> Option<(f32, f32, f32)> {
    if history.len() < 2 {
        return None;
    }
    let total: f32 = history.iter().map(|p| p.1).sum();
    let avg = total / history.len() as f32;
    let peak = history.iter().map(|p| p.1).fold(0.0_f32, f32::max);
    let min = history.iter().map(|p| p.1).fold(f32::MAX, f32::min);
    Some((avg, peak, min))
}

/// The display name for an endpoint: the configured `*_NAME` if set,
/// otherwise the endpoint URL with its scheme stripped.
fn endpoint_label(name: &Option<String>, endpoint: &str) -> String {
    name.clone().unwrap_or_else(|| {
        endpoint
            .trim_start_matches("https://")
            .trim_start_matches("http://")
            .to_owned()
    })
}

#[allow(clippy::too_many_arguments)]
fn print_report(
    tx_label: &str,
    accounts_label: &str,
    account_include: &[String],
    tx_history: &[(f32, f32)],
    tx_received: u64,
    accounts_history: &[(f32, f32)],
    accounts_received: u64,
    race: &race::RaceSummary,
    print_missed_sigs: bool,
) {
    let duration = tx_history
        .last()
        .or_else(|| accounts_history.last())
        .map(|p| p.0);

    println!("\n================================ Run Summary ================================");
    // println!("  Tx stream                  : {tx_label}");
    // println!("  Account stream             : {accounts_label}");
    println!(
        "  Address(es) watched        : {}",
        account_include.join(", ")
    );
    if let Some(d) = duration {
        println!("  Duration                   : {d:.0}s");
    }

    println!();
    println!("  Transactions received      : {tx_received}");
    if let Some((avg, peak, min)) = throughput_stats(tx_history) {
        println!("  Tx avg/peak/min tx/s       : {avg:.1} / {peak:.1} / {min:.1}");
    }

    println!();
    println!("  Account updates received   : {accounts_received}");
    if let Some((avg, peak, min)) = throughput_stats(accounts_history) {
        println!("  Account avg/peak/min upd/s : {avg:.1} / {peak:.1} / {min:.1}");
    }

    println!();
    println!(
        "  Matched signatures (seen on both streams) : {}",
        race.matched
    );
    if race.ties > 0 {
        println!("  Ties (same instant)                       : {}", race.ties);
    }
    println!(
        "  Tx-stream only (no account match)         : {}",
        race.tx_only
    );
    println!(
        "  Account-stream only (no tx match)         : {}",
        race.account_only
    );
    if race.warmup_skipped > 0 {
        println!(
            "  Skipped as startup warm-up skew           : {}",
            race.warmup_skipped
        );
    }

    println!();
    println!("  Arrival race — lead time when each stream reported the signature first:");
    println!(
        "  {:<9} {:<38} {:>10} {:>8} {:>10} {:>10} {:>10}",
        "Stream", "Name", "Total txns", "Ahead %", "p99 (ms)", "p95 (ms)", "p50 (ms)"
    );
    print_race_row("Tx", tx_label, &race.tx);
    print_race_row("Account", accounts_label, &race.account);

    println!("================================================================================\n");

    if print_missed_sigs && !race.account_only_sigs.is_empty() {
        println!(
            "Signatures seen on the account stream but never on the tx stream ({}):",
            race.account_only_sigs.len()
        );
        for (sig, at) in &race.account_only_sigs {
            println!("{}  {sig}", humantime::format_rfc3339_millis(*at));
        }
        println!();
    }
}

fn print_race_row(stream: &str, label: &str, s: &race::StreamRace) {
    println!(
        "  {:<9} {:<38} {:>10} {:>7.1}% {:>10.1} {:>10.1} {:>10.1}",
        stream, label, s.ahead, s.ahead_pct, s.p99_ms, s.p95_ms, s.p50_ms
    );
}
