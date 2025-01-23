use {
    anyhow::Context,
    backoff::{future::retry, ExponentialBackoff},
    clap::Parser as ClapParser,
    futures::{future::TryFutureExt, sink::SinkExt, stream::StreamExt},
    log::{error, info},
    serde_json::json,
    serde_json::Value,
    
    solana_sdk::{signature::Signature},
    solana_transaction_status::UiTransactionEncoding,
    std::{collections::HashMap, env, sync::Arc, time::Duration},
    tokio::sync::Mutex,
    tonic::transport::channel::ClientTlsConfig,
    yellowstone_grpc_client::{GeyserGrpcClient, Interceptor},
    yellowstone_grpc_proto::prelude::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest, SubscribeRequestPing,
        SubscribeRequestFilterTransactions,SubscribeUpdateTransactionInfo
    },
    supabase_rs::SupabaseClient,
    supabase_rs::errors::ErrorTypes,
    yellowstone_grpc_proto::convert_from
};
type TransactionsFilterMap = HashMap<String, SubscribeRequestFilterTransactions>;
// const client : Result<SupabaseClient, ErrorTypes> = SupabaseClient::new(
//     "https://psqoveyaszqvuiteplsi.supabase.co".to_string(), "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InBzcW92ZXlhc3pxdnVpdGVwbHNpIiwicm9sZSI6ImFub24iLCJpYXQiOjE3Mzc1Mjk5NDgsImV4cCI6MjA1MzEwNTk0OH0.q3WMH4FKh78jd4fDnsTHugAAyoi4D-k1l2FZwtvTN2s".to_string()
// );
#[derive(Debug, Clone, ClapParser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, help = "gRPC endpoint")]
    /// Service endpoint
    endpoint: String,

    #[clap(long, help = "X-Token")]
    x_token: String,

    #[clap(long, help = "Program Id to subscribe to")]
    address: String,
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

    pub fn get_raydium_pool_subscribe_request(&self) -> anyhow::Result<SubscribeRequest> {
        let mut transactions : TransactionsFilterMap = HashMap::new();
        transactions.insert(
            "client".to_owned(),
            SubscribeRequestFilterTransactions {
                vote: None,
                failed: Some(false),
                signature: None,
                account_include:vec![self.address.to_string()] ,
                account_exclude: vec![],
                account_required: vec![],
            },
        );

        Ok(SubscribeRequest {
            slots: HashMap::default(),
            accounts: HashMap::default(),
            transactions,
            transactions_status: HashMap::default(),
            entry: HashMap::default(),
            blocks: HashMap::default(),
            blocks_meta: HashMap::default(),
            commitment: Some(CommitmentLevel::Processed as i32),
            accounts_data_slice: Vec::default(),
            ping: None,
            from_slot: None,
        })
    }
}
// .ok_or(anyhow::anyhow!("no created_at in the message"))?
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = SupabaseClient::new(
        "https://psqoveyaszqvuiteplsi.supabase.co".to_string(),"eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJpc3MiOiJzdXBhYmFzZSIsInJlZiI6InBzcW92ZXlhc3pxdnVpdGVwbHNpIiwicm9sZSI6ImFub24iLCJpYXQiOjE3Mzc1Mjk5NDgsImV4cCI6MjA1MzEwNTk0OH0.q3WMH4FKh78jd4fDnsTHugAAyoi4D-k1l2FZwtvTN2s".to_string()
    ).unwrap();
     let result = insert_example(client).await;
     info!("Result: {:?}",result);
     Ok(())
    // env::set_var(
    //     env_logger::DEFAULT_FILTER_ENV,
    //     env::var_os(env_logger::DEFAULT_FILTER_ENV).unwrap_or_else(|| "info".into()),
    // );
    // env_logger::init();

    // let args = Args::parse();
    // let zero_attempts = Arc::new(Mutex::new(true));

    // // The default exponential backoff strategy intervals:
    // // [500ms, 750ms, 1.125s, 1.6875s, 2.53125s, 3.796875s, 5.6953125s,
    // // 8.5s, 12.8s, 19.2s, 28.8s, 43.2s, 64.8s, 97s, ... ]
    // retry(ExponentialBackoff::default(), move || {
    //     let args = args.clone();
    //     println!("args: {:?}", args);
    //     let zero_attempts = Arc::clone(&zero_attempts);

    //     async move {
    //         let mut zero_attempts = zero_attempts.lock().await;
    //         if *zero_attempts {
    //             *zero_attempts = false;
    //         } else {
    //             info!("Retry to connect to the server");
    //         }
    //         drop(zero_attempts);

    //         let client = args.connect().await.map_err(backoff::Error::transient)?;
    //         info!("Connected");

    //         let request = args
    //             .get_raydium_pool_subscribe_request()
    //             .map_err(backoff::Error::Permanent)?;

    //         geyser_subscribe(client, request)
    //             .await
    //             .map_err(backoff::Error::transient)?;

    //         Ok::<(), backoff::Error<anyhow::Error>>(())
    //     }
    //     .inspect_err(|error| error!("failed to connect: {error}"))
    // })
    // .await
    // .map_err(Into::into)
}

async fn geyser_subscribe(
    mut _client: GeyserGrpcClient<impl Interceptor>,
    request: SubscribeRequest,
) -> anyhow::Result<()> {
    let (mut subscribe_tx, mut stream) = _client.subscribe_with_request(Some(request)).await?;

    info!("stream opened");
    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => {
                match msg.update_oneof {
                    Some(UpdateOneof::Transaction(msg)) => {
                         let tx = msg
                         .transaction
                         .ok_or(anyhow::anyhow!("no transaction in the message"))?;
                     let value = create_pretty_transaction(tx)?;
                     //let meta = &value["tx"]["meta"];

                    }                   
                    Some(UpdateOneof::Ping(_)) => {
                        // This is necessary to keep load balancers that expect client pings alive.
                        subscribe_tx
                            .send(SubscribeRequest {
                                ping: Some(SubscribeRequestPing { id: 1 }),
                                ..Default::default()
                            })
                            .await?;
                    }
                    Some(UpdateOneof::Pong(_)) => {
                        // Handle pong response if needed
                    }
                    None => {
                        error!("update not found in the message");
                        break;
                    }
                    _ => {}
                }
            }
            Err(error) => {
                error!("error: {error:?}");
                break;
            }
        }
    }
    info!("stream closed");
    Ok(())
}
fn create_pretty_transaction(tx: SubscribeUpdateTransactionInfo) -> anyhow::Result<Value> {
    Ok(json!({
        "signature": Signature::try_from(tx.signature.as_slice()).context("invalid signature")?.to_string(),
        "isVote": tx.is_vote,
        "tx": convert_from::create_tx_with_meta(tx)
            .map_err(|error| anyhow::anyhow!(error))
            .context("invalid tx with meta")?
            .encode(UiTransactionEncoding::Base64, Some(u8::MAX), true)
            .context("failed to encode transaction")?,
    }))
}
async fn insert_example(
    client: SupabaseClient
 ) -> Result<(), String> {
     let insert_result = client
         .insert_if_unique(
             "test",
             json!({
                 "dog": "value_test"
             }),
        ).await;
        match insert_result {
            Ok(_) => Ok(()),  // Return Ok if insertion was successful
            Err(e) => Err(e.to_string()),  // Return Err with error message if insertion failed
        }
    }
