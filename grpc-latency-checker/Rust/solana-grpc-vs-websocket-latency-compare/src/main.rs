use {
    chrono::Utc,
    clap::Parser,
    futures::{SinkExt, StreamExt},
    log::{error, info},
    maplit::hashmap,
    solana_client::{client_error::reqwest::Url, nonblocking::pubsub_client::PubsubClient, rpc_config::CommitmentConfig},
    solana_sdk::signature::Signature,
    std::{
        collections::{HashMap, HashSet},
        env,
        sync::Arc,
    },
    tokio::sync::{mpsc, oneshot},
    yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient},
    yellowstone_grpc_proto::geyser::{
        CommitmentLevel, SubscribeRequest, SubscribeRequestFilterTransactions,
        subscribe_update::UpdateOneof,
    },
};

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default)]
enum SourceType {
    #[default]
    GRPC,
    Websocket,
}

#[derive(Eq, Hash, PartialEq, Default, Debug)]
struct Timing {
    sig: String,
    timestamp: u64,
    node: Arc<SourceType>,
}
#[derive(Default, Debug)]
struct LatencyChecker {
    txns: HashMap<String, HashSet<Timing>>,
}
struct LatencyCheckerInput {
    signature: String,
    timestamp: u64,
    node: Arc<SourceType>,
}

#[derive(Default, Debug)]
struct LatencyReportLag {
    count: u64,
    time_taken: u64,
}
impl LatencyChecker {
    fn add_txn(&mut self, signature: String, timestamp: u64, node: Arc<SourceType>) {
        let timing = Timing {
            sig: signature.clone(),
            timestamp,
            node,
        };
        if let Some(set) = self.txns.get_mut(&signature) {
            set.insert(timing);
        } else {
            let mut set = HashSet::new();
            set.insert(timing);
            self.txns.insert(signature, set);
        }
    }

    async fn listen_messages(&mut self, mut m_rx: mpsc::Receiver<LatencyCheckerInput>) {
        while let Some(m) = m_rx.recv().await {
            info!(
                "received txn {} at {} from node {:?}",
                m.signature, m.timestamp, m.node
            );
            self.add_txn(m.signature, m.timestamp, m.node);
        }
    }

    fn get_report(&self) {
        info!("Generating report...");
        let mut txns_compare: HashMap<Arc<SourceType>, LatencyReportLag> = HashMap::new(); // map of node vs (fastest,slowest) between others
        let mut per_node_count: HashMap<Arc<SourceType>, u64> = HashMap::new();
        for v in self.txns.values() {
            for t in v {
                *per_node_count.entry(t.node.clone()).or_insert(0) += 1;
            }
            let mut values: Vec<_> = v.into_iter().collect();
            values.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));
            let fastest = values.first();
            let slowest = values.last();

            if let Some(f) = fastest {
                let s_tmp = slowest.map(|s| s.timestamp).unwrap_or(0);

                if f.timestamp == s_tmp {
                    continue;
                }
                info!("fast: {}, slow: {}", f.timestamp, s_tmp);
                if let Some(c) = txns_compare.get_mut(&f.node) {
                    c.count += 1;
                    c.time_taken += s_tmp - f.timestamp;
                } else {
                    txns_compare.insert(
                        f.node.clone(),
                        LatencyReportLag {
                            count: 1,
                            time_taken: s_tmp - f.timestamp,
                        },
                    );
                }
            }
        }

        let mut printed: Vec<SourceType> = Vec::new();
        let total_count: u64 = txns_compare.values().map(|v| v.count).sum();

        info!("Final results:");
        info!("----------  Total Transactions: {} --------", total_count);
        for (k, v) in txns_compare {
            let percentage = if total_count > 0 {
                (v.count as f64 / total_count as f64) * 100.0
            } else {
                0.0
            };

            info!(
                "{:?}, count: {} (faster in {:.2}% cases), avg_gain: {} ms",
                k,
                v.count,
                percentage,
                v.time_taken / v.count
            );
            printed.push(k.as_ref().clone());
        }

        if printed.is_empty() {
            info!("Error: Data not received from one endpoint");
        } else {
            for endpoint in [SourceType::GRPC, SourceType::Websocket] {
                if !printed.contains(&endpoint) {
                    info!(
                        "{:?}, count: 0 (faster in 0% cases), avg_gain: 0 ms (always slower or equal)",
                        endpoint
                    );
                }
            }
        }

        info!("--- Per-node total received txns ---");
        for (node, count) in &per_node_count {
            info!("{:?} received {} transactions", node, count);
        }
        info!("-------------------------------------");
    }
}

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(long)]
    endpoint: String,
    #[clap(long)]
    x_token: String,

    #[clap(long)]
    ws_uri: String,

    #[clap(long, default_value = "60")]
    duration: u64, // seconds
}

async fn grpc_message_handler(
    timeout: oneshot::Receiver<bool>,
    endpoint: String,
    token: Option<String>,
    m_tx: mpsc::Sender<LatencyCheckerInput>,
) {
    let mut client = GeyserGrpcClient::build_from_shared(endpoint.clone())
        .unwrap()
        .x_token(token)
        .unwrap()
        .tls_config(ClientTlsConfig::new().with_native_roots())
        .unwrap()
        //.send_compressed(yellowstone_grpc_proto::tonic::codec::CompressionEncoding::Gzip)
        //.accept_compressed(yellowstone_grpc_proto::tonic::codec::CompressionEncoding::Gzip)
        .connect()
        .await
        .unwrap();
    let (mut subscribe_tx, mut stream) = client.subscribe().await.unwrap();

    subscribe_tx
        .send(SubscribeRequest {
            slots: HashMap::new(),
            accounts: HashMap::new(),
            transactions: HashMap::new(),
            transactions_status: hashmap! { "".to_owned() => SubscribeRequestFilterTransactions {
                vote: None,
                failed: None,
                signature: None,
                account_include: vec!["pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA".to_string()],
                account_exclude: Vec::new(),
                account_required: Vec::new(),
            } },
            entry: HashMap::new(),
            blocks: HashMap::new(),
            blocks_meta: HashMap::new(),
            commitment: Some(CommitmentLevel::Processed as i32),
            accounts_data_slice: vec![],
            ping: None,
            from_slot: None,
        })
        .await
        .unwrap();
    tokio::select! {
        _ = timeout => {
            println!("Timeout reached, ending stream...");
        }
        _ = async {
            while let Some(message) = stream.next().await {
                match message {
                    Ok(msg) => match msg.update_oneof {
                        Some(UpdateOneof::TransactionStatus(tx)) => {
                            let sig = Signature::try_from(tx.signature.as_slice())
                                .expect("valid signature from transaction")
                                .to_string();
                            let current_time_millis = Utc::now().timestamp_millis() as u64;
                            // info!("received txn {} at {} from node {}", sig, current_time_millis, endpoint);
                            _ = m_tx.send(LatencyCheckerInput{
                                signature: sig,
                                timestamp: current_time_millis,
                                node: Arc::new(SourceType::GRPC),
                            }).await;
                        }
                        _ => {}
                    },
                    Err(error) => {
                        error!("stream error: {error:?}");
                        break;
                    }
                }
            }
        } => {}
    }
}

async fn ws_message_handler(
    timeout: oneshot::Receiver<bool>,
    ws_url: String,
    m_tx: mpsc::Sender<LatencyCheckerInput>,
) {
    let mut ws_url = Url::parse(&ws_url).unwrap();
    if ws_url.path().is_empty() {
        ws_url.set_path("/");
    }
    let ps_client = PubsubClient::new(ws_url.to_string()).await.unwrap();
    let (mut stream, unsubscriber) = ps_client
        .logs_subscribe(
            solana_client::rpc_config::RpcTransactionLogsFilter::Mentions(vec![
                "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA".to_string(),
            ]),
            solana_client::rpc_config::RpcTransactionLogsConfig {
                commitment: Some(CommitmentConfig::processed()),
            },
        )
        .await
        .unwrap();

    tokio::select! {
        _ = timeout => {
            println!("Timeout reached, ending stream...");
        }
        _ = async {
             while let Some(message) = stream.next().await {
                let sig = message.value.signature;
                let current_time_millis = Utc::now().timestamp_millis() as u64;
                _ = m_tx.send(LatencyCheckerInput{
                    signature: sig,
                    timestamp: current_time_millis,
                    node: Arc::new(SourceType::Websocket),
                }).await;
            }
        } => {}
    }

    unsubscriber().await;
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    unsafe {
        env::set_var(
            env_logger::DEFAULT_FILTER_ENV,
            env::var_os(env_logger::DEFAULT_FILTER_ENV).unwrap_or_else(|| "info".into()),
        );
    }
    env_logger::init();
    let args = Args::parse();
    info!("Args: {:?}", args);
    let timeout = tokio::time::sleep(tokio::time::Duration::from_secs(args.duration));

    let mut latency_checker = LatencyChecker::default();

    let mut shutdown_sig = Vec::new();
    let (m_tx, m_rx) = mpsc::channel(100_000);

    let endpoint = Arc::new(args.endpoint.clone());

    let (tx, rx) = oneshot::channel();
    shutdown_sig.push(tx);

    let m_tx_g = m_tx.clone();

    info!("starting yellowstone grpc stream{}", endpoint,);
    tokio::spawn(async move {
        grpc_message_handler(rx, args.endpoint, Some(args.x_token), m_tx_g).await;
    });

    let endpoint = Arc::new(args.ws_uri.clone());

    let (tx, rx) = oneshot::channel();
    shutdown_sig.push(tx);

    let m_tx_ws = m_tx.clone();

    info!("starting websocket stream {}", endpoint);
    tokio::spawn(async move {
        ws_message_handler(rx, args.ws_uri, m_tx_ws).await;
    });

    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("failed to listen for Ctrl+C");
    };

    info!("Press Ctrl+C to stop early, or wait for timeout...");

    tokio::select! {
        _ = latency_checker.listen_messages(m_rx) => {}

        _ = timeout => {
            for sig in shutdown_sig {
                _ = sig.send(true);
            }
            latency_checker.get_report();
        }
        _ = ctrl_c => {
            info!("Ctrl+C received, shutting down...");
            for sig in shutdown_sig {
                _ = sig.send(true);
            }
            latency_checker.get_report();
        }
    }
}
