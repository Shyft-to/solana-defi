use {
    backoff::{future::retry, ExponentialBackoff},
    clap::Parser,
    futures::{sink::SinkExt, stream::StreamExt},
    log::{error, info, warn},
    std::{collections::HashMap, time::{Duration, Instant}},
    tonic::transport::channel::ClientTlsConfig,
    yellowstone_grpc_client::{GeyserGrpcClient, Interceptor},
    yellowstone_grpc_proto::prelude::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
        SubscribeRequestFilterTransactions, SubscribeRequestPing,
    },
};

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

    #[clap(long, env = "LOG_SIG", default_value = "true", help = "Print transaction signatures")]
    log_sig: bool,

    #[clap(long, env = "RUN_DURATION_MINS", help = "Auto-stop after this many minutes (omit to run forever)")]
    run_duration_mins: Option<u64>,
}

impl Args {
    async fn connect(&self) -> anyhow::Result<GeyserGrpcClient<impl Interceptor>> {
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

    if let Some(mins) = args.run_duration_mins {
        info!("Will auto-stop after {} minutes", mins);
    }

    let run = retry(ExponentialBackoff::default(), || async {
        info!("Connecting to {}", args.endpoint);

        let mut client = args
            .connect()
            .await
            .map_err(backoff::Error::transient)?;

        let (mut sink, mut stream) = client
            .subscribe_with_request(Some(args.subscribe_request()))
            .await
            .map_err(|e| backoff::Error::transient(anyhow::anyhow!(e)))?;

        info!("Subscribed — waiting for transactions...");

        let mut ping_id: i32 = 0;
        let idle_timeout = Duration::from_secs(30);
        let stats_interval = Duration::from_secs(5);
        let mut tx_count: u64 = 0;
        let mut total_count: u64 = 0;
        let mut window_start = Instant::now();

        loop {
            match tokio::time::timeout(idle_timeout, stream.next()).await {
                Err(_) => {
                    warn!("No messages received for {}s — reconnecting", idle_timeout.as_secs());
                    return Err(backoff::Error::transient(anyhow::anyhow!("stream idle timeout")));
                }
                Ok(None) => {
                    warn!("Stream closed by server — reconnecting");
                    return Err(backoff::Error::transient(anyhow::anyhow!("stream ended unexpectedly")));
                }
                Ok(Some(Err(e))) => {
                    error!("Stream error: {e} — reconnecting");
                    return Err(backoff::Error::transient(anyhow::anyhow!(e)));
                }
                Ok(Some(Ok(update))) => match update.update_oneof {
                    Some(UpdateOneof::Transaction(tx)) => {
                        let sig = tx
                            .transaction
                            .as_ref()
                            .and_then(|t| t.transaction.as_ref())
                            .and_then(|t| t.signatures.first())
                            .map(|b| bs58::encode(b).into_string())
                            .unwrap_or_else(|| "<unknown>".to_string());

                        if args.log_sig {
                            info!("{}", sig);
                        }

                        tx_count += 1;
                        total_count += 1;
                        let elapsed = window_start.elapsed();
                        if elapsed >= stats_interval {
                            let tps = tx_count as f64 / elapsed.as_secs_f64();
                            info!("-----> throughput: {:.1} tx/s | total transactions: {} <------\n", tps, total_count);
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
    });

    if let Some(mins) = args.run_duration_mins {
        let deadline = Duration::from_secs(mins * 60);
        match tokio::time::timeout(deadline, run).await {
            Ok(result) => result?,
            Err(_) => info!("Run duration of {} minute(s) reached — stopping", mins),
        }
    } else {
        run.await?;
    }

    Ok(())
}
