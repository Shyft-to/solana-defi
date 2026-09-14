use {
    backoff::{backoff::Constant, future::retry_notify},
    clap::Parser,
    futures::{sink::SinkExt, stream::StreamExt},
    log::{error, info, warn},
    std::{
        collections::HashMap,
        sync::{
            atomic::{AtomicU64, Ordering},
            Arc, Mutex,
        },
        time::{Duration, Instant, SystemTime},
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
        env = "SHOW_TIMESTAMPS",
        default_value = "false",
        help = "Print created_at/received_at timestamps before each latency line"
    )]
    show_timestamps: bool,

    #[clap(
        long,
        env = "MAX_TRANSACTIONS",
        help = "Stop after this many transactions and print a latency report (omit or set 0 to run indefinitely)"
    )]
    max_transactions: Option<u64>,

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

    if let Some(max) = args.max_transactions {
        if max > 0 {
            info!("Will auto-stop after {} transactions", max);
        }
    }

    let stats_history: Arc<Mutex<Vec<(f32, f32)>>> = Arc::new(Mutex::new(Vec::new()));
    let latencies: Arc<Mutex<Vec<f64>>> = Arc::new(Mutex::new(Vec::new()));
    let tx_seen = Arc::new(AtomicU64::new(0));
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
            let mut total_count: u64 = 0;
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
                    Ok(Some(Ok(update))) => {
                        let received_at = SystemTime::now();
                        let created_at = update.created_at.clone();
                        match update.update_oneof {
                        Some(UpdateOneof::Transaction(tx)) => {
                            if args.log_sig {
                            let sig = tx
                                .transaction
                                .as_ref()
                                .and_then(|t| t.transaction.as_ref())
                                .and_then(|t| t.signatures.first())
                                .map(|b| bs58::encode(b).into_string())
                                .unwrap_or_else(|| "<unknown>".to_string());

                                info!("{}", sig);
                            }
                            
                            let created_system_time =
                                created_at.and_then(|ts| SystemTime::try_from(ts).ok());

                            // Printed unconditionally (created_at left empty when
                            // missing/invalid) so this line never depends on whether the
                            // latency below comes out positive, negative, or unavailable.
                            if args.log_sig && args.show_timestamps {
                                let created_str = created_system_time
                                    .map(|t| humantime::format_rfc3339_millis(t).to_string())
                                    .unwrap_or_default();
                                info!(
                                    "[{region}] created_at: {created_str} | received_at: {}",
                                    humantime::format_rfc3339_millis(received_at)
                                );
                            }

                            match created_system_time {
                                Some(created_system_time) => {
                                    match received_at.duration_since(created_system_time) {
                                        Ok(latency) => {
                                            let latency_ms = latency.as_secs_f64() * 1000.0;
                                            latencies.lock().unwrap().push(latency_ms);
                                            if args.log_sig {
                                                info!(
                                                    "[{region}] created_at -> received_at latency: {:.2}ms",
                                                    latency_ms
                                                );
                                            }
                                        }
                                        Err(e) => {
                                            warn!(
                                                "[{region}] received_at is before created_at by {:?} (clock skew?)",
                                                e.duration()
                                            );
                                        }
                                    }
                                }
                                None => {
                                    warn!("[{region}] update missing/invalid created_at timestamp");
                                }
                            }

                            let seen = tx_seen.fetch_add(1, Ordering::Relaxed) + 1;
                            if let Some(max) = args.max_transactions {
                                if max > 0 {
                                    // Log progress roughly every 5%, plus always the final tx.
                                    let step = (max / 20).max(1);
                                    if seen % step == 0 || seen == max {
                                        let pct = (seen as f64 / max as f64) * 100.0;
                                        info!("[{region}] Progress: {seen}/{max} ({pct:.1}%)");
                                    }
                                }
                                if max > 0 && seen >= max {
                                    info!("[{region}] Reached {max} transactions — closing stream");
                                    // Half-close the client -> server request channel...
                                    if let Err(e) = sink.close().await {
                                        warn!("[{region}] Error closing subscribe sink: {e}");
                                    }
                                    // ...then drop the server -> client stream. tonic/h2 send
                                    // RST_STREAM on drop of an unfinished Streaming<T>, which is
                                    // how a bidi gRPC call is cancelled client-side (there is no
                                    // explicit `.cancel()` in this crate).
                                    drop(stream);
                                    return Ok(());
                                }
                            }

                            // tx_count += 1;
                            // total_count += 1;
                            // let elapsed = window_start.elapsed();
                            // if elapsed >= stats_interval {
                            //     let tps = tx_count as f64 / elapsed.as_secs_f64();
                            //     info!("[{region}] -----> throughput: {:.1} tx/s | total transactions: {} <------\n", tps, total_count);
                            //     stats_history
                            //         .lock()
                            //         .unwrap()
                            //         .push((run_start.elapsed().as_secs_f32(), tps as f32));
                            //     tx_count = 0;
                            //     window_start = Instant::now();
                            // }

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
                        }
                    }
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

    run.await?;

    let history = stats_history.lock().unwrap();
    print_summary(&history);

    let latency_samples = latencies.lock().unwrap();
    print_latency_report(&args.region, &latency_samples);

    Ok(())
}

fn print_summary(history: &[(f32, f32)]) {
    if history.len() < 2 {
        return;
    }

    let total: f32 = history.iter().map(|p| p.1).sum();
    let avg = total / history.len() as f32;
    let peak = history.iter().map(|p| p.1).fold(0.0_f32, f32::max);
    let min = history.iter().map(|p| p.1).fold(f32::MAX, f32::min);
    let duration = history.last().unwrap().0;

    println!("\n========== Run Summary ==========");
    println!("  Duration : {:.0}s", duration);
    println!("  Avg tx/s : {:.1}", avg);
    println!("  Peak tx/s: {:.1}", peak);
    println!("  Min  tx/s: {:.1}", min);
    println!("=================================\n");
}

/// Linear-interpolated percentile over a pre-sorted ascending slice.
fn percentile(sorted: &[f64], pct: f64) -> f64 {
    if sorted.is_empty() {
        return 0.0;
    }
    let rank = (pct / 100.0) * (sorted.len() - 1) as f64;
    let lo = rank.floor() as usize;
    let hi = rank.ceil() as usize;
    if lo == hi {
        sorted[lo]
    } else {
        let frac = rank - lo as f64;
        sorted[lo] + (sorted[hi] - sorted[lo]) * frac
    }
}

fn print_latency_report(region: &str, latencies: &[f64]) {
    if latencies.is_empty() {
        println!("\nNo latency samples collected — nothing to report.\n");
        return;
    }

    let mut sorted = latencies.to_vec();
    sorted.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let count = sorted.len();
    let sum: f64 = sorted.iter().sum();
    let avg = sum / count as f64;
    let min = sorted[0];
    let max = sorted[count - 1];
    let p50 = percentile(&sorted, 50.0);
    let p95 = percentile(&sorted, 95.0);
    let p99 = percentile(&sorted, 99.0);

    println!("\n===== Latency Report [{region}] — {count} samples =====");
    println!("+---------+--------------+");
    println!("| Metric  | Latency (ms) |");
    println!("+---------+--------------+");
    println!("| Min     | {min:>12.2} |");
    println!("| Average | {avg:>12.2} |");
    println!("| P50     | {p50:>12.2} |");
    println!("| P95     | {p95:>12.2} |");
    println!("| P99     | {p99:>12.2} |");
    println!("| Max     | {max:>12.2} |");
    println!("+---------+--------------+\n");
}
