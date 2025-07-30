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
    raydium_launchpad_interface::instructions::RaydiumLaunchpadProgramIx,
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
#[derive(Debug, Clone, PartialEq)]
pub struct RLEventType {
     pub token_name : String,
     pub token_symbol : String,
     pub token_supply : u64,
     pub decimal : u8, 
     pub token_uri : String,
     pub creator : String,
     pub token_sold: u64,
     pub token_raised: u64,

}
#[derive(Debug, Clone, Serialize, PartialEq)]
pub struct CompiledInstruction {
    pub data: Option<DecodedInstruction>,
    #[serde(serialize_with = "serialize_pubkey")]
    pub program_id: Pubkey,
    #[serde(serialize_with = "serialize_option_pubkey")]
    pub parent_program_id: Option<Pubkey>,
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
    println!("Getting New Minted Tokens on Raydium Launchpad");
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

                    let raydium_launchpad_idl = fs::read_to_string("idls/raydium_launchpad.json")
                        .expect("Unable to read IDL JSON file");

                    let idl: Idl = serde_json::from_str(&raydium_launchpad_idl)
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
                                    let decoded = match RaydiumLaunchpadProgramIx::deserialize(
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
                           if let Some(parsed_rl_txn) = rl_transaction_formatter(parsed_tx.clone()){
                             println!("{:#?}", parsed_rl_txn);
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
fn rl_transaction_formatter(parsed_txn: ParsedTransaction) -> Option<RLEventType> {
    let initialize_instruction = parsed_txn.message.instructions.iter().find(|instruction| {
        if let Some(DecodedInstruction { name, .. }) = &instruction.data {
            name == "initialize"
        } else {
            false
        }
    });
    if let Some(instruction) = initialize_instruction {
        if let Some(DecodedInstruction { data: serde_json::Value::Object(data), accounts, .. }) = &instruction.data {
            if let Some(serde_json::Value::Object(initialize_data)) = data.get("Initialize") {
                if let Some(serde_json::Value::Object(base_mint_param)) = initialize_data.get("base_mint_param") {
                    let token_name = base_mint_param.get("name")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string();
                    
                    let token_symbol = base_mint_param.get("symbol")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string();
                    
                    let decimal = base_mint_param.get("decimals")
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0) as u8;
                    
                    let token_uri = base_mint_param.get("uri")
                        .and_then(|v| v.as_str())
                        .unwrap_or_default()
                        .to_string();

                    let token_supply = initialize_data.get("curve_param")
                        .and_then(|v| v.get("Constant"))
                        .and_then(|v| v.get("data"))
                        .and_then(|v| v.get("supply"))
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0);
                    
                    let token_sold = initialize_data.get("curve_param")
                        .and_then(|v| v.get("Constant"))
                        .and_then(|v| v.get("data"))
                        .and_then(|v| v.get("total_base_sell"))
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0);
                     
                    let token_raised = initialize_data.get("curve_param")
                        .and_then(|v| v.get("Constant"))
                        .and_then(|v| v.get("data"))
                        .and_then(|v| v.get("total_quote_fund_raising"))
                        .and_then(|v| v.as_u64())
                        .unwrap_or(0);    

                    // Extract creator address from accounts
                    let creator = accounts.iter()
                        .find(|acc| acc.name == "creator")
                        .map(|acc| acc.pubkey.to_string())
                        .unwrap_or_default();

                    return Some(RLEventType {
                        token_name,
                        token_symbol,
                        token_supply,
                        decimal,
                        token_uri,
                        creator,
                        token_sold,
                        token_raised
                    });
                }
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