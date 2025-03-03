mod serialization;
mod instruction_account_mapper;
// mod token_serializable;

use {
    backoff::{future::retry, ExponentialBackoff}, clap::Parser as ClapParser, futures::{
        future::TryFutureExt,
        sink::SinkExt,
        stream::StreamExt,
    }, instruction_account_mapper::AccountMetadata, log::{error, info}, pump_interface::accounts::BondingCurveAccount, serde::{Serialize, Deserialize}, serialization::{serialize_option_pubkey, serialize_pubkey}, solana_sdk::{ 
              pubkey::Pubkey, signature::Signature
    }, std::{
        collections::HashMap, env, str::FromStr, sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}
    }, tokio::sync::Mutex, tonic::transport::channel::ClientTlsConfig, yellowstone_grpc_client::{GeyserGrpcClient, Interceptor}, yellowstone_grpc_proto::prelude::{
            subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest, SubscribeRequestPing,SubscribeRequestFilterAccounts,SubscribeRequestFilterAccountsFilter,SubscribeRequestFilterAccountsFilterMemcmp, subscribe_request_filter_accounts_filter::Filter, subscribe_request_filter_accounts_filter_memcmp::Data
    },
    
};

use solana_client::{rpc_client::RpcClient, rpc_config::RpcTransactionConfig};
use tokio::task;
use solana_transaction_status::option_serializer::OptionSerializer;




type AccountFilterMap = HashMap<String, SubscribeRequestFilterAccounts>;

const PUMP_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

#[derive(Debug, Clone, ClapParser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, help = "gRPC endpoint")]
    endpoint: String,

    #[clap(long, help = "X-Token")]
    x_token: String,

    #[clap(long, help = "Solana RPC URL")]
    rpc_url: String,
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
        let complete_field_offset: u64 = 48; // This has been calculated by seeing the account structure as per the IDL, 6 u64 bytes(8bytes * 6) before this.

        accounts.insert(
            "migration".to_owned(),
            SubscribeRequestFilterAccounts {
                account: vec![],
                owner: vec![PUMP_PROGRAM_ID.to_string()],
                nonempty_txn_signature: None,
                filters: vec![
                    SubscribeRequestFilterAccountsFilter {
                        filter: Some(Filter::Memcmp(SubscribeRequestFilterAccountsFilterMemcmp {
                            offset: complete_field_offset,
                            data: Some(Data::Bytes(vec![1])), // Correctly wrapped in Data::Bytes
                        })),
                    },
                ],
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


#[derive(Debug, Serialize)]
pub struct DecodedInstruction {
    pub name: String,
    pub accounts: Vec<AccountMetadata>,
    pub data: serde_json::Value,
    #[serde(serialize_with = "serialize_pubkey")]
    pub program_id: Pubkey,
    #[serde(serialize_with = "serialize_option_pubkey")]
    pub parent_program_id: Option<Pubkey>,
}



#[derive(Debug, Serialize, Deserialize)]
pub struct Structure {
    pub discriminator: u64,
    pub virtual_token_reserves: u64,
    pub virtual_sol_reserves: u64,
    pub real_token_reserves: u64,
    pub real_sol_reserves: u64,
    pub token_total_supply: u64,
    pub complete: bool,
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

            geyser_subscribe(client, request,args.rpc_url)
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
    rpc_url: String,
) -> anyhow::Result<()> {
    
    let (mut subscribe_tx, mut stream) = client.subscribe_with_request(Some(request)).await?;

    info!("stream opened");

    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => match msg.update_oneof {
                
                Some(UpdateOneof::Account(account)) => {
                    //println!("Account update: \n{:?}\n", account);
                    let slot = account.slot;
                    let account_update = account.account;

                    let current_timestamp = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as u64;
                    
                    
                    if let Some(account_data) = account_update {
                        let pubkey_str = bs58::encode(&account_data.pubkey).into_string();
                        let owner = bs58::encode(&account_data.owner).into_string();

                        println!("Account Update Received for: {}, Owner: {} at slot: {}", pubkey_str, owner, slot);


                        match Pubkey::from_str(&pubkey_str) {
                            Ok(pubkey) => {
                                let rpc_url_clone = rpc_url.clone(); 
                                // Spawn the function in a separate task to execute concurrently
                                task::spawn(async move {
                                    get_tokens_in_wallet(pubkey, current_timestamp, rpc_url_clone).await;
                                });
                            }
                            Err(e) => {
                                println!("Failed to parse pubkey: {}", e);
                            }
                        }

                        if let Some(decoded_structure) = decode_account_data(&account_data.data) {
                            println!("Decoded Structure for account {}: {:#?}", pubkey_str, decoded_structure);
                        } else {
                            println!("Failed to decode structure for account {}", pubkey_str);
                        }
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

fn decode_account_data(data: &[u8]) -> Option<Structure> {
    if data.len() < 49 {
        // 8 bytes * 6 (for u64) + 1 byte (for bool) = 49 bytes minimum
        println!("Data length too short to decode.");
        return None;
    }

    //println!("Decode with Interface: {:?}",BondingCurveAccount::deserialize(data));

    // Use slices to read chunks of bytes
    let discriminator = u64::from_le_bytes(data[0..8].try_into().unwrap());
    let virtual_token_reserves = u64::from_le_bytes(data[8..16].try_into().unwrap());
    let virtual_sol_reserves = u64::from_le_bytes(data[16..24].try_into().unwrap());
    let real_token_reserves = u64::from_le_bytes(data[24..32].try_into().unwrap());
    let real_sol_reserves = u64::from_le_bytes(data[32..40].try_into().unwrap());
    let token_total_supply = u64::from_le_bytes(data[40..48].try_into().unwrap());
    let complete = data[48] != 0;  // 1 byte for bool

    Some(Structure {
        discriminator,
        virtual_token_reserves,
        virtual_sol_reserves,
        real_token_reserves,
        real_sol_reserves,
        token_total_supply,
        complete,
    })
}


async fn get_tokens_in_wallet(address: Pubkey, current_timestamp: u64, rpc_url: String) {
    //let rpc_url = "https://rpc.shyft.to?api_key=";
    let client = RpcClient::new(rpc_url.to_string());

    let mut time_diff: u64 = 0;

    match client.get_signatures_for_address(&address) {
        Ok(signatures) => {
            if signatures.is_empty() {
                //println!("No signatures found, migrations at least 3 days old");
                return;
            }

            let mut required_signature: Option<String> = None; // Initialize as None

            for signature_info in &signatures {
                //println!("Signature: {:?}", signature_info);

                if signature_info.err.is_none() {
                    let block_time = signature_info.block_time.unwrap_or(0) as u64;
                    time_diff = current_timestamp.saturating_sub(block_time * 1000);

                    if time_diff > 10 * 60 * 60 * 1000 {
                        return;
                    } else {
                        required_signature = Some(signature_info.signature.clone()); // Store in Some
                        break;
                    }
                }
            }

            if let Some(sig_str) = required_signature {
                let formatted_required_signature = Signature::from_str(&sig_str)
                    .expect("Failed to parse signature");

                let config = RpcTransactionConfig {
                    commitment: None,
                    encoding: Some(solana_transaction_status::UiTransactionEncoding::Json),
                    max_supported_transaction_version: Some(0),
                    ..RpcTransactionConfig::default()
                };
            
                // Now use formatted_required_signature
                match client.get_transaction_with_config(&formatted_required_signature, config) {
                    Ok(transaction) => {
                        // println!("Transaction details: {:#?}", transaction);
                        // Extract the `meta` field correctly
                        if let Some(meta) = &transaction.transaction.meta {
                            if let OptionSerializer::Some(post_balances) = &meta.post_token_balances {
                                for balance in post_balances.iter() {
                                    if balance.ui_token_amount.amount == "0" {
                                        println!("Mint being transferred: {}", balance.mint);
                                    }
                                }
                            }
                        }
                    },
                    Err(e) => println!("Failed to fetch transaction: {}", e),
                }
            } else {
                println!("No valid signature found.");
            }
        }
        Err(e) => println!("Error fetching signatures: {}", e),
    }
}




