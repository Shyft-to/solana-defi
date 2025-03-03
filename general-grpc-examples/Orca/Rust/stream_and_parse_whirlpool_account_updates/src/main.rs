use {
    backoff::{future::retry, ExponentialBackoff}, clap::Parser as ClapParser, futures::{
        future::TryFutureExt,
        sink::SinkExt,
        stream::StreamExt,
    }, log::{error, info},
        whirlpool_interface::accounts::{WhirlpoolsConfigExtension, WhirlpoolsConfigExtensionAccount,WHIRLPOOLS_CONFIG_EXTENSION_ACCOUNT_DISCM,WhirlpoolsConfig, WhirlpoolsConfigAccount, WHIRLPOOLS_CONFIG_ACCOUNT_DISCM, FeeTier, FeeTierAccount, FEE_TIER_ACCOUNT_DISCM, PositionBundle, PositionBundleAccount,POSITION_BUNDLE_ACCOUNT_DISCM, Position, PositionAccount, POSITION_ACCOUNT_DISCM, TickArray, TickArrayAccount, TICK_ARRAY_ACCOUNT_DISCM, TokenBadge, TokenBadgeAccount, TOKEN_BADGE_ACCOUNT_DISCM, Whirlpool, WhirlpoolAccount, WHIRLPOOL_ACCOUNT_DISCM},
        serde::Serialize, std::{
        collections::HashMap, env, sync::Arc, time::Duration
    }, tokio::sync::Mutex, tonic::transport::channel::ClientTlsConfig, yellowstone_grpc_client::{GeyserGrpcClient, Interceptor}, yellowstone_grpc_proto::{
        geyser::SubscribeRequestFilterAccounts,
        prelude::{
            subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest, SubscribeRequestPing,
        },
    }
};

type AccountFilterMap = HashMap<String, SubscribeRequestFilterAccounts>;

const WHIRLPOOL_PROGRAM_ID: &str = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";

#[derive(Debug, Clone, ClapParser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, help = "gRPC endpoint")]
    endpoint: String,

    #[clap(long, help = "X-Token")]
    x_token: String,
}



impl Args {
    async fn connect(&self) -> anyhow::Result<GeyserGrpcClient<impl Interceptor>> {
        GeyserGrpcClient::build_from_shared(self.endpoint.clone())?
            .x_token(Some(self.x_token.clone()))?
            .connect_timeout(Duration::from_secs(20))
            .timeout(Duration::from_secs(20))
            .tls_config(ClientTlsConfig::new().with_native_roots())?
            .max_decoding_message_size(1024 * 1024 * 1024)
            .connect()
            .await
            .map_err(Into::into)
    }

    pub fn get_txn_updates(&self) -> anyhow::Result<SubscribeRequest> {
        

        let mut accounts: AccountFilterMap = HashMap::new();

        accounts.insert(
            "accountData".to_owned(),
            SubscribeRequestFilterAccounts {
                account: vec![],
                owner: vec![WHIRLPOOL_PROGRAM_ID.to_string()],
                nonempty_txn_signature: None,
                filters: vec![]
            },
        );

        Ok(SubscribeRequest {
            accounts,
            slots: HashMap::default(),
            transactions: HashMap::default(),
            transactions_status: HashMap::default(),
            blocks: HashMap::default(),
            blocks_meta: HashMap::default(),
            entry: HashMap::default(),
            commitment: Some(CommitmentLevel::Processed as i32),
            accounts_data_slice: Vec::default(),
            ping: None,
            from_slot: None,
        })
    }
}


pub trait AccountData: std::fmt::Debug {}

#[derive(Debug,Default)]
pub struct EmptyAccount;

impl AccountData for EmptyAccount {}

#[derive(Debug, Serialize)]
pub enum DecodedAccount {
    WhirlpoolsConfig(WhirlpoolsConfig),
    WhirlpoolsConfigExtension(WhirlpoolsConfigExtension),
    FeeTier(FeeTier),
    PositionBundle(PositionBundle),
    Position(Position),
    TickArray(TickArray),
    TokenBadge(TokenBadge),
    Whirlpool(Whirlpool),
}

#[derive(Debug)]
pub struct AccountDecodeError {
    pub message: String,
}



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

            let request = args.get_txn_updates().map_err(backoff::Error::Permanent)?;

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
            Ok(msg) => match msg.update_oneof {
                
                Some(UpdateOneof::Account(account)) => {
                    
                    let slot = account.slot;
                    //let account_update = account.account;
                    if let Some(account_data) = account.account {
                        let pubkey_str = bs58::encode(&account_data.pubkey).into_string();
                        let owner = bs58::encode(&account_data.owner).into_string();
                        let lamports = account_data.lamports;
                        let executable = account_data.executable;
                
                        let decoded_account = match decode_account_data(&account_data.data) {
                            Ok(data) => data,
                            Err(e) => {
                                eprintln!("Failed 2to decode account data: {}", e.message);
                                continue;// Handle the error as needed
                            }
                        };
                
                        let account_info = serde_json::json!({
                            "pubkey": pubkey_str,
                            "lamports": lamports,
                            "owner": owner,
                            "executable": executable,
                            "slot": slot,
                            "decoded_data": decoded_account
                        });
                
                        println!("\nAccount Info: {}", account_info);
                    } else {
                        println!("Account data is None for slot: {}", slot);
                    }

                }
                Some(UpdateOneof::Ping(_)) => {
                    subscribe_tx
                        .send(SubscribeRequest {
                            ping: Some(SubscribeRequestPing { id: 1 }),
                            ..Default::default()
                        })
                        .await?;
                }
                Some(UpdateOneof::Pong(_)) => {}
                None => {
                    error!("update not found in the message");
                    break;
                }
                _ => {}
            },
            Err(error) => {
                error!("error: {error:?}");
                break;
            }
        }
    }

    info!("stream closed");
    Ok(())
}

pub fn decode_account_data(buf: &[u8]) -> Result<DecodedAccount, AccountDecodeError> {
    if buf.len() < 8 {
        return Err(AccountDecodeError {
            message: "Buffer too short to contain a valid discriminator.".to_string(),
        });
    }

    let discriminator: [u8; 8] = buf[..8].try_into().expect("Failed to extract first 8 bytes");

    match discriminator {
        WHIRLPOOLS_CONFIG_ACCOUNT_DISCM => {
            //println!("Whirlpool Config Account detected. Proceeding with deserialization...");
            let data = WhirlpoolsConfigAccount::deserialize(buf)
                .map_err(|e| AccountDecodeError {
                    message: format!("Failed to deserialize WhirlpoolsConfigAccount: {}", e),
                })?;
            println!("\nDecoded Whirlpool Config Account Structure: {:#?}", data);
            Ok(DecodedAccount::WhirlpoolsConfig(data.0)) // Unwrapping the inner BondingCurve struct
        }
        WHIRLPOOLS_CONFIG_EXTENSION_ACCOUNT_DISCM => {
            let data = WhirlpoolsConfigExtensionAccount::deserialize(buf)
                .map_err(|e| AccountDecodeError {
                    message: format!("Failed to deserialize WhirlpoolsConfigExtAccount: {}", e),
                })?;

            println!("\nDecoded Whirlpool Config Ext Account Structure: {:#?}", data);
            Ok(DecodedAccount::WhirlpoolsConfigExtension(data.0)) // Unwrapping the inner BondingCurve struct
        }
        FEE_TIER_ACCOUNT_DISCM => {
            let data = FeeTierAccount::deserialize(buf)
                .map_err(|e| AccountDecodeError {
                    message: format!("Failed to deserialize FreeTier Account: {}", e),
                })?;

            println!("\nDecoded Fee Tier Account Structure: {:#?}", data);
            Ok(DecodedAccount::FeeTier(data.0)) // Unwrapping the inner BondingCurve struct
        }
        POSITION_BUNDLE_ACCOUNT_DISCM => {
            let data = PositionBundleAccount::deserialize(buf)
                .map_err(|e| AccountDecodeError {
                    message: format!("Failed to deserialize PositionBundleAccount: {}", e),
                })?;
            println!("\nDecoded Position Bundle Account Structure: {:#?}", data);
            Ok(DecodedAccount::PositionBundle(data.0)) // Unwrapping the inner BondingCurve struct
        }
        POSITION_ACCOUNT_DISCM => {
            let data = PositionAccount::deserialize(buf)
                .map_err(|e| AccountDecodeError {
                    message: format!("Failed to deserialize PositionAccount: {}", e),
                })?;
            println!("\nDecoded Position Account Structure: {:#?}", data);
            Ok(DecodedAccount::Position(data.0)) // Unwrapping the inner BondingCurve struct
        }
        TICK_ARRAY_ACCOUNT_DISCM => {
            let data = TickArrayAccount::deserialize(buf)
                .map_err(|e| AccountDecodeError {
                    message: format!("Failed to deserialize TickArrayAccount: {}", e),
                })?;
            println!("\nDecoded Tick Array Account Structure: {:#?}", data);
            Ok(DecodedAccount::TickArray(data.0)) // Unwrapping the inner BondingCurve struct
        }
        TOKEN_BADGE_ACCOUNT_DISCM => {
            let data = TokenBadgeAccount::deserialize(buf)
                .map_err(|e| AccountDecodeError {
                    message: format!("Failed to deserialize TokenBadgeAccount: {}", e),
                })?;
            println!("\nDecoded Token Badge Account Structure: {:#?}", data);
            Ok(DecodedAccount::TokenBadge(data.0)) // Unwrapping the inner BondingCurve struct
        }
        WHIRLPOOL_ACCOUNT_DISCM => {
            let data = WhirlpoolAccount::deserialize(buf)
                .map_err(|e| AccountDecodeError {
                    message: format!("Failed to deserialize WhirlpoolAccount: {}", e),
                })?;
            println!("\nDecoded Whirlpool Account Structure: {:#?}", data);
            Ok(DecodedAccount::Whirlpool(data.0)) // Unwrapping the inner BondingCurve struct
        }
        _ => Err(AccountDecodeError {
            message: "Account discriminator not found.".to_string(),
        }),
    }
}
