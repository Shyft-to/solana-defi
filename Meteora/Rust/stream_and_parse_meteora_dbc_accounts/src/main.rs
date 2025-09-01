use {
    backoff::{future::retry, ExponentialBackoff},
    clap::Parser as ClapParser,
    futures::{
        future::TryFutureExt,
        sink::SinkExt,
        stream::StreamExt,
    },
    log::{error, info},
    serde::Serialize,
    std::{
        collections::HashMap, env, sync::Arc, time::Duration
    },
    tokio::sync::Mutex,
    tonic::transport::channel::ClientTlsConfig,
    yellowstone_grpc_client::{GeyserGrpcClient, Interceptor},
    yellowstone_grpc_proto::{
        geyser::SubscribeRequestFilterAccounts,
        prelude::{
            subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest, SubscribeRequestPing,
        },
    }
};

use meteora_dbc_interface::accounts::{
    ClaimFeeOperator,
    Config,
    LockEscrow,
    MeteoraDammMigrationMetadata,
    MeteoraDammV2Metadata,
    PartnerMetadata,
    PoolConfig,
    VirtualPool,
    VirtualPoolMetadata,
    ClaimFeeOperatorAccount,
    ConfigAccount,
    LockEscrowAccount,
    MeteoraDammMigrationMetadataAccount,
    MeteoraDammV2MetadataAccount,
    PartnerMetadataAccount,
    PoolConfigAccount,
    VirtualPoolAccount,
    VirtualPoolMetadataAccount,
    CLAIM_FEE_OPERATOR_ACCOUNT_DISCM,
    CONFIG_ACCOUNT_DISCM,
    LOCK_ESCROW_ACCOUNT_DISCM,
    METEORA_DAMM_MIGRATION_METADATA_ACCOUNT_DISCM,
    METEORA_DAMM_V2_METADATA_ACCOUNT_DISCM,
    PARTNER_METADATA_ACCOUNT_DISCM,
    POOL_CONFIG_ACCOUNT_DISCM,
    VIRTUAL_POOL_ACCOUNT_DISCM,
    VIRTUAL_POOL_METADATA_ACCOUNT_DISCM,
};

type AccountFilterMap = HashMap<String, SubscribeRequestFilterAccounts>;

const METEORA_DBC_PROGRAM_ID: &str = "dbcij3LWUppWqq96dh6gJWwBifmcGfLSB5D4DuSMaqN";

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
                owner: vec![METEORA_DBC_PROGRAM_ID.to_string()],
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

#[allow(dead_code)]
#[derive(Debug, Clone, Serialize)]
pub enum DecodedAccount {
    ClaimFeeOperator(ClaimFeeOperator),
    Config(Config),
    LockEscrow(LockEscrow),
    MeteoraDammMigrationMetadata(MeteoraDammMigrationMetadata),
    MeteoraDammV2Metadata(MeteoraDammV2Metadata),
    PartnerMetadata(PartnerMetadata),
    PoolConfig(PoolConfig),
    VirtualPool(VirtualPool),
    VirtualPoolMetadata(VirtualPoolMetadata),
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ParsedData {
    parsed_data: DecodedAccount,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Data {
    account: ParsedData,
    executable: bool,
    lamports: u64,
    owner: String,
    rent_epoch: u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct Value {
    data: Data,
}

pub trait AccountData: std::fmt::Debug {}

#[derive(Debug, Default)]
pub struct EmptyAccount;

impl AccountData for EmptyAccount {}

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
                    if let Some(account_data) = account.account {
                        let pubkey_str = bs58::encode(&account_data.pubkey).into_string();
                        let owner = bs58::encode(&account_data.owner).into_string();
                        let lamports = account_data.lamports;
                        let executable = account_data.executable;
                        let rent = account_data.rent_epoch;

                        let decoded_account = match decode_account_data(&account_data.data) {
                            Ok(data) => data,
                            Err(e) => {
                                eprintln!("Failed to decode account data: {}", e.message);
                                return Ok(());
                            }
                        };
                        let account_json = Value {
                            data: Data {
                                account: ParsedData {
                                    parsed_data: decoded_account
                                },
                                executable: executable,
                                lamports: lamports,
                                owner: owner,
                                rent_epoch: rent  
                            }
                        };

                        println!("\nAccount Info: {:#?}", account_json);
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

    if discriminator == CLAIM_FEE_OPERATOR_ACCOUNT_DISCM {
        let data = ClaimFeeOperatorAccount::deserialize(buf).map_err(|e| AccountDecodeError {
            message: format!("Failed to deserialize ClaimFeeOperator: {}", e),
        })?;
        Ok(DecodedAccount::ClaimFeeOperator(data.0))
    } else if discriminator == CONFIG_ACCOUNT_DISCM {
        let data = ConfigAccount::deserialize(buf).map_err(|e| AccountDecodeError {
            message: format!("Failed to deserialize Config: {}", e),
        })?;
        Ok(DecodedAccount::Config(data.0))
    } else if discriminator == LOCK_ESCROW_ACCOUNT_DISCM {
        let data = LockEscrowAccount::deserialize(buf).map_err(|e| AccountDecodeError {
            message: format!("Failed to deserialize LockEscrow: {}", e),
        })?;
        Ok(DecodedAccount::LockEscrow(data.0))
    } else if discriminator == METEORA_DAMM_MIGRATION_METADATA_ACCOUNT_DISCM {
        let data = MeteoraDammMigrationMetadataAccount::deserialize(buf).map_err(|e| AccountDecodeError {
            message: format!("Failed to deserialize MeteoraDammMigrationMetadata: {}", e),
        })?;
        Ok(DecodedAccount::MeteoraDammMigrationMetadata(data.0))
    } else if discriminator == METEORA_DAMM_V2_METADATA_ACCOUNT_DISCM {
        let data = MeteoraDammV2MetadataAccount::deserialize(buf).map_err(|e| AccountDecodeError {
            message: format!("Failed to deserialize MeteoraDammV2Metadata: {}", e),
        })?;
        Ok(DecodedAccount::MeteoraDammV2Metadata(data.0))
    } else if discriminator == PARTNER_METADATA_ACCOUNT_DISCM {
        let data = PartnerMetadataAccount::deserialize(buf).map_err(|e| AccountDecodeError {
            message: format!("Failed to deserialize PartnerMetadata: {}", e),
        })?;
        Ok(DecodedAccount::PartnerMetadata(data.0))
    } else if discriminator == POOL_CONFIG_ACCOUNT_DISCM {
        let data = PoolConfigAccount::deserialize(buf).map_err(|e| AccountDecodeError {
            message: format!("Failed to deserialize PoolConfig: {}", e),
        })?;
        Ok(DecodedAccount::PoolConfig(data.0))
    } else if discriminator == VIRTUAL_POOL_ACCOUNT_DISCM {
        let data = VirtualPoolAccount::deserialize(buf).map_err(|e| AccountDecodeError {
            message: format!("Failed to deserialize VirtualPool: {}", e),
        })?;
        Ok(DecodedAccount::VirtualPool(data.0))
    } else if discriminator == VIRTUAL_POOL_METADATA_ACCOUNT_DISCM {
        let data = VirtualPoolMetadataAccount::deserialize(buf).map_err(|e| AccountDecodeError {
            message: format!("Failed to deserialize VirtualPoolMetadata: {}", e),
        })?;
        Ok(DecodedAccount::VirtualPoolMetadata(data.0))
    } else {
        Err(AccountDecodeError {
            message: "Account discriminator not found.".to_string(),
        })
    }
}