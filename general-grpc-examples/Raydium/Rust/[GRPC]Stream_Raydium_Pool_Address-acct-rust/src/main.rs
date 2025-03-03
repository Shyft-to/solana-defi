use {
    backoff::{future::retry, ExponentialBackoff},
    clap::Parser as ClapParser,
    futures::{future::TryFutureExt, sink::SinkExt, stream::StreamExt},
    log::{error, info},
    serde_json::json,
    serde_json::Value,
    solana_sdk::{pubkey::Pubkey},
    //solana_transaction_status::UiTransactionEncoding,
    std::{collections::HashMap, env, sync::Arc, time::Duration},
    tokio::sync::Mutex,
    tonic::transport::channel::ClientTlsConfig,
    yellowstone_grpc_client::{GeyserGrpcClient, Interceptor},
    yellowstone_grpc_proto::prelude::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest,
        SubscribeRequestFilterAccounts, SubscribeRequestPing,SubscribeUpdateAccountInfo
    },
 //   yellowstone_vixen_core::Parser as VixenParser,
    yellowstone_vixen_parser::raydium::{AccountParser as RaydiumParser, RaydiumProgramState},
};

type AccountFilterMap = HashMap<String, SubscribeRequestFilterAccounts>;

#[derive(Debug, Clone, ClapParser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, help = "gRPC endpoint")]
    /// Service endpoint
    endpoint: String,

    #[clap(long, help = "X-Token")]
    x_token: String,

    #[clap(long, help = "Pool address of the raydium pool to subscribe to")]
    pool_address: String,
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
        let mut accounts: AccountFilterMap = HashMap::new();

        accounts.insert(
            "client".to_owned(),
            SubscribeRequestFilterAccounts {
                nonempty_txn_signature: None,
                account: vec![],
                owner: vec![self.pool_address.clone()],
                filters: vec![],
            },
        );

        Ok(SubscribeRequest {
            slots: HashMap::default(),
            accounts : accounts,
            transactions: HashMap::default(),
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
    env::set_var(
        env_logger::DEFAULT_FILTER_ENV,
        env::var_os(env_logger::DEFAULT_FILTER_ENV).unwrap_or_else(|| "info".into()),
    );
    env_logger::init();

    let args = Args::parse();
    let zero_attempts = Arc::new(Mutex::new(true));

    // The default exponential backoff strategy intervals:
    // [500ms, 750ms, 1.125s, 1.6875s, 2.53125s, 3.796875s, 5.6953125s,
    // 8.5s, 12.8s, 19.2s, 28.8s, 43.2s, 64.8s, 97s, ... ]
    retry(ExponentialBackoff::default(), move || {
        let args = args.clone();
        println!("args: {:?}", args);
        let zero_attempts = Arc::clone(&zero_attempts);

        async move {
            let mut zero_attempts = zero_attempts.lock().await;
            if *zero_attempts {
                *zero_attempts = false;
            } else {
                info!("Retry to connect to the server");
            }
            drop(zero_attempts);

            let client = args.connect().await.map_err(backoff::Error::transient)?;
            info!("Connected");

            let request = args
                .get_raydium_pool_subscribe_request()
                .map_err(backoff::Error::Permanent)?;

            geyser_subscribe(client, request)
                .await
                .map_err(backoff::Error::transient)?;

            Ok::<(), backoff::Error<anyhow::Error>>(())
        }
        .inspect_err(|error| error!("failed to connect: {error}"))
    })
    .await
    .map_err(Into::into)
}

async fn geyser_subscribe(
    mut client: GeyserGrpcClient<impl Interceptor>,
    request: SubscribeRequest,
) -> anyhow::Result<()> {
    let (mut subscribe_tx, mut stream) = client.subscribe_with_request(Some(request)).await?;

    info!("stream opened");
    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => {
                match msg.update_oneof {
                    Some(UpdateOneof::Account(msg)) => {
                         let account = msg
                         .account
                         .ok_or(anyhow::anyhow!("no account in the message"))?;
                        let data = RaydiumProgramState::try_unpack(&&account.data);
                        let value = create_pretty_account(account.clone())?;   
                        info!("Pretty account message: {:?}",value);
                        info!("Data: {:?}",data);     
                      //  let data = RaydiumParser.parse(unsafe{std::mem::transmute(&msg.account)}).await;
                      // info!("Raydium: {:?}",data)
                        // match RaydiumParser
                        //     .parse(unsafe{ std::mem::transmute(&msg)})
                        //     .await
                        // {
                        //     Ok(RaydiumProgramState::PoolState(pool_state)) => {
                        //         info!("PoolState {:?}", pool_state);
                        //         // Successfully parsed PoolState
                        //         info!(
                        //             "Slot: {:?}\tSwap price: {:.2}",
                        //             msg.slot,
                        //             get_raydium_sol_usd_price(pool_state.sqrt_price_x64)
                        //         );
                        //     }
                        //     Ok(_) => {
                        //         // Other variants, add handling if needed
                        //         info!("Received non-PoolState RaydiumProgramState");
                        //     }
                        //     Err(e) => {
                        //         // Failed to parse Raydium account data
                        //         error!("Failed to parse Raydium account data: {:?}",e);
                        //     }
                        // }
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
fn create_pretty_account(account: SubscribeUpdateAccountInfo) -> anyhow::Result<Value> {
    Ok(json!({
        "pubkey": Pubkey::try_from(account.pubkey).map_err(|_| anyhow::anyhow!("invalid account pubkey"))?.to_string(),
        "lamports": account.lamports,
        "owner": Pubkey::try_from(account.owner).map_err(|_| anyhow::anyhow!("invalid account owner"))?.to_string(),
        "executable": account.executable,
        "rentEpoch": account.rent_epoch,
        "writeVersion": account.write_version,
        "txnSignature": account.txn_signature.map(|sig| bs58::encode(sig).into_string()),
    }))
}
// /**
//  * Convert sqrt_price_x64 to normal price
//  *
//  * This does not consider mint decimals. Nor this is the right way to calculate price.
//  * Use only for SOL/USD
//  */
// fn get_raydium_sol_usd_price(sqrt_price_x64: u128) -> f64 {
//     let sqrt_price = sqrt_price_x64 as f64 / (1u128 << 64) as f64;
//     sqrt_price * sqrt_price * 1000.0
// }