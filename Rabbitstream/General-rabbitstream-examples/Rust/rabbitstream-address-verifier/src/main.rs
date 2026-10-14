use {
    backoff::{backoff::Constant, future::retry_notify},
    clap::Parser,
    futures::{sink::SinkExt, stream::StreamExt},
    log::{debug, error, info, warn},
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
        SubscribeRequestFilterTransactions, SubscribeRequestPing,
    },
};

mod slack;

type TxnFilterMap = HashMap<String, SubscribeRequestFilterTransactions>;

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, env = "ENDPOINT", help = "gRPC endpoint")]
    endpoint: String,

    #[clap(long, env = "X_TOKEN", help = "X-Token")]
    x_token: String,

    #[clap(
        long,
        env = "ACCOUNT_INCLUDE",
        value_delimiter = ',',
        help = "Comma-separated program/account addresses to filter on"
    )]
    account_include: Vec<String>,

    #[clap(
        long,
        env = "LOG_SIG",
        default_value = "true",
        help = "Print transaction signatures"
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
        help = "How often to print throughput stats (seconds)"
    )]
    stats_interval_secs: u64,

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

impl Args {
    async fn connect(&self) -> anyhow::Result<GeyserGrpcClient> {
        GeyserGrpcClient::build_from_shared(self.endpoint.clone())?
            .x_token(Some(self.x_token.clone()))?
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(10))
            .tls_config(ClientTlsConfig::new().with_native_roots())?
            .max_decoding_message_size(1024 * 1024 * 1024)
            .connect()
            .await
            .map_err(Into::into)
    }

    fn subscribe_request(&self) -> SubscribeRequest {
        let mut transactions: TxnFilterMap = HashMap::new();

        transactions.insert(
            "client".to_owned(),
            SubscribeRequestFilterTransactions {
                vote: Some(false),
                failed: Some(false),
                account_include: self.account_include.clone(),
                account_exclude: vec![],
                account_required: vec![],
                signature: None,
            },
        );

        SubscribeRequest {
            accounts: HashMap::default(),
            slots: HashMap::default(),
            transactions,
            transactions_status: HashMap::default(),
            blocks: HashMap::default(),
            blocks_meta: HashMap::default(),
            entry: HashMap::default(),
            commitment: Some(CommitmentLevel::Processed as i32),
            accounts_data_slice: Vec::default(),
            ping: None,
            from_slot: None,
        }
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let args = Args::parse();
    info!("Watching accounts: {}", args.account_include.join(", "));

    // Rabbitstream's server-side account_include filtering should guarantee every
    // transaction we receive references one of these addresses, but its payloads
    // omit `meta` (no loaded_writable/readonly_addresses for ALT lookups), so we
    // can only verify against the transaction's static `message.account_keys`.
    let target_addresses: HashSet<Vec<u8>> = args
        .account_include
        .iter()
        .map(|addr| {
            bs58::decode(addr)
                .into_vec()
                .map_err(|e| anyhow::anyhow!("invalid account_include address {addr}: {e}"))
        })
        .collect::<anyhow::Result<_>>()?;

    if let Some(mins) = args.run_duration_mins {
        if mins > 0 {
            info!("Will auto-stop after {} minutes", mins);
        }
    }

    let stats_history: Arc<Mutex<Vec<(f32, f32)>>> = Arc::new(Mutex::new(Vec::new()));
    let received_count = AtomicU64::new(0);
    let matched_count = AtomicU64::new(0);
    let alt_indeterminate_count = AtomicU64::new(0);
    let unmatched_count = AtomicU64::new(0);
    let run_start = Instant::now();
    let region = args.region.clone();
    let notify_region = region.clone();
    let notify_slack_url = args.slack_url.clone();

    let run = retry_notify(
        Constant::new(Duration::from_secs(2)),
        || async {
            info!("Connecting to {}", args.endpoint);

            let mut client = args.connect().await.map_err(|e| {
                error!("[{region}] 🐇 Connect failed: {e}");
                backoff::Error::transient(e)
            })?;

            let (mut sink, mut stream) = client
                .subscribe_with_request(Some(args.subscribe_request()))
                .await
                .map_err(|e| {
                    error!("[{region}] 🐇 Subscribe failed: {e}");
                    backoff::Error::transient(anyhow::anyhow!(e))
                })?;

            info!("Subscribed — waiting for transactions...");

            let mut ping_id: i32 = 0;
            let idle_timeout = Duration::from_secs(30);
            let stats_interval = Duration::from_secs(args.stats_interval_secs);
            let mut tx_count: u64 = 0;
            let mut window_start = Instant::now();

            loop {
                match tokio::time::timeout(idle_timeout, stream.next()).await {
                    Err(_) => {
                        warn!(
                            "[{region}] No messages received for {}s — reconnecting",
                            idle_timeout.as_secs()
                        );
                        return Err(backoff::Error::transient(anyhow::anyhow!(
                            "stream idle timeout"
                        )));
                    }
                    Ok(None) => {
                        warn!("[{region}] Stream closed by server — reconnecting");
                        return Err(backoff::Error::transient(anyhow::anyhow!(
                            "stream ended unexpectedly"
                        )));
                    }
                    Ok(Some(Err(e))) => {
                        error!("[{region}] Stream error: {e} — reconnecting");
                        return Err(backoff::Error::transient(anyhow::anyhow!(e)));
                    }
                    Ok(Some(Ok(update))) => match update.update_oneof {
                        Some(UpdateOneof::Transaction(tx)) => {
                            let inner = tx.transaction.as_ref();
                            let message = inner
                                .and_then(|t| t.transaction.as_ref())
                                .and_then(|t| t.message.as_ref());

                            let sig = inner
                                .and_then(|t| t.transaction.as_ref())
                                .and_then(|t| t.signatures.first())
                                .map(|b| bs58::encode(b).into_string())
                                .unwrap_or_else(|| "<unknown>".to_string());

                            let matched = message.is_some_and(|m| {
                                m.account_keys.iter().any(|k| target_addresses.contains(k))
                            });

                            // If the account_include address isn't among the static
                            // account_keys, it may still have been loaded via an Address
                            // Lookup Table — meta.loaded_writable/readonly_addresses would
                            // normally resolve that, but Rabbitstream's payload omits meta,
                            // so an ALT-using tx can't be conclusively judged either way.
                            let has_alt_lookup =
                                message.is_some_and(|m| !m.address_table_lookups.is_empty());

                            if args.log_sig {
                                info!("{}", sig);
                            }

                            received_count.fetch_add(1, Ordering::Relaxed);
                            if matched {
                                matched_count.fetch_add(1, Ordering::Relaxed);
                            } else if has_alt_lookup {
                                alt_indeterminate_count.fetch_add(1, Ordering::Relaxed);
                                debug!(
                                    "[{region}] transaction not found in static account_keys, but uses an address lookup table — can't verify without meta: {sig}"
                                );
                            } else {
                                unmatched_count.fetch_add(1, Ordering::Relaxed);
                                warn!(
                                    "[{region}] ⚠️ transaction does not include any account_include address (no ALT lookups either): {sig}"
                                );
                            }

                            tx_count += 1;
                            let elapsed = window_start.elapsed();
                            if elapsed >= stats_interval {
                                let tps = tx_count as f64 / elapsed.as_secs_f64();
                                let total = received_count.load(Ordering::Relaxed);
                                info!("[{region}] -----> throughput: {:.1} tx/s | total transactions: {} <------\n", tps, total);
                                stats_history
                                    .lock()
                                    .unwrap()
                                    .push((run_start.elapsed().as_secs_f32(), tps as f32));
                                tx_count = 0;
                                window_start = Instant::now();
                            }
                        }
                        Some(UpdateOneof::Ping(_)) => {
                            ping_id += 1;
                            sink.send(yellowstone_grpc_proto::prelude::SubscribeRequest {
                                ping: Some(SubscribeRequestPing { id: ping_id }),
                                ..Default::default()
                            })
                            .await
                            .map_err(|e| backoff::Error::transient(anyhow::anyhow!(e)))?;
                        }
                        Some(UpdateOneof::Pong(_)) => {}
                        _ => {}
                    },
                }
            }
        },
        move |err, dur: Duration| {
            let msg = format!(
                "[{notify_region}] 🐇 Disconnected ({err}) — reconnecting in {:.0}s",
                dur.as_secs_f64()
            );
            error!("{msg}");

            let slack_url = notify_slack_url.clone();
            tokio::spawn(async move {
                slack::report(&slack_url, &msg).await;
            });
        },
    );

    match args.run_duration_mins {
        Some(mins) if mins > 0 => {
            let deadline = Duration::from_secs(mins * 60);
            match tokio::time::timeout(deadline, run).await {
                Ok(result) => result?,
                Err(_) => info!("Run duration of {} minute(s) reached — stopping", mins),
            }
        }
        _ => run.await?,
    }

    let history = stats_history.lock().unwrap();
    print_summary(
        &history,
        &args.endpoint,
        &args.account_include,
        received_count.load(Ordering::Relaxed),
        matched_count.load(Ordering::Relaxed),
        alt_indeterminate_count.load(Ordering::Relaxed),
        unmatched_count.load(Ordering::Relaxed),
    );

    Ok(())
}

fn print_summary(
    history: &[(f32, f32)],
    endpoint: &str,
    account_include: &[String],
    received: u64,
    matched: u64,
    alt_indeterminate: u64,
    unmatched: u64,
) {
    let pct = |n: u64| {
        if received > 0 {
            n as f64 / received as f64 * 100.0
        } else {
            0.0
        }
    };

    println!("\n========== Run Summary ==========");
    println!("  Endpoint                   : {}", endpoint);
    println!(
        "  Address(es) verified       : {}",
        account_include.join(", ")
    );
    println!("  Transactions received      : {}", received);
    println!(
        "  Matched (static keys)      : {} ({:.1}%)",
        matched,
        pct(matched)
    );
    println!(
        "  Indeterminate (ALT lookup) : {} ({:.1}%)",
        alt_indeterminate,
        pct(alt_indeterminate)
    );
    println!(
        "  Unmatched (no ALT either)  : {} ({:.1}%)",
        unmatched,
        pct(unmatched)
    );

    if history.len() >= 2 {
        let total: f32 = history.iter().map(|p| p.1).sum();
        let avg = total / history.len() as f32;
        let peak = history.iter().map(|p| p.1).fold(0.0_f32, f32::max);
        let min = history.iter().map(|p| p.1).fold(f32::MAX, f32::min);
        let duration = history.last().unwrap().0;

        println!("  Duration : {:.0}s", duration);
        println!("  Avg tx/s : {:.1}", avg);
        println!("  Peak tx/s: {:.1}", peak);
        println!("  Min  tx/s: {:.1}", min);
    }

    println!("=================================\n");
}
