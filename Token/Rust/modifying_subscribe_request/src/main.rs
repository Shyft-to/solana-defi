use {
    anyhow::Context,
    backoff::{future::retry, ExponentialBackoff},
    clap::Parser as ClapParser,
    futures::{future::TryFutureExt, sink::SinkExt, stream::StreamExt},
    log::{error, info},
    serde_json::json,
    serde::{Serialize, Deserialize},
    serde_json::Value,
    tokio::time::{sleep,Duration},
    std::{
      collections::HashMap, env, fs, str::FromStr, sync::Arc, time::{ SystemTime, UNIX_EPOCH}
    },
    tokio::io::{AsyncReadExt, AsyncWriteExt}, // These traits are needed for async I/O
    //std::net::{TcpListener, TcpStream},
    std::io::{Read, Write},
    std::env::var,
        tokio::task,
    solana_transaction_status::{UiTransactionEncoding,
      ConfirmedTransactionWithStatusMeta, InnerInstruction, InnerInstructions, Reward, RewardType, TransactionStatusMeta, TransactionTokenBalance, TransactionWithStatusMeta, VersionedTransactionWithStatusMeta
    },
    solana_account_decoder_client_types::token::UiTokenAmount,
    solana_sdk::{
      hash::Hash, instruction::{AccountMeta, CompiledInstruction, Instruction}, message::{v0::{LoadedAddresses, Message, MessageAddressTableLookup}, MessageHeader, VersionedMessage}, pubkey::Pubkey, signature::Signature, transaction::VersionedTransaction, transaction_context::TransactionReturnData
    }, 
    tokio::sync::Mutex,
    tokio::net::TcpListener,
    tonic::transport::channel::ClientTlsConfig,
    yellowstone_grpc_client::{GeyserGrpcClient, Interceptor},
    yellowstone_grpc_proto::prelude::{
        subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest, SubscribeRequestPing,UnixTimestamp,
        SubscribeRequestFilterTransactions,SubscribeUpdateTransactionInfo,SubscribeRequestFilterAccounts
    },
    yellowstone_grpc_proto::convert_from
  };
  use spl_token::instruction::TokenInstruction;

#[derive(Debug,Serialize)]
  #[derive(Clone)]
  struct TransactionInstructionWithParent {
      instruction: Instruction,
      parent_program_id: Option<Pubkey>,
  }
type AccountFilterMap = HashMap<String, SubscribeRequestFilterAccounts>;
  
type TransactionsFilterMap = HashMap<String, SubscribeRequestFilterTransactions>;
const PUMP_PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

#[derive(Debug, Clone, ClapParser)]
#[clap(author, version, about)]
struct Args {
    #[clap(short, long, help = "gRPC endpoint")]
    /// Service endpoint
    endpoint: String,

    #[clap(long, help = "X-Token")]
    x_token: String
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

    
  pub fn get_account_updates(&self) -> anyhow::Result<SubscribeRequest> {
        let mut accounts: AccountFilterMap = HashMap::new();

        accounts.insert(
            "modifying_B".to_owned(),
            SubscribeRequestFilterAccounts {
                account: vec![],
                owner: vec![PUMP_PROGRAM_ID.to_string()],
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
    pub fn get_txn_updates(&self) -> anyhow::Result<SubscribeRequest> {
        let mut transactions: TransactionsFilterMap = HashMap::new();

        transactions.insert(
            "Modifying_A".to_owned(),
            SubscribeRequestFilterTransactions {
                vote: Some(false),
                failed: Some(false),
                account_include: vec![PUMP_PROGRAM_ID.to_string()],
                account_exclude: vec![],
                account_required: vec![],
                signature: None,
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
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env::set_var(
        env_logger::DEFAULT_FILTER_ENV,
        env::var_os(env_logger::DEFAULT_FILTER_ENV).unwrap_or_else(|| "info".into()),
    );
    env_logger::init();

    let args = Args::parse();
    let zero_attempts = Arc::new(Mutex::new(true));

    let client = Arc::new(Mutex::new(args.connect().await?)); 
    retry(ExponentialBackoff::default(), move || {
        let args = args.clone();
        let zero_attempts = Arc::clone(&zero_attempts);
        let client = Arc::clone(&client); // Share client

        async move {
            let mut zero_attempts = zero_attempts.lock().await;
            if *zero_attempts {
                *zero_attempts = false;
            } else {
                info!("Retry to connect to the server");
            }
            drop(zero_attempts);
          

            let account_request = args.get_account_updates().unwrap();
            let txn_request = args.get_txn_updates().unwrap();

            let mut client = client.lock().await;
            info!("Subscribing to transaction updates...");
            geyser_subscribe(&mut client, account_request, txn_request).await.map_err(backoff::Error::transient)?;
            
            Ok::<(), backoff::Error<anyhow::Error>>(())
        
    }
        .inspect_err(|error| error!("failed to connect: {error}"))
    })
    .await
    .map_err(Into::into)
}
async fn geyser_subscribe(
    client: &mut GeyserGrpcClient<impl Interceptor>, // Client to subscribe
    account_request: SubscribeRequest, // Initial account request
    txn_request: SubscribeRequest, // Transaction request
) -> anyhow::Result<()> {
    let (mut subscribe_tx, mut stream) = client.subscribe_with_request(Some(txn_request)).await?;
    
    info!("Started streaming transaction updates...");

    // Track the time since the start
    let start_time = tokio::time::Instant::now();
    let mut switched_to_account = false;

    // Handle incoming stream messages
    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => {
                info!("Received message: {:?}", msg);

                match msg.update_oneof {
                    Some(UpdateOneof::Transaction(msg)) => {
                        info!("Received Transaction update: {:?}", msg);
                    }
                    Some(UpdateOneof::Ping(_)) => {
                        // Respond to ping
                        subscribe_tx.send(SubscribeRequest {
                            ping: Some(SubscribeRequestPing { id: 1 }),
                            ..Default::default()
                        }).await?;
                    }
                    Some(UpdateOneof::Pong(_)) => {
                        // Handle pong response
                    }
                    None => {
                        error!("No update found in the message.");
                        break;
                    }
                    _ => {
                        info!("Received other update: {:?}", msg);
                    }
                }
            }
            Err(e) => {
                error!("Error receiving message: {:?}", e);
                break;
            }
        }

        // Switch to the account request after a certain time period (e.g., 10 seconds)
        if !switched_to_account && start_time.elapsed() >= Duration::from_secs(10) {
            info!("Switching from txn_request to account_request after 10 seconds...");
            subscribe_tx.send(account_request.clone()).await?; 
            switched_to_account = true; 
        }
    }

    info!("Stream closed after updates.");
    Ok(())
}

