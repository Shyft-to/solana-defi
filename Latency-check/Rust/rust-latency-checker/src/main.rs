use {
    chrono::{DateTime, Utc},
    clap::{Parser, ValueEnum},
    futures::{sink::SinkExt, stream::StreamExt},
    log::{error, info},
    maplit::hashmap,
    solana_sdk::signature::Signature,
    std::{
        collections::{BTreeMap, HashMap},
        env,
    },
    tonic::transport::channel::ClientTlsConfig,
    yellowstone_grpc_client::GeyserGrpcClient,
    yellowstone_grpc_proto::prelude::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
        SubscribeRequestFilterBlocksMeta, SubscribeRequestFilterTransactions,
    },
};

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, default_value_t = String::from("http://127.0.0.1:10000"))]
    /// Service endpoint
    endpoint: String,

    #[clap(long)]
    x_token: Option<String>,

    /// Commitment level: processed, confirmed or finalized
    #[clap(long)]
    commitment: Option<ArgsCommitment>,

    /// Timeout in milliseconds (default: 1000)
    #[clap(long, default_value_t = 60)]
    timeout: u64, // Keeping it u64 for flexibility

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

pub struct LatencyCollector {
    min_latency: u64,
    max_latency: u64,
    total_latency: u64,
    total_txns: u64,
    avg_latency: u64,
    less_than_400: u64,
    less_than_800: u64,
    less_than_1000: u64,
    less_than_1200: u64,
    less_than_1500: u64,
    less_than_1800: u64,
    less_than_2000: u64,
    more_than_2000: u64,
}

impl LatencyCollector {
    pub fn new() -> Self {
        LatencyCollector {
            min_latency: u64::MAX,
            max_latency: u64::MIN,
            total_latency: 0,
            total_txns: 0,
            avg_latency: 0,
            less_than_400: 0,
            less_than_800: 0,
            less_than_1000: 0,
            less_than_1200: 0,
            less_than_1500: 0,
            less_than_1800: 0,
            less_than_2000: 0,
            more_than_2000: 0,
        }
    }

    pub fn collect_data(&mut self, transaction_time: u64, time_received: u64) {
        let latency = self.calculate_latency(transaction_time, time_received);
        
        if latency < self.min_latency {
            self.min_latency = latency;
        }
        if latency > self.max_latency {
            self.max_latency = latency;
        }
        
        self.total_latency += latency;
        self.total_txns += 1;
        self.avg_latency = self.total_latency / self.total_txns;
        
        match latency {
            l if l < 400 => self.less_than_400 += 1,
            l if l < 800 => self.less_than_800 += 1,
            l if l < 1000 => self.less_than_1000 += 1,
            l if l < 1200 => self.less_than_1200 += 1,
            l if l < 1500 => self.less_than_1500 += 1,
            l if l < 1800 => self.less_than_1800 += 1,
            l if l < 2000 => self.less_than_2000 += 1,
            _ => self.more_than_2000 += 1,
        }
    }

    fn calculate_latency(&self, transaction_time: u64, time_received: u64) -> u64 {
        time_received.saturating_sub(transaction_time)
    }
    pub fn generate_report(&self) {
        println!("**********************************************\n");
        println!("*  Min Latency: {}", self.min_latency);
        println!("*  Max Latency: {}", self.max_latency);
        let avg_latency = if self.total_txns > 0 {
            self.total_latency as f64 / self.total_txns as f64
        } else {
            0.0
        };
        println!("*  Average Latency: {:.2}", avg_latency);
        println!("*  Total Txns: {}", self.total_txns);
        println!("*  0-399ms: {} | {:.2} %", self.less_than_400, (self.less_than_400 as f64 / self.total_txns as f64) * 100.0 );
        println!("*  400-799ms: {} | {:.2} %", self.less_than_800, (self.less_than_800 as f64 / self.total_txns as f64) * 100.0) ;
        println!("*  800-999ms: {} | {:.2} %", self.less_than_1000, (self.less_than_1000 as f64 / self.total_txns as f64) * 100.0);
        println!("*  1000-1199ms: {} | {:.2} %", self.less_than_1200, (self.less_than_1200 as f64 / self.total_txns as f64) * 100.0);
        println!("*  1200-1499ms: {} | {:.2} %", self.less_than_1500, (self.less_than_1500 as f64 / self.total_txns as f64) * 100.0);
        println!("*  1500-1799ms: {} | {:.2} %", self.less_than_1800, (self.less_than_1800 as f64 / self.total_txns as f64) * 100.0);
        println!("*  1800-2000ms: {} | {:.2} %", self.less_than_2000, (self.less_than_2000 as f64 / self.total_txns as f64) * 100.0);
        println!("*  2000ms+: {} | {:.2} %", self.more_than_2000, (self.more_than_2000 as f64 / self.total_txns as f64) * 100.0);
        println!("\n**********************************************");
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var(
        env_logger::DEFAULT_FILTER_ENV,
        env::var_os(env_logger::DEFAULT_FILTER_ENV).unwrap_or_else(|| "info".into()),
    );
    env_logger::init();

    let args = Args::parse();

    let mut client = GeyserGrpcClient::build_from_shared(args.endpoint)?
        .x_token(args.x_token)?
        .tls_config(ClientTlsConfig::new().with_native_roots())?
        .connect()
        .await?;
    let (mut subscribe_tx, mut stream) = client.subscribe().await?;

    let commitment: CommitmentLevel = args.commitment.unwrap_or_default().into();
    subscribe_tx
        .send(SubscribeRequest {
            slots: HashMap::new(),
            accounts: HashMap::new(),
            transactions: HashMap::new(),
            transactions_status: hashmap! { "".to_owned() => SubscribeRequestFilterTransactions {
                vote: Some(false),
                failed: Some(false),
                signature: args.signature,
                account_include: args.account_include,
                account_exclude: args.account_exclude,
                account_required: args.account_required,
            } },
            entry: HashMap::new(),
            blocks: HashMap::new(),
            blocks_meta: hashmap! { "".to_owned() => SubscribeRequestFilterBlocksMeta {} },
            commitment: Some(commitment as i32),
            accounts_data_slice: vec![],
            ping: None,
            from_slot: None,
        })
        .await?;

    let mut _lc = LatencyCollector::new();

    let mut messages: BTreeMap<u64, (Option<u64>, Vec<(String, u64)>)> = BTreeMap::new();
    let timeout = tokio::time::sleep(tokio::time::Duration::from_secs(args.timeout_dur));
    tokio::select! {
        _ = timeout => {
            println!("Timeout reached, ending stream...");
            _lc.generate_report();
        }
        _ = async {
            while let Some(message) = stream.next().await {
                match message {
                    Ok(msg) => match msg.update_oneof {
                        Some(UpdateOneof::TransactionStatus(tx)) => {
                            let entry = messages.entry(tx.slot).or_default();
                            //println!("entry: {:?}", entry);
                            let sig = Signature::try_from(tx.signature.as_slice())
                                .expect("valid signature from transaction")
                                .to_string();
                            let current_time_millis = Utc::now().timestamp_millis() as u64;
                            if let Some(timestamp) = entry.0 {
                                info!("received txn0 {} at {}", sig, timestamp);
                            } else {
                                entry.1.push((sig, current_time_millis));
                            }
                        }
                        Some(UpdateOneof::BlockMeta(block)) => {
                            let entry = messages.entry(block.slot).or_default();
                            entry.0 = block.block_time.map(|obj| {
                                //println!("block time: {}", obj.timestamp);
                                obj.timestamp as u64 * 1000
                            });
                            if let Some(timestamp) = entry.0 {
                                for (sig, txn_time) in &entry.1 {
                                    //let current_timestamp_millis = Utc::now().timestamp_millis();
                                    _lc.collect_data(timestamp, *txn_time);
                                    info!("received txn1 {} at {}", sig, timestamp);
                                    println!("\nlatency: {} ms",txn_time.saturating_sub(timestamp));
                                }
                            }

                            while let Some(slot) = messages.keys().next().cloned() {
                                if slot < block.slot - 20 {
                                    messages.remove(&slot);
                                } else {
                                    break;
                                }
                            }
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

    Ok(())
}

