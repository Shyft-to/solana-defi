use {
    jito_protos::shredstream::{
        shredstream_proxy_client::ShredstreamProxyClient, SubscribeEntriesRequest,
    },
    log::error,
    tonic::{metadata::MetadataValue, transport::Endpoint, Request},
    std::{fs, str::FromStr, time::Duration},
    solana_entry::entry::Entry,
    serde::Serialize,
    solana_sdk::{
        bs58,
       pubkey::Pubkey, hash::Hash, 
        message::{v0::{LoadedAddresses,
              MessageAddressTableLookup},
            VersionedMessage, MessageHeader,
        }
    },
    crate::processor::{
        builder::compiled_instruction_to_instruction,
        parser::parse_transaction_accounts,
    },
    pumpswap_amm_interface::instructions::PumpSwapAmmProgramIx,
    crate::processor::models::{
        mapper::instruction::{AccountMetadata, Idl},
        serialize::serialization::{serialize_pubkey, serialize_option_pubkey},
    },
};

use crate::processor::models::mapper::instruction::InstructionAccountMapper;

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct DecodedInstruction {
    pub name: String,
    pub accounts: Vec<AccountMetadata>,
    pub data: serde_json::Value,
    #[serde(serialize_with = "serialize_pubkey")]
    pub program_id: Pubkey,
    #[serde(serialize_with = "serialize_option_pubkey")]
    pub parent_program_id: Option<Pubkey>,
}

#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CompiledInstruction {
    pub data: Option<DecodedInstruction>,
    #[serde(serialize_with = "serialize_pubkey")]
    pub program_id: Pubkey,
    #[serde(serialize_with = "serialize_option_pubkey")]
    pub parent_program_id: Option<Pubkey>,
}
#[derive(Debug, Clone, PartialEq)]
pub struct PumpCreatedPool {
     pub mint : String,
     pub pool : String,
     pub initial_quote_pool_amount : String,
     pub initial_base_pool_amount : String, 
     pub creator : String,
}
#[derive(Debug, Clone, Serialize)]
pub struct ParsedTransaction {
    pub signatures: Vec<String>,
    pub message: ParsedMessage,
    pub slot: u64,
}
#[derive(Clone, Debug, Serialize,PartialEq)]
pub struct ParsedMessage {
    pub header: MessageHeader,
    pub account_keys: Vec<Pubkey>,
    pub recent_blockhash: Hash,
    pub instructions: Vec<CompiledInstruction>,
    pub address_table_lookups: Vec<MessageAddressTableLookup>,
}
pub async fn connect_and_stream(
    endpoint: &str,
    x_token: Option<&str>,
    pubkey_filter: Option<&str>,
) -> Result<(), Box<dyn std::error::Error>> {
    println!("Listening to Newly Created Pool on Pumpswap Amm..");
    let endpoint = Endpoint::from_str(endpoint)?
        .keep_alive_while_idle(true)
        .http2_keep_alive_interval(Duration::from_secs(5))
        .keep_alive_timeout(Duration::from_secs(10))
        .tcp_keepalive(Some(Duration::from_secs(15)))
        .connect_timeout(Duration::from_secs(5));

    let channel = endpoint.connect().await?;
    let mut client = ShredstreamProxyClient::new(channel);

    let mut request = Request::new(SubscribeEntriesRequest {});
    if let Some(token) = x_token {
        let metadata_value = MetadataValue::from_str(token)?;
        request.metadata_mut().insert("x-token", metadata_value);
    }

    let mut stream = client.subscribe_entries(request).await?.into_inner();

    while let Some(result) = stream.message().await.transpose() {
        match result {
            Ok(slot_entry) => {
                let entries = match bincode::deserialize::<Vec<Entry>>(&slot_entry.entries) {
                    Ok(e) => e,
                    Err(e) => {
                        eprintln!("Deserialization failed: {e}");
                        continue;
                    }
                };

                let matching_entries = if let Some(pubkey) = pubkey_filter {
                    entries
                        .iter()
                        .filter(|entry| {
                            entry.transactions.iter().any(|tx| {
                                let account_keys = match &tx.message {
                                    VersionedMessage::Legacy(msg) => &msg.account_keys,
                                    VersionedMessage::V0(msg) => &msg.account_keys,
                                };

                                account_keys.iter().any(|key| key.to_string() == pubkey)
                            })
                        })
                        .collect::<Vec<_>>()
                } else {
                    entries.iter().collect()
                };

                if !matching_entries.is_empty() {
                  
                    let pump_amm_idl = fs::read_to_string("idls/pump_amm.json")
                        .expect("Unable to read IDL JSON file");

                    let idl: Idl = serde_json::from_str(&pump_amm_idl)
                        .expect("Failed to deserialize IDL");

                    matching_entries.iter().for_each(|entry| {
                        entry.transactions.iter().for_each(|tx| {
                            let loaded_addresses = match &tx.message {
                                VersionedMessage::Legacy(_) => LoadedAddresses {
                                    writable: vec![],
                                    readonly: vec![],
                                },
                                VersionedMessage::V0(_) => LoadedAddresses {
                                    writable: vec![],
                                    readonly: vec![],
                                },
                            };

                            let parsed_accounts =
                                parse_transaction_accounts(&tx.message, loaded_addresses);

                            let parsed_instructions: Vec<CompiledInstruction> = tx
                                .message
                                .instructions()
                                .iter()
                                .map(|ci| {
                                    let instruction = compiled_instruction_to_instruction(
                                        ci,
                                        parsed_accounts.clone(),
                                    );
                                    let program_id = instruction.program_id;
                                    let decoded = match PumpSwapAmmProgramIx::deserialize(
                                        &instruction.data,
                                    ) {
                                        Ok(decoded_ix) => {
                                            let decode_ix_formatted = pascal_to_snake_case(&decoded_ix.name().to_string()); 
                                            match idl.map_accounts(
                                                &instruction.accounts,
                                                &decoded_ix.name().to_string(),
                                            ) {
                                                Ok(mapped_accounts) => {
                                                    Some(DecodedInstruction {
                                                        name: decode_ix_formatted,
                                                        accounts: mapped_accounts,
                                                        data: match serde_json::to_value(decoded_ix) {
                                                            Ok(data) => data,
                                                            Err(e) => {
                                                                error!("Failed to serialize ix data: {:?}", e);
                                                                serde_json::json!(null)
                                                            }
                                                        },
                                                        program_id,
                                                        parent_program_id: None,
                                                    })
                                                }
                                                Err(err) => {
                                                    error!(
                                                        "Error mapping accounts: {:?}",
                                                        err
                                                    );
                                                    None
                                                }
                                            }
                                        }
                                        Err(_) => None,
                                    };

                                    CompiledInstruction {
                                        data: decoded,
                                        program_id,
                                        parent_program_id: None,
                                    }
                                })
                                .collect();

                            let parsed_tx = ParsedTransaction {
                                signatures: tx.signatures.iter().map(|s| s.to_string()).collect(),
                                message: match &tx.message {
                                    VersionedMessage::V0(msg) => ParsedMessage {
                                        header: msg.header.clone(), 
                                        account_keys: msg.account_keys.clone(),
                                        recent_blockhash: msg.recent_blockhash.clone(),
                                        instructions: parsed_instructions.clone(), 
                                        address_table_lookups: msg.address_table_lookups.clone(),
                                    },
                                    VersionedMessage::Legacy(msg) => ParsedMessage {
                                        header: msg.header.clone(),
                                        account_keys: msg.account_keys.clone(),
                                        recent_blockhash: msg.recent_blockhash.clone(),
                                        instructions: parsed_instructions.clone(), 
                                        address_table_lookups: vec![], 
                                    },
                                },
                                slot: slot_entry.slot,
                            };
                           if let Some(new_pool_txn) = pump_amm_formatter(parsed_tx){;
                            println!("{:#?}", new_pool_txn);
                           }
                        });
                    });
                }
            }
            Err(e) => {
                eprintln!("stream error: {e}");
                return Err(Box::new(e));
            }
        }
    }

    Ok(())
}

fn pump_amm_formatter(parsed_txn: ParsedTransaction) -> Option<PumpCreatedPool> {
    let create_instruction = parsed_txn.message.instructions.iter().find(|instruction| {
        if let Some(DecodedInstruction { name, .. }) = &instruction.data {
            name == "create_pool"
        } else {
            false
        }
    });
    
   if let Some(instruction) = create_instruction {
        if let Some(DecodedInstruction { data: serde_json::Value::Object(data), accounts, .. }) = &instruction.data {
            if let Some(serde_json::Value::Object(create_data)) = data.get("CreatePool") {
                let mint = accounts.iter()
                    .find(|acct| (acct.name == "base_mint" || acct.name == "quote_mint") &&
                       acct.pubkey.to_string() != "So11111111111111111111111111111111111111112"
                        )
                    .map(|acct| acct.pubkey.to_string())
                    .unwrap_or_default();
                
                let pool = accounts.iter()
                    .find(|acct| acct.name == "pool")
                    .map(|acct| acct.pubkey.to_string())
                    .unwrap_or_default();
                         

                let initial_quote_pool_amount = create_data.get("quote_amount_in")
                    .and_then(|v| v.as_u64())
                    .map(|v| v.to_string())
                    .unwrap_or_default();


                let initial_base_pool_amount = create_data.get("base_amount_in")
                    .and_then(|v| v.as_u64())
                    .map(|v| v.to_string())
                    .unwrap_or_default();

                let creator_bytes = create_data.get("coin_creator")
                    .and_then(|v| v.as_array());

                let creator = creator_bytes
                    .and_then(|arr| {
                        let byte_vec: Vec<u8> = arr.iter()
                            .filter_map(|v| v.as_u64().map(|num| num as u8))
                            .collect();
                        if byte_vec.len() == 32 {
                            Some(bs58::encode(byte_vec).into_string())
                        } else {
                            None
                        }
                    })
                    .unwrap_or_default();

                return Some(PumpCreatedPool {
                    mint,
                    pool,
                    initial_quote_pool_amount,
                    initial_base_pool_amount,
                    creator
                });
            }
        }
    }

    None
}


fn pascal_to_snake_case(input: &str) -> String {
    let mut snake = String::new();

    for (i, ch) in input.chars().enumerate() {
        if ch.is_uppercase() {
            if i != 0 {
                snake.push('_');
            }
            for lower_ch in ch.to_lowercase() {
                snake.push(lower_ch);
            }
        } else {
            snake.push(ch);
        }
    }

    snake
}