use {
    chrono::Utc,
    std::sync::Arc,
    clap::{Parser, ValueEnum},
    futures::{sink::SinkExt, stream::StreamExt},
    log::{error, info},
    maplit::hashmap,
    solana_sdk::signature::Signature,
    std::{
        collections::{HashMap, HashSet},
        env,
    },
    tonic::transport::channel::ClientTlsConfig,
    yellowstone_grpc_client::GeyserGrpcClient,
    yellowstone_grpc_proto::prelude::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
        SubscribeRequestFilterBlocksMeta, SubscribeRequestFilterTransactions,
    },
    tokio::sync::{oneshot,mpsc},
};

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long)]
    #[arg(use_value_delimiter = true)]
    /// Service endpoint
    endpoints: Vec<String>,

    #[clap(long)]
    x_token: Option<String>,

    /// Commitment level: processed, confirmed or finalized
    #[clap(long)]
    commitment: Option<ArgsCommitment>,

    /// Filter vote transactions
    #[clap(long)]
    vote: Option<bool>,

    /// Filter failed transactions
    #[clap(long)]
    failed: Option<bool>,

    /// Filter by transaction signature
    #[clap(long)]
    signature: Option<String>,

    /// Filter included account in transactions
    #[clap(long)]
    account_include: Vec<String>,

    /// Filter excluded account in transactions
    #[clap(long)]
    account_exclude: Vec<String>,

    /// Filter required account in transactions
    #[clap(long)]
    account_required: Vec<String>,
}

#[derive(Debug, Clone, Copy, Default, ValueEnum)]
enum ArgsCommitment {
    #[default]
    Processed,
    Confirmed,
    Finalized,
}

impl From<ArgsCommitment> for CommitmentLevel {
    fn from(commitment: ArgsCommitment) -> Self {
        match commitment {
            ArgsCommitment::Processed => CommitmentLevel::Processed,
            ArgsCommitment::Confirmed => CommitmentLevel::Confirmed,
            ArgsCommitment::Finalized => CommitmentLevel::Finalized,
        }
    }
}

#[derive(Eq, Hash, PartialEq, Default, Debug)]
struct Timing {
    sig: String,
    timestamp: u64,
    node: Arc<String>,
}
#[derive(Default, Debug)]
struct LatencyChecker {
    txns: HashMap<String, HashSet<Timing>>,
    //slots: HashMap<String, HashSet<Timing>>,
    blocks: HashMap<String, HashSet<Timing>>,
}
struct LatencyCheckerInput {
    signature: String,
    timestamp: u64,
    node: Arc<String>,
    m_type: u8,
}

#[derive(Default, Debug)]
struct LatencyReportLag {
    count: u64,
    time_taken: u64,
}
impl LatencyChecker {
    // m_type 0
    fn add_txn(&mut self, signature: String, timestamp: u64, node: Arc<String>) {
        let timing = Timing { sig: signature.clone(), timestamp, node };
        if let Some(set) = self.txns.get_mut(&signature) {
            set.insert(timing);
        } else {
            let mut set = HashSet::new();
            set.insert(timing);
            self.txns.insert(signature, set);
        }
    }
    // m_type 1
    fn add_block(&mut self, block: String, timestamp: u64, node: Arc<String>) {
        let timing = Timing { sig: block.clone(), timestamp, node };
        if let Some(set) = self.blocks.get_mut(&block) {
            set.insert(timing);
        } else {
            let mut set = HashSet::new();
            set.insert(timing);
            self.blocks.insert(block, set);
        }
    }

    async fn listen_messages(&mut self, mut m_rx: mpsc::Receiver<LatencyCheckerInput>) {
        while let Some(m) = m_rx.recv().await {
            if m.m_type == 0 {
                self.add_txn(m.signature, m.timestamp, m.node);
            } else {
                self.add_block(m.signature, m.timestamp, m.node);
            }
        }
    }

    fn get_report(&self) {
        let mut txns_compare: HashMap<Arc<String>, LatencyReportLag> = HashMap::new();// map of node vs (fastest,slowest) between others
        let blocks_compare: HashMap<Arc<String>, LatencyReportLag> = HashMap::new();

        for v in self.txns.values() {
            let mut values: Vec<_> = v.into_iter().collect();
            values.sort_by(|a,b| a.timestamp.cmp(&b.timestamp));
            let fastest = values.first();
            let slowest = values.last();

            if let Some(f) = fastest {
                let s_tmp = slowest.map(|s| s.timestamp).unwrap_or(0);
                info!("Fastes: {}, slow: {}", f.timestamp, s_tmp);
                if let Some(c) = txns_compare.get_mut(&f.node) {
                    c.count += 1;
                    c.time_taken += s_tmp - f.timestamp;
                } else {
                    txns_compare.insert(f.node.clone(), LatencyReportLag {
                        count: 1,
                        time_taken: s_tmp - f.timestamp,
                    });
                }
            }
        }

        info!("Final results:");
        info!("----------  Transactions --------");
        for (k,v) in txns_compare {
            info!("{:?}, count: {}, avg_gain: {}", k, v.count, v.time_taken/v.count);
        }
        info!("----------  Blocks --------");
        info!("{{Node name/ip}}: ({{times faster}}, {{times slower}})");
        info!("{:?}", blocks_compare);
    }
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() {
    env::set_var(
        env_logger::DEFAULT_FILTER_ENV,
        env::var_os(env_logger::DEFAULT_FILTER_ENV).unwrap_or_else(|| "info".into()),
    );
    env_logger::init();

    let mut latency_checker = LatencyChecker::default();

    let timeout = tokio::time::sleep(tokio::time::Duration::from_secs(10));
    let args = Args::parse();
    info!("Args: {:?}", args);
    let mut shutdown_sig = Vec::new();
    let (m_tx, m_rx) = mpsc::channel(100_000);

    for endpoint in args.endpoints {
        let token = args.x_token.clone();
        let (tx, rx) = oneshot::channel();
        shutdown_sig.push(tx);
        let m_tx = m_tx.clone();
        
        info!("starting {endpoint}");
        tokio::spawn(async move {
            grpc_message_handler(rx, endpoint,token, m_tx).await;
        });
    }

    tokio::select! {
        _ = latency_checker.listen_messages(m_rx) => {}
        _ = timeout => {
            for sig in shutdown_sig {
                _ = sig.send(true);
            }
            latency_checker.get_report();
        }
    }
}


async fn grpc_message_handler(timeout: oneshot::Receiver<bool>, endpoint: String, token: Option<String>, m_tx: mpsc::Sender<LatencyCheckerInput>) {
    let mut client = GeyserGrpcClient::build_from_shared(endpoint.clone()).unwrap()
        .x_token(token).unwrap()
        .tls_config(ClientTlsConfig::new().with_native_roots()).unwrap()
        .send_compressed(tonic::codec::CompressionEncoding::Gzip)
        .accept_compressed(tonic::codec::CompressionEncoding::Gzip)
        .connect()
        .await.unwrap();
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
    .await.unwrap();
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
                            info!("received txn1 {} at {}", sig, current_time_millis);
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
