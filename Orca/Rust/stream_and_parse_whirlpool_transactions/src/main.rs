mod serialization;
mod instruction_account_mapper;
mod token_serializable;

use {
    backoff::{future::retry, ExponentialBackoff}, clap::Parser as ClapParser, futures::{
        future::TryFutureExt,
        sink::SinkExt,
        stream::StreamExt,
    }, 
    serialization::{serialize_pubkey, serialize_option_pubkey},
    instruction_account_mapper::{AccountMetadata, Idl, InstructionAccountMapper}, log::{error, info}, serde::Serialize, solana_account_decoder_client_types::token::UiTokenAmount, solana_sdk::{
        hash::Hash, instruction::{AccountMeta, CompiledInstruction, Instruction}, message::{v0::{LoadedAddresses, Message, MessageAddressTableLookup}, MessageHeader, VersionedMessage}, pubkey::Pubkey, signature::Signature,transaction::{VersionedTransaction,TransactionVersion, Legacy}, transaction_context::TransactionReturnData
    }, solana_transaction_status::{
     ConfirmedTransactionWithStatusMeta, InnerInstruction, InnerInstructions, Reward, RewardType, TransactionStatusMeta, TransactionTokenBalance, TransactionWithStatusMeta, VersionedTransactionWithStatusMeta
    }, std::{
        collections::HashMap, env, fs, str::FromStr, sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}
    }, tokio::sync::Mutex, tonic::transport::channel::ClientTlsConfig, whirlpool_interface::instructions::WhirlpoolProgramIx, yellowstone_grpc_client::{GeyserGrpcClient, Interceptor}, yellowstone_grpc_proto::{
        geyser::SubscribeRequestFilterTransactions,
        prelude::{
            subscribe_update::UpdateOneof, CommitmentLevel, SubscribeRequest, SubscribeRequestPing,
        },
    },
    spl_token::instruction::TokenInstruction
};
use crate::token_serializable::convert_to_serializable;

type TxnFilterMap = HashMap<String, SubscribeRequestFilterTransactions>;

const WHIRLPOOL_PROGRAM_ID: &str = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";
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
                account_include: vec![WHIRLPOOL_PROGRAM_ID.to_string()],
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

#[derive(Debug,Serialize)]
struct TransactionInstructionWithParent {
    instruction: Instruction,
    parent_program_id: Option<Pubkey>,
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

#[derive(Debug)]
pub struct TransactionWithActions {
    pub slot: u64,
    pub tx_with_meta: TransactionWithStatusMeta,
    pub block_time: i64,
    pub actions: Vec<DecodedInstruction>,
    pub transactions: Vec<TransactionInstructionWithParent>,
    pub version: Option<TransactionVersion>
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

/// Converts a string to camel case.
fn to_camel_case(name: &str) -> String {
    let mut chars = name.chars();
    match chars.next() {
        Some(first_char) => first_char.to_lowercase().collect::<String>() + chars.as_str(),
        None => String::new(),
    }
}

/// Extracts the instruction name and converts it to camel case.
fn get_instruction_name_with_typename(instruction: &TokenInstruction) -> String {
    let debug_string = format!("{:?}", instruction);
    if let Some(first_brace) = debug_string.find(" {") {
        let name = &debug_string[..first_brace]; // Extract name before `{`
        to_camel_case(name)
    } else {
        to_camel_case(&debug_string) // Directly convert unit variant names
    }
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
                Some(UpdateOneof::Transaction(update)) => {
                    let slot = update.slot;
                    let block_time = SystemTime::now()
                        .duration_since(UNIX_EPOCH)
                        .expect("Time went backwards")
                        .as_secs() as i64;
                    let update: Option<
                        yellowstone_grpc_proto::prelude::SubscribeUpdateTransactionInfo,
                    > = update.transaction;
                    if let Some(txn) = update {
                        let raw_signature = txn.signature.clone();
                        info!("signature: {}", bs58::encode(&raw_signature).into_string());
                        let raw_transaction = txn.transaction.expect("transaction empty");
                        let raw_message = raw_transaction.message.expect("message empty").clone();
                        let header = raw_message.header.expect("header empty");
                        let meta = txn.meta.expect("Meta empty");

                        if raw_signature.len() != 64 {
                            panic!("Signature must be exactly 64 bytes");
                        }

                        let raw_signature_array: [u8; 64] = raw_signature.try_into().expect("Failed to convert to [u8; 64]");
                        let signature = Signature::from(raw_signature_array);
                        let recent_blockhash=  Hash::new_from_array(raw_message
                            .recent_blockhash
                            .clone()
                            .try_into()
                            .expect("Failed to convert Vec<u8> to [u8; 32]"));
                        let confirmed_txn_with_meta = ConfirmedTransactionWithStatusMeta {
                            slot,
                            tx_with_meta: TransactionWithStatusMeta::Complete(
                                VersionedTransactionWithStatusMeta {
                                    transaction: VersionedTransaction {
                                        signatures: vec![signature],
                                        message: VersionedMessage::V0(
                                            Message {
                                                header: MessageHeader {
                                                    num_required_signatures: header.num_required_signatures as u8,
                                                    num_readonly_signed_accounts: header.num_readonly_signed_accounts as u8,
                                                    num_readonly_unsigned_accounts: header.num_readonly_unsigned_accounts as u8,
                                                },
                                                account_keys: raw_message
                                                    .account_keys
                                                    .iter()
                                                    .map(|k: &Vec<u8>| {
                                                        k.clone()
                                                            .try_into()
                                                            .expect("Failed to convert Vec<u8> to [u8; 32]")
                                                    })
                                                    .collect(),
                                                recent_blockhash,
                                                instructions: raw_message
                                                    .instructions
                                                    .iter()
                                                    .map(|ix| CompiledInstruction {
                                                        program_id_index: ix.program_id_index as u8,
                                                        accounts: ix.accounts.clone(),
                                                        data: ix.data.clone(),
                                                    })
                                                    .collect(),
                                                address_table_lookups:
                                                    raw_message
                                                        .address_table_lookups
                                                        .iter()
                                                        .map(|l| MessageAddressTableLookup {
                                                            account_key: Pubkey::new_from_array(l.account_key.clone().try_into().expect("Failed to convert Vec<u8> to [u8; 32]")),
                                                            writable_indexes: l.writable_indexes.clone(),
                                                            readonly_indexes: l.readonly_indexes.clone(),
                                                        })
                                                        .collect(),
                                            }
                                        ),
                                    },
                                    meta: TransactionStatusMeta {
                                        status: Result::Ok(()),
                                        fee: meta.fee,
                                        pre_balances: meta.pre_balances.clone(),
                                        post_balances: meta.post_balances.clone(),
                                        inner_instructions: Some(
                                            meta.inner_instructions.iter().map(|f| {
                                                InnerInstructions {
                                                    index: f.index as u8,
                                                    instructions: f.instructions.iter().map(|v| {
                                                        InnerInstruction {
                                                            instruction: CompiledInstruction {
                                                                program_id_index: v.program_id_index as u8,
                                                                accounts: v.accounts.clone(),
                                                                data: v.data.clone(),
                                                            },
                                                            stack_height: Some(v.stack_height.unwrap()),
                                                        }
                                                    }).collect(),
                                                }
                                            }).collect()
                                        ),
                                        log_messages: Some(
                                            meta.log_messages
                                                .iter()
                                                .map(|f| f.clone())
                                                .collect::<Vec<String>>(),
                                        ),
                                        pre_token_balances: Some(meta.pre_token_balances
                                            .iter()
                                            .map(|tb| TransactionTokenBalance {
                                                account_index: tb.account_index as u8,
                                                mint: tb.mint.clone(),
                                                ui_token_amount: UiTokenAmount {
                                                    ui_amount: {
                                                        let ui_token_amount = tb.ui_token_amount.clone().unwrap_or_default();
                                                        if ui_token_amount.ui_amount == 0.0 {
                                                            None
                                                        } else {
                                                            Some(ui_token_amount.ui_amount)
                                                        }
                                                    },
                                                    decimals: tb
                                                        .ui_token_amount
                                                        .clone()
                                                        .unwrap_or_default()
                                                        .decimals
                                                        as u8,
                                                    amount: tb
                                                        .ui_token_amount
                                                        .clone()
                                                        .unwrap_or_default()
                                                        .amount,
                                                    ui_amount_string: tb
                                                        .ui_token_amount
                                                        .clone()
                                                        .unwrap_or_default()
                                                        .ui_amount_string,
                                                },
                        
                                                owner: tb.clone().owner,
                                                program_id:
                                                    tb.clone().program_id,
                        
                                            })
                                            .collect()),
                                        post_token_balances: Some(meta.post_token_balances
                                            .iter()
                                            .map(|tb| TransactionTokenBalance {
                                                account_index: tb.account_index as u8,
                                                mint: tb.mint.clone(),
                                                ui_token_amount: UiTokenAmount {
                                                    ui_amount: {
                                                        let ui_token_amount = tb.ui_token_amount.clone().unwrap_or_default();
                                                        if ui_token_amount.ui_amount == 0.0 {
                                                            None
                                                        } else {
                                                            Some(ui_token_amount.ui_amount)
                                                        }
                                                    },
                                                    decimals: tb
                                                        .ui_token_amount
                                                        .clone()
                                                        .unwrap_or_default()
                                                        .decimals
                                                        as u8,
                                                    amount: tb
                                                        .ui_token_amount
                                                        .clone()
                                                        .unwrap_or_default()
                                                        .amount,
                                                    ui_amount_string: tb
                                                        .ui_token_amount
                                                        .clone()
                                                        .unwrap_or_default()
                                                        .ui_amount_string,
                                                },
                        
                                                owner: tb.clone().owner,
                                                program_id:
                                                    tb.clone().program_id,
                        
                                            })
                                            .collect(),),
                                        rewards: Some(
                                            meta.rewards
                                                .iter()
                                                .map(|r| Reward {
                                                    pubkey: r.clone().pubkey,
                                                    lamports: r.lamports,
                                                    post_balance: r.post_balance,
                                                    reward_type: match r.reward_type {
                                                        0 => Some(RewardType::Fee),
                                                        1 => Some(RewardType::Rent),
                                                        2 => Some(RewardType::Staking),
                                                        3 => Some(RewardType::Voting),
                                                        _ => None,
                                                    },
                                                    commission: Some(unsafe {
                                                        r.clone().commission.as_bytes_mut()[0]
                                                    }),
                                                })
                                                .collect::<Vec<_>>(),
                                        ),
                                        loaded_addresses: LoadedAddresses {
                                            writable: meta
                                                .loaded_writable_addresses
                                                .iter()
                                                .map(|addr| Pubkey::new_from_array(addr.clone().try_into().expect("Failed to convert Vec<u8> to [u8; 32]")))
                                                .collect(),
                                            readonly: meta
                                                .loaded_readonly_addresses
                                                .iter()
                                                .map(|addr| Pubkey::new_from_array(addr.clone().try_into().expect("Failed to convert Vec<u8> to [u8; 32]")))
                                                .collect(),
                                        },
                                        return_data: meta.return_data.as_ref().map(|return_data| TransactionReturnData {
                                            program_id: Pubkey::new_from_array(
                                                return_data.program_id.clone().try_into().expect("Failed to convert Vec<u8> to [u8; 32]")
                                            ),
                                            data: return_data.data.clone(),
                                        }),
                                        compute_units_consumed: Some(meta.compute_units_consumed.unwrap()),
                                    }
                                }
                            ),
                            block_time: Some(block_time),
                        };

                        let dedoced_txn: Vec<TransactionInstructionWithParent> = match &confirmed_txn_with_meta.tx_with_meta {
                            TransactionWithStatusMeta::Complete(versioned_tx_with_meta) => {
                                flatten_transaction_response(versioned_tx_with_meta)
                            }
                            TransactionWithStatusMeta::MissingMetadata(_) => {
                                vec![]
                            }
                        };

                        let version = Some(if raw_message.versioned {
                            TransactionVersion::Number(0)
                        } else {
                            TransactionVersion::Legacy(Legacy::Legacy)
                        });

                        let mut decoded_actions: Vec<DecodedInstruction> = Vec::new();

                        let idl_json = fs::read_to_string("idls/whirlpool_idl.json")
                        .expect("Unable to read IDL JSON file");

                        let token_idl_json = fs::read_to_string("idls/token_program_idl.json")
                        .expect("Unable to read Token IDL JSON file");
        
                        dedoced_txn.iter().for_each(|instruction| {
                        if instruction.instruction.program_id
                            == Pubkey::from_str(WHIRLPOOL_PROGRAM_ID)
                                .expect("Failed to parse public key")
                        {
                            match WhirlpoolProgramIx::deserialize(&instruction.instruction.data) {
                                Ok(decoded_ix) => {
                                    let idl: Idl =
                                        serde_json::from_str(&idl_json).expect("Failed to deserialize IDL");
                            

                                    match idl
                                        .map_accounts(&instruction.instruction.accounts, &decoded_ix.name())
                                    {
                                        Ok(mapped_accounts) => {
                                            let decoded_instruction = DecodedInstruction {
                                                name: decoded_ix.name(),
                                                accounts: mapped_accounts,
                                                data: match serde_json::to_value(decoded_ix) {
                                                    Ok(data) => data,
                                                    Err(e) => {
                                                        error!("Failed to serialize ix data: {:?}", e);
                                                        return;
                                                    }
                                                },
                                                program_id: instruction.instruction.program_id,
                                                parent_program_id: instruction.parent_program_id,
                                            };
                            
                                            match serde_json::to_string_pretty(&decoded_instruction) {
                                                Ok(json_string) => {
                                                    info!("Decoded Instruction:\n{}", json_string);
                                                    decoded_actions.push(decoded_instruction);
                                                },
                                                Err(e) => error!("Failed to serialize ix data for instruction: {:?}", e),
                                            }
                                        }
                                        Err(err) => error!("Error mapping accounts: {:?}", err),
                                    }
                                }
                                Err(e) => {
                                    error!("Failed to decode instruction: {:?}\n", e);
                                }
                            }}
                            else if instruction.instruction.program_id == Pubkey::from_str(TOKEN_PROGRAM_ID).expect("Failed to parse TOKEN_PROGRAM_ID") {
                                
                                match TokenInstruction::unpack(&instruction.instruction.data) {
                                    Ok(decoded_ix) => {
                                        //println!("Decoded Token Instruction:\n{:?}\n", decoded_ix);
                                        
                                        let ix_name = get_instruction_name_with_typename(&decoded_ix);
                                        //println!("Instruction name: {}", ix_name);

                                        let serializable_ix = convert_to_serializable(decoded_ix);
                                        
                                        let token_idl: Idl =
                                            serde_json::from_str(&token_idl_json).expect("Failed to deserialize IDL");
                                
    
                                        match token_idl
                                            .map_accounts(&instruction.instruction.accounts, &ix_name)
                                        {
                                            Ok(mapped_accounts) => {
                                                let decoded_instruction = DecodedInstruction {
                                                    name: ix_name,
                                                    accounts: mapped_accounts,
                                                    data: match serde_json::to_value(serializable_ix) {
                                                        Ok(data) => data,
                                                        Err(e) => {
                                                            error!("Failed to serialize token ix data: {:?}", e);
                                                            return;
                                                        }
                                                    },
                                                    program_id: instruction.instruction.program_id,
                                                    parent_program_id: instruction.parent_program_id,
                                                };
                                
                                                match serde_json::to_string_pretty(&decoded_instruction) {
                                                    Ok(json_string) => {
                                                        info!("Decoded Token Instruction:\n{}", json_string);
                                                        decoded_actions.push(decoded_instruction);
                                                    },
                                                    Err(e) => error!("Failed to serialize ix data for instruction: {:?}", e),
                                                }
                                            }
                                            Err(err) => error!("Error mapping accounts: {:?}", err),
                                        }
                                        
                                    },
                                    Err(_) => println!("Failed to decode token instruction"),
                                }
                            }  
                    });
                    let decoded_transaction_with_actions = TransactionWithActions {
                        slot,
                        tx_with_meta: confirmed_txn_with_meta.tx_with_meta,
                        block_time,
                        actions: decoded_actions,
                        version,
                        transactions: dedoced_txn
                    };

                    println!("Decoded Transaction: \n{:?}", decoded_transaction_with_actions);  
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

fn flatten_transaction_response(
    transaction_with_meta: &VersionedTransactionWithStatusMeta,
) -> Vec<TransactionInstructionWithParent> {
    let mut result = Vec::new();

    let transaction = transaction_with_meta
        .transaction.clone();
    let ci_ixs = transaction.message.instructions();
    let parsed_accounts = parse_transaction_accounts(
        &transaction.message,
        transaction_with_meta
            .meta
            .loaded_addresses.clone()
    );

    let ordered_cii = match &transaction_with_meta
        .meta
        .inner_instructions
    {
        Some(cii) => {
            let mut cii = cii.clone();
            cii.sort_by(|a, b| a.index.cmp(&b.index));
            cii
        }
        _ => Vec::new(),
    };

    let total_calls: usize = ordered_cii
        .iter()
        .fold(ci_ixs.len(), |acc, cii| acc + cii.instructions.len());

    let mut last_pushed_ix = -1;
    let mut call_index: isize = -1;

    for cii in ordered_cii.iter() {
        while last_pushed_ix != cii.index as i64 {
            last_pushed_ix += 1;
            call_index += 1;
            let ci_ix = &ci_ixs[last_pushed_ix as usize];
            result.push(TransactionInstructionWithParent {
                instruction: compiled_instruction_to_instruction(ci_ix, parsed_accounts.clone()),
                parent_program_id: None,
            });
        }

        for cii_entry in &cii.instructions {
            let parent_program_id =
                parsed_accounts[ci_ixs[last_pushed_ix as usize].program_id_index as usize].pubkey;


                    let ix = CompiledInstruction {
                        program_id_index: cii_entry.instruction.program_id_index,
                        accounts: cii_entry.instruction.accounts.clone(),
                        data: cii_entry.instruction.data.clone(),
                    };
                    result.push(TransactionInstructionWithParent {
                        instruction: compiled_instruction_to_instruction(
                            &ix,
                            parsed_accounts.clone(),
                        ),
                        parent_program_id: Some(parent_program_id),
                    });
                
            

            call_index += 1;
        }
    }

    while call_index < (total_calls - 1).try_into().unwrap() {
        last_pushed_ix += 1;
        call_index += 1;
        let ci_ix = &ci_ixs[last_pushed_ix as usize];
        result.push(TransactionInstructionWithParent {
            instruction: compiled_instruction_to_instruction(ci_ix, parsed_accounts.clone()),
            parent_program_id: None,
        });
    }

    result
}

fn compiled_instruction_to_instruction(
    ci: &CompiledInstruction,
    parsed_accounts: Vec<AccountMeta>,
) -> Instruction {
    let program_id = parsed_accounts[ci.program_id_index as usize].pubkey;
    let accounts: Vec<AccountMeta> = ci.accounts.iter().map(|&index| {
        if index as usize >= parsed_accounts.len() {
            panic!(
                "Trying to resolve account at index {} while parsedAccounts is only {}. \
                Looks like you're trying to parse versioned transaction, make sure that LoadedAddresses are passed to the \
                parseTransactionAccounts function",
                index, parsed_accounts.len()
            );
        }
        parsed_accounts[index as usize].clone()
    }).collect();

    Instruction {
        program_id,
        accounts,
        data: ci.data.clone(),
    }
}

pub fn parse_transaction_accounts(
    message: &VersionedMessage,
    loaded_addresses: LoadedAddresses,
) -> Vec<AccountMeta> {
    let accounts = message.static_account_keys();
    let readonly_signed_accounts_count = message.header().num_readonly_signed_accounts as usize;
    let readonly_unsigned_accounts_count = message.header().num_readonly_unsigned_accounts as usize;
    let required_signatures_accounts_count = message.header().num_required_signatures as usize;
    let total_accounts = accounts.len();

    let mut parsed_accounts: Vec<AccountMeta> = accounts
        .iter()
        .enumerate()
        .map(|(index, pubkey)| {
            let is_writable = index
                < required_signatures_accounts_count - readonly_signed_accounts_count
                || (index >= required_signatures_accounts_count
                    && index < total_accounts - readonly_unsigned_accounts_count);

            AccountMeta {
                pubkey: *pubkey,
                is_signer: index < required_signatures_accounts_count,
                is_writable,
            }
        })
        .collect();

    parsed_accounts.extend(loaded_addresses.writable.into_iter().map(|pubkey| AccountMeta {
        pubkey,
        is_signer: false,
        is_writable: true,
    }));

    parsed_accounts.extend(loaded_addresses.readonly.into_iter().map(|pubkey| AccountMeta {
        pubkey,
        is_signer: false,
        is_writable: false,
    }));

    parsed_accounts
}