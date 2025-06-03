use {
    backoff::{future::retry, ExponentialBackoff}, clap::Parser as ClapParser, futures::{
        future::TryFutureExt,
        sink::SinkExt,
        stream::StreamExt,
    }, log::{error, info}, raydium_launchpad_interface::{accounts::{
            GLOBAL_CONFIG_ACCOUNT_DISCM,
            GlobalConfig,
            GlobalConfigAccount,
            PLATFORM_CONFIG_ACCOUNT_DISCM,
            PlatformConfig,
            PlatformConfigAccount,
            POOL_STATE_ACCOUNT_DISCM,              
            PoolState,
            PoolStateAccount,
            VESTING_RECORD_ACCOUNT_DISCM,
            VestingRecord,
            VestingRecordAccount
    }},serde::{Deserialize, Serialize}, std::{
        collections::HashMap, env, sync::Arc, time::Duration
    }, tokio::sync::Mutex, tonic::transport::channel::ClientTlsConfig, yellowstone_grpc_client::{GeyserGrpcClient, Interceptor}, yellowstone_grpc_proto::{
        geyser::SubscribeRequestFilterAccounts,
        prelude::{
            subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest, SubscribeRequestPing,
        },
    }
};
use borsh::{BorshDeserialize, BorshSerialize};

type AccountFilterMap = HashMap<String, SubscribeRequestFilterAccounts>;

const RAYDIUM_CP_PROGRAM_ID: &str = "LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj";

#[derive(Debug, Clone,Serialize)]
pub enum DecodedAccount {
    GlobalConfig(GlobalConfig),
    VestingRecord(VestingRecord),
    PlatformConfig(PlatformConfig),
    PoolState(PoolState),
}
#[allow(dead_code)]
#[derive(Debug, Clone)]
struct ParsedData  {
    parsed: DecodedAccount,
}
#[allow(dead_code)]
#[derive(Debug,Clone)]
struct Data  {
   parsed : ParsedData,
   executable: bool,
   lamports : u64,
   owner : String,
   rent_epoch : u64,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
struct AccountValue {
    data: Data,
}

pub trait AccountData: std::fmt::Debug {}

#[derive(Debug,Default)]
pub struct EmptyAccount;

impl AccountData for EmptyAccount {}



#[derive(Debug)]
pub struct AccountDecodeError {
    pub message: String,
}


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
            .connect_timeout(Duration::from_secs(10))
            .timeout(Duration::from_secs(10))
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
                owner: vec![RAYDIUM_CP_PROGRAM_ID.to_string()],
                nonempty_txn_signature: None,
                filters: vec![]
            },
        );

        Ok(SubscribeRequest {
            accounts,
            slots: HashMap::default(),
            transactions:HashMap::default(),
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
                                return Ok(());// Handle the error as needed
                            }
                        };
                        let account_json = AccountValue {
                            data : Data {
                                parsed: ParsedData {
                                     parsed: decoded_account
                                },
                              executable : executable,
                              lamports : lamports,
                              owner : owner,
                              rent_epoch : rent  
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

    match discriminator {
        GLOBAL_CONFIG_ACCOUNT_DISCM => {
            let data = GlobalConfigAccount::deserialize(buf)
                .map_err(|e| AccountDecodeError {
                    message: format!("Failed to deserialize GlobalConfig: {}", e),
                })?;
            Ok(DecodedAccount::GlobalConfig(data.0))
        }
        PLATFORM_CONFIG_ACCOUNT_DISCM => {
            let data = PlatformConfigAccount::deserialize(buf)
                .map_err(|e| AccountDecodeError {
                    message: format!("Failed to deserialize PlatformConfig: {}", e),
                })?;
            Ok(DecodedAccount::PlatformConfig(data.0))
        }
         POOL_STATE_ACCOUNT_DISCM => {
            let data = PoolStateAccount::deserialize(buf)
                .map_err(|e| AccountDecodeError {
                    message: format!("Failed to deserialize PoolState: {}", e),
                })?;
            Ok(DecodedAccount::PoolState(data.0))
        }
         VESTING_RECORD_ACCOUNT_DISCM => {
            let data = VestingRecordAccount::deserialize(buf)
                .map_err(|e| AccountDecodeError {
                    message: format!("Failed to deserialize VestingRecord: {}", e),
                })?;
            Ok(DecodedAccount::VestingRecord(data.0))
        }
        _ => Err(AccountDecodeError {
            message: "Account discriminator not found.".to_string(),
        }),
    }
}

