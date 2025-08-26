use chrono::{DateTime, TimeZone, Utc};
use clap::Parser;
use futures::StreamExt;
use prost_types::Timestamp;
use std::{
    collections::HashMap,
    process,
    str::FromStr,
    time::{Duration, Instant},
    vec,
};
use tokio::sync::mpsc;

use tonic::{
    metadata::MetadataValue,
    transport::{Channel, ClientTlsConfig},
    Request,
};
use yellowstone_grpc_proto::geyser::{
    geyser_client::GeyserClient, CommitmentLevel, SubscribeRequest,
    SubscribeRequestFilterTransactions,
};

type AnyError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug, Clone, Parser)]
#[clap(author, version, about)]
struct Args {
    #[clap(long, default_value_t = String::from("http://127.0.0.1:10000"))]
    endpoint_a: String,

    #[clap(long)]
    endpoint_b: Option<String>,

    #[clap(long)]
    x_token_a: Option<String>,

    #[clap(long)]
    x_token_b: Option<String>,

    #[clap(long, default_value_t = 30)]
    timeout_dur: u64,
}

async fn run_client(
    name: String,
    endpoint: String,
    token: Option<String>,
    tx: mpsc::Sender<(String, usize, i64)>,
) -> Result<(), AnyError> {
    let channel = Channel::from_shared(endpoint.clone())?
        .tls_config(ClientTlsConfig::new().with_enabled_roots())?
        .connect()
        .await?;

    let interceptor = move |mut req: Request<()>| {
        if let Some(t) = &token {
            if let Ok(meta_val) = MetadataValue::from_str(t) {
                req.metadata_mut().insert("x-token", meta_val);
            }
        }
        Ok(req)
    };
    let mut client = GeyserClient::with_interceptor(channel, interceptor)
        .max_decoding_message_size(128 * 1024 * 1024)
        .max_encoding_message_size(128 * 1024 * 1024);

    let request = SubscribeRequest {
        slots: HashMap::new(),
        accounts: HashMap::new(),
        transactions: {
            let mut transactions = HashMap::new();
            transactions.insert(
                "slot_transaction_updates".to_string(),
                SubscribeRequestFilterTransactions {
                    vote: Some(false),
                    failed: Some(false),
                    account_include: vec![],
                    account_exclude: vec![],
                    account_required: vec![],
                    signature: None,
                },
            );
            transactions
        },
        transactions_status: HashMap::new(),
        entry: HashMap::new(),
        blocks: HashMap::new(),
        blocks_meta: HashMap::new(),
        commitment: Some(CommitmentLevel::Processed as i32),
        accounts_data_slice: vec![],
        ping: None,
        from_slot: None,
    };

    let request_stream = futures::stream::iter(vec![request]);
    let mut response_stream = client
        .subscribe(Request::new(request_stream))
        .await?
        .into_inner();

    println!("Streaming from {name} ({endpoint})...");

    let mut count = 0;
    let mut last_print = Instant::now();
    let print_interval = Duration::from_secs(1);

    while let Some(update) = response_stream.next().await {
        match update {
            Ok(resp) => {
                count += 1;
                let now_ms = Utc::now().timestamp_millis();

                let created_at = resp.created_at.unwrap_or(Timestamp {
                    seconds: 0,
                    nanos: 0,
                });

                let datetime: DateTime<Utc> = Utc
                    .timestamp_opt(created_at.seconds, created_at.nanos as u32)
                    .unwrap();
                let message_ms = datetime.timestamp_millis();
                let latency_ms = now_ms - message_ms;

                if last_print.elapsed() >= print_interval {
                    let _ = tx.send((name.clone(), count, latency_ms)).await;
                    count = 0;
                    last_print = Instant::now();
                }
            }
            Err(e) => {
                eprintln!("Stream error on {name}: {:?}", e);
                break;
            }
        }
    }

    Ok(())
}

#[tokio::main(flavor = "multi_thread", worker_threads = 4)]
async fn main() -> Result<(), AnyError> {
    let args = Args::parse();

    let (tx, mut rx) = mpsc::channel::<(String, usize, i64)>(100);

    tokio::spawn(run_client(
        "endpointA".to_string(),
        args.endpoint_a.clone(),
        args.x_token_a.clone(),
        tx.clone(),
    ));

    if let Some(endpoint_b) = args.endpoint_b.clone() {
        tokio::spawn(run_client(
            "endpointB".to_string(),
            endpoint_b,
            args.x_token_b.clone(),
            tx.clone(),
        ));
    }

    let timeout_dur = args.timeout_dur;

    drop(tx);

    let stats_task = tokio::spawn(async move {
        let mut stats: HashMap<String, (usize, i64)> = HashMap::new();
        let mut totals: HashMap<String, usize> = HashMap::new();
        let start = Instant::now();

        while let Some((name, count, latency)) = rx.recv().await {
            *totals.entry(name.clone()).or_insert(0) += count;
            stats.insert(name.clone(), (count, latency));

            if let Some(a) = stats.get("endpointA") {
                if let Some(b) = stats.get("endpointB") {
                    println!(
                        "{}: {} msg/s (lat {} ms) | {}: {} msg/s (lat {} ms)",
                        args.endpoint_a,
                        a.0,
                        a.1,
                        args.endpoint_b.clone().unwrap_or("".to_string()),
                        b.0,
                        b.1
                    );
                    stats.clear();
                } else {
                    println!("{}: {} msg/s (lat {} ms)", args.endpoint_a, a.0, a.1);
                    stats.clear();
                }
            }

            if start.elapsed() >= Duration::from_secs(timeout_dur) {
                println!("\n==== Totals after {}s ====", timeout_dur);
                println!(
                    "{} total messages: {}",
                    args.endpoint_a,
                    totals.get("endpointA").unwrap_or(&0)
                );
                if let Some(endpoint_b) = &args.endpoint_b {
                    println!(
                        "{} total messages: {}",
                        endpoint_b,
                        totals.get("endpointB").unwrap_or(&0)
                    );
                }
                break;
            }
        }
    });

    stats_task.await?;

    process::exit(0);
}

