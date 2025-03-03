use {
    anyhow::Context,
    backoff::{future::retry, ExponentialBackoff},
    clap::Parser as ClapParser,
    futures::{future::TryFutureExt, sink::SinkExt, stream::StreamExt},
    log::{error, info},
    serde_json::json,
    serde::{Serialize, Deserialize},
    serde_json::Value,
    std::{
      collections::HashMap, env, fs, str::FromStr, sync::Arc, time::{Duration, SystemTime, UNIX_EPOCH}
    },
    tokio::io::{AsyncReadExt, AsyncWriteExt}, // These traits are needed for async I/O
    //std::net::{TcpListener, TcpStream},
    std::io::{Read, Write},
    std::env::var,
        serialization::{serialize_pubkey, serialize_option_pubkey},
    tokio::task,
    solana_transaction_status::{UiTransactionEncoding,
      ConfirmedTransactionWithStatusMeta, InnerInstruction, InnerInstructions, Reward, RewardType, TransactionStatusMeta, TransactionTokenBalance, TransactionWithStatusMeta, VersionedTransactionWithStatusMeta
    },
    instruction_account_mapper::{AccountMetadata, Idl, InstructionAccountMapper}, 
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
        SubscribeRequestFilterTransactions,SubscribeUpdateTransactionInfo,SubscribeRequestFilterBlocks
    },
    pump_interface::instructions::PumpProgramIx,
    yellowstone_grpc_proto::convert_from
  };
  use spl_token::instruction::TokenInstruction;

  use chrono::{DateTime, NaiveDateTime, Utc};
  //type TransactionsFilterMap = HashMap<String, SubscribeRequestFilterTransactions>;
  type BlocksFilterMap = HashMap<String, SubscribeRequestFilterBlocks>;
  
  mod serialization;
  mod instruction_account_mapper;
  mod token_serializable;
  const PROGRAM_ID: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";
  
  
  
  #[derive(Debug, Clone, ClapParser)]
  #[clap(author, version, about)]
  struct Args {
    #[clap(short, long, help = "gRPC endpoint")]
    /// Service endpoint
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
  
    pub fn get_raydium_pool_subscribe_request(&self) -> anyhow::Result<SubscribeRequest> {
      let mut blocks: BlocksFilterMap = HashMap::new();
      blocks.insert(
            "client".to_owned(),
            SubscribeRequestFilterBlocks {
                account_include:vec![PROGRAM_ID.to_string()] ,
                include_transactions: None,
                include_accounts: None,
                include_entries: None,
            },
        );
  
        Ok(SubscribeRequest {
            slots: HashMap::default(),
            accounts: HashMap::default(),
            transactions : HashMap::default(),
            transactions_status: HashMap::default(),
            entry: HashMap::default(),
            blocks: blocks,
            blocks_meta: HashMap::default(),
            commitment: Some(CommitmentLevel::Processed as i32),
            accounts_data_slice: Vec::default(),
            ping: None,
            from_slot: None,
        })
    }
  }
  #[derive(Debug,Serialize)]
  #[derive(Clone)]
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
  fn to_camel_case(name: &str) -> String {
      let mut chars = name.chars();
      match chars.next() {
          Some(first_char) => first_char.to_lowercase().collect::<String>() + chars.as_str(),
          None => String::new(),
      }
  }
  
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
                    let slot : &u64 = &msg.slot;
                    let slot_cloned = slot.clone();
                    let block_hash : &str = &msg.blockhash;
                    let block_time = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time went backwards")
                    .as_secs() as i64;
                    let datetime = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(block_time, 0), Utc);
                    let formatted_time = datetime.format("%Y-%m-%d %H:%M:%S").to_string();
                  //   let transactions = msg.transactions;
                    let transactions: Vec<
                    yellowstone_grpc_proto::prelude::SubscribeUpdateTransactionInfo,
                > = msg.transactions;
                    for txn in transactions {
                      let raw_signature = txn.signature.clone();
                      let raw_signature_array: [u8; 64] = raw_signature.try_into().expect("Failed to convert to [u8; 64]");
                      let signature = Signature::from(raw_signature_array);
                      let meta = txn.meta.expect("Meta empty");
                      let raw_transaction = txn.transaction.expect("transaction empty");
                      let raw_message = raw_transaction.message.expect("message empty").clone();
                      let header = raw_message.header.expect("header empty");
                      let recent_blockhash=  Hash::new_from_array(raw_message
                          .recent_blockhash
                          .clone()
                          .try_into()
                          .expect("Failed to convert Vec<u8> to [u8; 32]"));
                      let confirmed_txn_with_meta = ConfirmedTransactionWithStatusMeta {
                          slot: *slot,
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
                      let decoded_txn: Vec<TransactionInstructionWithParent> = match &confirmed_txn_with_meta.tx_with_meta {
                          TransactionWithStatusMeta::Complete(versioned_tx_with_meta) => {
                              flatten_transaction_response(versioned_tx_with_meta)
                          }
                          TransactionWithStatusMeta::MissingMetadata(_) => {
                              vec![]
                          }
                  };
                  let decoded_txn_cloned = decoded_txn.clone(); // Clone it before spawning tasks

                  let idl_json = fs::read_to_string("idls/pump_0.1.0.json")
                  .expect("Unable to read IDL JSON file");
  
                  let token_idl_json = fs::read_to_string("idls/token_program_idl.json")
                  .expect("Unable to read Token IDL JSON file");
  
                  decoded_txn_cloned.iter().for_each(|instruction| {
                    let instruction = instruction.clone();
                    let signature = signature.to_string();
                    let idl_json = idl_json.clone();
                    let token_idl_json = token_idl_json.clone();
              task::spawn(async move {   
                  if instruction.instruction.program_id
                      == Pubkey::from_str(PROGRAM_ID)
                          .expect("Failed to parse public key")
                  {
                      match PumpProgramIx::deserialize(&instruction.instruction.data) {
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
                                            let event = json_string.clone().to_string().replace("\n", "").replace("\"", "").replace("\r", "");
                                             let parsed_json: Value = serde_json::from_str(&json_string).expect("Invalid JSON");
                                            let name = &parsed_json["data"].as_object().unwrap();
                                            let (key,value) = name.iter().next().unwrap();
                                            let value_object = value.as_object().unwrap();
                                            let amount = value_object["amount"].as_i64().unwrap();
                                            let max_amount_in = value_object["max_sol_cost"].as_i64().unwrap_or(0);
                                            let amount_in_lamport = max_amount_in as f64 / 1000000000.0;
                                            let data = &parsed_json["accounts"].as_array().unwrap();
                                            let signer = data.iter().find(|obj| obj["name"] == "user").unwrap();
                                            let mint = data.iter().find(|obj| obj["name"]== "mint").unwrap();
                                            let mint_pubkey = mint["pubkey"].as_str().unwrap();
                                            let signer_pubkey = signer["pubkey"].as_str().unwrap();
                                             //info!("data : {:?}", signer);
                                             info!("
                                             TYPE: {:?}
                                             MINT: {:?}
                                             SIGNER: {:?} 
                                             TOKEN AMOUNT: {:?}
                                             SOL : {:?} sol
                                             SIGNATURE: {:?}: 
                                             ",
                                             key,mint_pubkey,signer_pubkey,amount,amount_in_lamport,signature);
                                            
                                         }
                                          Err(e) => error!("Failed to serialize ix data for instruction: {:?}", e),
                                      }
                                  }
                                  Err(err) => error!("Error mapping accounts: {:?}", err),
                              }
                          }
                          Err(e) => {
                          }
                      }}
                    });
                   })
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
  