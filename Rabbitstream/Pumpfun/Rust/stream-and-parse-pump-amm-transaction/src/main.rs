use {
    backoff::{future::retry, ExponentialBackoff},
    clap::Parser as ClapParser,
    futures::{
        future::TryFutureExt,
        sink::SinkExt,         
        stream::StreamExt,
    },
    log::{error, info},
    solana_sdk::{
        hash::Hash,
        message::{v0::LoadedAddresses, MessageHeader},
        pubkey::Pubkey,
        signature::Signature,
        transaction::{Result as TransactionResult}, 
        message::v0::MessageAddressTableLookup,
        transaction_context::TransactionReturnData,
    },
    solana_transaction_status::{TransactionTokenBalance, Rewards},
    std::{collections::HashMap, env, sync::Arc, time::Duration},
    tokio::sync::Mutex,
    tonic::transport::channel::ClientTlsConfig,
    yellowstone_grpc_client::{GeyserGrpcClient, Interceptor},
    yellowstone_grpc_proto::{
        geyser::SubscribeRequestFilterTransactions,
        prelude::{subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest, SubscribeRequestPing},
    },
};


mod processor;
use processor::TransactionProcessor;
use processor::types::DecodedInstruction;

type TxnFilterMap = HashMap<String, SubscribeRequestFilterTransactions>;

const PUMPFUN_AMM_PROGRAM_ID: &str = "pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA";
const TOKEN_PROGRAM_ID: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";


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
        let mut transactions: TxnFilterMap = HashMap::new();

        transactions.insert(
            "client".to_owned(),
            SubscribeRequestFilterTransactions {
                vote: Some(false),
                failed: Some(false),
                account_include: vec![PUMPFUN_AMM_PROGRAM_ID.to_string()],
                account_exclude: vec![],
                account_required: vec![],
                signature: None,
            },
        );

        Ok(SubscribeRequest {
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
        })
    }
}



#[derive(Clone, Debug)]
struct ParsedEventTransaction {
    parsed_transaction: ParsedConfirmedTransactionWithStatusMeta,
}

#[derive(Clone, Debug)]
pub struct ParsedConfirmedTransactionWithStatusMeta {
    pub slot: u64,
    pub meta: ParsedTransactionStatusMeta,
    pub transaction: ParsedTransaction,
    pub block_time: Option<i64>,
}

#[derive(Clone, Debug)]
pub struct ParsedTransaction {
    pub signatures: Vec<Signature>,
    pub message: ParsedMessage,
}

#[derive(Clone, Debug)]
pub struct ParsedMessage {
    pub header: MessageHeader,
    pub account_keys: Vec<Pubkey>,
    pub recent_blockhash: Hash,
    pub instructions: Vec<DecodedInstruction>,
    pub address_table_lookups: Vec<MessageAddressTableLookup>,
}

#[derive(Clone, Debug)]
pub struct ParsedTransactionStatusMeta {
    pub status: TransactionResult<()>,
    pub fee: u64,
    pub pre_balances: Vec<u64>,
    pub post_balances: Vec<u64>,
    pub inner_instructions: Vec<DecodedInstruction>, 
    pub log_messages: Option<Vec<String>>,
    pub pre_token_balances: Option<Vec<TransactionTokenBalance>>,
    pub post_token_balances: Option<Vec<TransactionTokenBalance>>,
    pub rewards: Option<Rewards>,
    pub loaded_addresses: LoadedAddresses,
    pub return_data: Option<TransactionReturnData>,
    pub compute_units_consumed: Option<u64>,
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

    let processor = TransactionProcessor::new()?;

    while let Some(message) = stream.next().await {
        match message {
            Ok(msg) => match msg.update_oneof {
               Some(UpdateOneof::Transaction(update)) => {
                 match processor.process_transaction_update(update) {
                  Ok(Some(pumpfun_txn)) => println!("Pump AMM:\n{:#?}", pumpfun_txn),
                  Ok(None) => (),
                  Err(e) => (),
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