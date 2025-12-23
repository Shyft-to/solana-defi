use {
    chrono::Utc,
    dotenv::dotenv,
    futures::{sink::SinkExt, stream::StreamExt},
    jito_protos::shredstream::{
        shredstream_proxy_client::ShredstreamProxyClient, SubscribeEntriesRequest,
    },
    log::{error, info},
    maplit::hashmap,
    serde::Deserialize,
    solana_entry::entry::Entry,
    solana_sdk::signature::Signature,
    std::{
        collections::{HashMap, HashSet},
        env,
        str::FromStr,
        sync::Arc,
        time::Duration,
    },
    tokio::sync::{mpsc, oneshot},
    tonic::{metadata::MetadataValue, transport::Endpoint, Request},
    yellowstone_grpc_client::{ClientTlsConfig, GeyserGrpcClient},
    yellowstone_grpc_proto::prelude::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
        SubscribeRequestFilterBlocksMeta, SubscribeRequestFilterTransactions,
    },
};

#[derive(Debug, Deserialize, Clone)]
struct StreamConfig {
    uri: String,
    x_token: Option<String>,
}

#[derive(Debug, Clone)]
struct Args {
    yellowstone_stream_configs: Option<Vec<StreamConfig>>,
    shred_stream_configs: Option<Vec<StreamConfig>>,
    timeout_dur: u64,
}

impl Args {
    fn from_env() -> Self {
        dotenv().ok();
        env::set_var(
            env_logger::DEFAULT_FILTER_ENV,
            env::var_os(env_logger::DEFAULT_FILTER_ENV).unwrap_or_else(|| "info".into()),
        );
        env_logger::init();

        let yellowstone_stream_configs =
            parse_json_vec_env::<StreamConfig>("YELLOWSTONE_STREAM_CONFIGS");
        let shred_stream_configs = parse_json_vec_env::<StreamConfig>("SHRED_STREAM_CONFIGS");

        let timeout_dur = env::var("TIMEOUT_DUR")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(60);

        Args {
            yellowstone_stream_configs: Some(yellowstone_stream_configs),
            shred_stream_configs: Some(shred_stream_configs),
            timeout_dur,
        }
    }
}

fn parse_json_vec_env<T: for<'de> Deserialize<'de>>(key: &str) -> Vec<T> {
    env::var(key)
        .ok()
        .and_then(|v| serde_json::from_str::<Vec<T>>(&v).ok())
        .unwrap_or_default()
}

#[derive(Eq, Hash, PartialEq, Default, Debug)]
struct Timing {
    sig: String,
    timestamp: u64,
    node: Arc<String>,
}
#[derive(Default, Debug)]
struct LatencyChecker {
    event_timings: HashMap<String, HashSet<Timing>>, 
    winning_margins: HashMap<Arc<String>, MarginSamples>,
}
struct LatencyCheckerInput {
    signature: String,
    timestamp: u64,
    node: Arc<String>,
    m_type: u8,
}

#[derive(Default, Debug)]
struct MarginSamples {
    // Stores the margin of victory (Time_Slowest - Time_Fastest) for every win by this node.
    samples: Vec<u64>,
}

#[derive(Default, Debug)]
struct LatencyReportLag {
    count: u64,
    time_taken: u64,
}

// Helper function to calculate a specific percentile (P25, P50, P99, etc.)
fn calculate_percentile(sorted_data: &[u64], percentile: f64) -> u64 {
    if sorted_data.is_empty() {
        return 0;
    }
    
    let count = sorted_data.len();
    let index_float = (percentile / 100.0) * count as f64;
    let index = (index_float.ceil() as usize).saturating_sub(1);
    let safe_index = index.min(count - 1);
    
    sorted_data[safe_index]
}

impl LatencyChecker {
    // m_type 0
    fn add_txn(&mut self, signature: String, timestamp: u64, node: Arc<String>) {
        let timing = Timing {
            sig: signature.clone(),
            timestamp,
            node, // Arc<String>
        };
        // Insert the timing into the event_timings map.
        self.event_timings
            .entry(signature)
            .or_insert_with(HashSet::new)
            .insert(timing);
    }
    
    // m_type 1: Block Hash
    fn add_block(&mut self, block: String, timestamp: u64, node: Arc<String>) {
        let timing = Timing {
            sig: block.clone(),
            timestamp,
            node, // Arc<String>
        };
        // Insert the timing into the event_timings map.
        self.event_timings
            .entry(block)
            .or_insert_with(HashSet::new)
            .insert(timing);
    }

    async fn listen_messages(&mut self, mut m_rx: mpsc::Receiver<LatencyCheckerInput>) {
        while let Some(m) = m_rx.recv().await {
            if m.m_type == 0 {
                info!(
                    "received txn {} at {} from node {}",
                    m.signature, m.timestamp, m.node
                );
                self.add_txn(m.signature, m.timestamp, m.node);
            } else {
                self.add_block(m.signature, m.timestamp, m.node);
            }
        }
    }

    fn get_report(&mut self, all_endpoints: &[Arc<String>]) {
        
        for v in self.event_timings.values() {
            if v.len() < 2 {
                continue;
            }
            
            let mut values: Vec<_> = v.iter().collect();
            values.sort_by(|a, b| a.timestamp.cmp(&b.timestamp));

            let fastest = values.first().unwrap(); 
            let slowest = values.last().unwrap();  

            let gain = slowest.timestamp.saturating_sub(fastest.timestamp);

            if gain > 0 {
                self.winning_margins
                    .entry(fastest.node.clone())
                    .or_default()
                    .samples
                    .push(gain);
            }
        }
        
        
        let total_events = self.event_timings.len() as f64;
        
        info!("Final Winning Margin Percentile Results:");
        info!("------------------------------------------");
        
        let mut printed_endpoints = HashSet::new();
        
        for (node, margins) in &mut self.winning_margins {
            printed_endpoints.insert(node.clone());
            
            let count = margins.samples.len() as f64;
            let count_u64 = margins.samples.len() as u64;

            if count_u64 == 0 {
                continue;
            }
            
            let percentage_wins = if total_events > 0.0 {
                (count / total_events) * 100.0
            } else {
                0.0
            };

            margins.samples.sort_unstable();

            // let p25 = calculate_percentile(&margins.samples, 25.0);
            let p50 = calculate_percentile(&margins.samples, 50.0);
            let p99 = calculate_percentile(&margins.samples, 99.0);
            
            info!(
                "Node: {:?}, Wins: {} ({:.2}%), P50(Median): {} ms, P99: {} ms",
                node,
                count_u64,
                percentage_wins,
                p50,
                p99
            );
        }

        // --- STEP 3: Report on Non-Winning Nodes ---
        info!("Total Events Observed: {}", total_events);

        for endpoint in all_endpoints {
            if !printed_endpoints.contains(endpoint) {
                // Percentage wins is 0.00%
                info!(
                    "Node: {:?} (Wins: 0 (0.00%)), P25/50/99 Margin: N/A (Never Won)",
                    endpoint
                );
            }
        }
        
        info!("------------------------------------------");
        info!("Note: Margin is the time advantage (Gain) the fastest node had over the slowest node for a single event.");
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    let args = Args::from_env();
    info!("Args: {:?}", args);
    let timeout = tokio::time::sleep(tokio::time::Duration::from_secs(args.timeout_dur));

    let mut latency_checker = LatencyChecker::default();

    let mut shutdown_sig = Vec::new();
    let (m_tx, m_rx) = mpsc::channel(100_000);

    let mut all_endpoints: Vec<Arc<String>> = Vec::new();

    match args.yellowstone_stream_configs {
        Some(yellowstone_stream_configs) => {
            for yellowstone_stream_config in yellowstone_stream_configs {
                let endpoint = Arc::new(yellowstone_stream_config.uri.clone());
                all_endpoints.push(endpoint.clone()); 

                let token = yellowstone_stream_config.x_token.clone();
                let (tx, rx) = oneshot::channel();
                shutdown_sig.push(tx);
                let m_tx = m_tx.clone();

                info!(
                    "starting yellowstone grpc stream{}",
                    yellowstone_stream_config.uri
                );
                tokio::spawn(async move {
                    grpc_message_handler(rx, yellowstone_stream_config.uri, token, m_tx).await;
                });
            }
        }
        None => {}
    }

    match args.shred_stream_configs {
        Some(shred_stream_configs) => {
            for shred_stream_config in shred_stream_configs {
                let endpoint = Arc::new(shred_stream_config.uri.clone());
                all_endpoints.push(endpoint.clone()); 


                let token = shred_stream_config.x_token.clone();
                let (tx, rx) = oneshot::channel();
                shutdown_sig.push(tx);
                let m_tx = m_tx.clone();

                info!("starting shredstream {}", shred_stream_config.uri);
                tokio::spawn(async move {
                    shred_message_handler(rx, shred_stream_config.uri, token, m_tx).await;
                });
            }
        }
        None => {}
    }

    tokio::select! {
        _ = latency_checker.listen_messages(m_rx) => {}
        _ = timeout => {
            for sig in shutdown_sig {
                _ = sig.send(true);
            }
            latency_checker.get_report(&all_endpoints);
        }
    }
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
    let endpoint = Arc::new(endpoint);

    let commitment: CommitmentLevel = CommitmentLevel::default();
    subscribe_tx
        .send(SubscribeRequest {
            slots: HashMap::new(),
            accounts: HashMap::new(),
            transactions: HashMap::new(),
            transactions_status: hashmap! { "".to_owned() => SubscribeRequestFilterTransactions {
                vote: Some(false),
                failed: Some(false),
                signature: None,
                account_include: Vec::new(),
                account_exclude: Vec::new(),
                account_required: Vec::new(),
            } },
            entry: HashMap::new(),
            blocks: HashMap::new(),
            blocks_meta: hashmap! { "".to_owned() => SubscribeRequestFilterBlocksMeta {} },
            commitment: Some(commitment as i32),
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
                                node: endpoint.clone(),
                                m_type: 0,
                            }).await;
                        }
                        Some(UpdateOneof::BlockMeta(block)) => {
                            let current_time_millis = Utc::now().timestamp_millis() as u64;
                            info!("received block {} at {}", block.blockhash, current_time_millis);
                            _ = m_tx.send(LatencyCheckerInput {
                                signature: block.blockhash,
                                timestamp:current_time_millis,
                                node: endpoint.clone(),
                                m_type: 1,
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

async fn shred_message_handler(
    timeout: oneshot::Receiver<bool>,
    endpoint: String,
    token: Option<String>,
    m_tx: mpsc::Sender<LatencyCheckerInput>,
) {
    let connection = Endpoint::from_shared(endpoint.clone())
        .unwrap()
        .keep_alive_while_idle(true)
        .http2_keep_alive_interval(Duration::from_secs(5))
        .keep_alive_timeout(Duration::from_secs(10))
        .tcp_keepalive(Some(Duration::from_secs(15)))
        .connect_timeout(Duration::from_secs(5));

    let endpoint = Arc::new(endpoint);

    let channel = connection.connect().await.unwrap();
    let mut client = ShredstreamProxyClient::new(channel);

    let mut request = Request::new(SubscribeEntriesRequest {});
    if let Some(token) = token {
        let metadata_value = MetadataValue::from_str(&token).unwrap();
        request.metadata_mut().insert("x-token", metadata_value);
    }

    let mut stream = client
        .subscribe_entries(request)
        .await
        .unwrap()
        .into_inner();
    tokio::select! {
        _ = timeout => {
            println!("Timeout reached, ending stream...");
        }
        _ = async {
            while let Some(message) = stream.next().await {
                match message {
                    Ok(msg) => {
                    let entries = match bincode::deserialize::<Vec<Entry>>(&msg.entries) {
                    Ok(e) => e,
                    Err(e) => {
                        eprintln!("Deserialization failed: {e}");
                        continue;
                    }
                };

                // println!(
                //     "slot {}, entries: {}, transactions: {}",
                //     msg.slot,
                //     entries.len(),
                //     entries.iter().map(|e| e.transactions.len()).sum::<usize>()
                // );

                for entry in entries {
                         let current_time_millis = Utc::now().timestamp_millis() as u64;
                         for txn in entry.transactions {
                            let sig = txn.signatures[0].to_string();
                            // info!("received txn {} at {} from node {}", sig, current_time_millis, endpoint);
                            _ = m_tx.send(LatencyCheckerInput {
                                signature: sig,
                                timestamp: current_time_millis,
                                node: endpoint.clone(),
                                m_type: 0,
                            }).await;
                         }
                }

                    }
                     Err(e) => {
                eprintln!("stream error: {e}");
                return Err(Box::new(e));
            }
                }
            }
            Ok(())
        } => {}
    }
}
