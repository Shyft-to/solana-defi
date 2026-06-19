use {
    anyhow::Context,
    solana_account_decoder_client_types::token::UiTokenAmount,
    solana_sdk::{
        instruction::CompiledInstruction,
        message::{
            v0::{LoadedAddresses, Message, MessageAddressTableLookup},
            MessageHeader, VersionedMessage,
        },
        pubkey::Pubkey,
        signature::Signature,
        transaction::VersionedTransaction,
        transaction_context::TransactionReturnData,
    },
    solana_transaction_status::{
        ConfirmedTransactionWithStatusMeta, InnerInstruction, InnerInstructions, Reward,
        RewardType, TransactionStatusMeta, TransactionTokenBalance,
        TransactionWithStatusMeta, VersionedTransactionWithStatusMeta,
    },
    crate::{
        TransactionProcessor, ParsedConfirmedTransactionWithStatusMeta, ParsedMessage,
        ParsedTransaction, ParsedTransactionStatusMeta, processor::types::DecodedInstruction,
    },
};


impl TransactionProcessor {
    pub fn build_confirmed_transaction(
        slot: u64,
        signature: Signature,
        raw_message: yellowstone_grpc_proto::prelude::Message,
        meta: yellowstone_grpc_proto::prelude::TransactionStatusMeta,
        block_time: i64,
    ) -> anyhow::Result<ConfirmedTransactionWithStatusMeta> {
        let header = raw_message.header.context("header empty")?;

        Ok(ConfirmedTransactionWithStatusMeta {
            slot,
            tx_with_meta: TransactionWithStatusMeta::Complete(VersionedTransactionWithStatusMeta {
                transaction: VersionedTransaction {
                    signatures: vec![signature],
                    message: VersionedMessage::V0(Message {
                        header: MessageHeader {
                            num_required_signatures: header.num_required_signatures as u8,
                            num_readonly_signed_accounts: header.num_readonly_signed_accounts as u8,
                            num_readonly_unsigned_accounts: header.num_readonly_unsigned_accounts as u8,
                        },
                        account_keys: raw_message
                            .account_keys
                            .iter()
                            .map(|k| {
                                k.clone()
                                    .try_into()
                                    .map(Pubkey::new_from_array)
                                    .map_err(|e| anyhow::anyhow!("Failed to convert account key: {:?}", e))
                            })
                            .collect::<anyhow::Result<Vec<_>>>()?,
                        recent_blockhash: Self::parse_blockhash(&raw_message.recent_blockhash)?,
                        instructions: raw_message
                            .instructions
                            .iter()
                            .map(|ix| CompiledInstruction {
                                program_id_index: ix.program_id_index as u8,
                                accounts: ix.accounts.clone(),
                                data: ix.data.clone(),
                            })
                            .collect(),
                        address_table_lookups: raw_message
                            .address_table_lookups
                            .iter()
                            .map(|l| {
                                let account_key = l.account_key
                                    .clone()
                                    .try_into()
                                    .map(Pubkey::new_from_array)
                                    .map_err(|e| anyhow::anyhow!("Failed to convert address_table_lookup account_key: {:?}", e))?;

                                Ok(MessageAddressTableLookup {
                                    account_key,
                                    writable_indexes: l.writable_indexes.clone(),
                                    readonly_indexes: l.readonly_indexes.clone(),
                                })
                            })
                            .collect::<anyhow::Result<Vec<_>>>()?,
                    }),
                },
                meta: TransactionStatusMeta {
                    status: Ok(()),
                    fee: meta.fee,
                    pre_balances: meta.pre_balances.clone(),
                    post_balances: meta.post_balances.clone(),
                    inner_instructions: Some(
                        meta.inner_instructions
                            .iter()
                            .map(|f| InnerInstructions {
                                index: f.index as u8,
                                instructions: f.instructions
                                    .iter()
                                    .map(|v| InnerInstruction {
                                        instruction: CompiledInstruction {
                                            program_id_index: v.program_id_index as u8,
                                            accounts: v.accounts.clone(),
                                            data: v.data.clone(),
                                        },
                                        stack_height: Some(v.stack_height.unwrap_or(0)),
                                    })
                                    .collect(),
                            })
                            .collect(),
                    ),
                    log_messages: Some(meta.log_messages.iter().cloned().collect()),
                    pre_token_balances: Some(
                        meta.pre_token_balances
                            .into_iter()
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
                                    decimals: tb.ui_token_amount.clone().unwrap_or_default().decimals as u8,
                                    amount: tb.ui_token_amount.clone().unwrap_or_default().amount,
                                    ui_amount_string: tb.ui_token_amount.clone().unwrap_or_default().ui_amount_string,
                                },
                                owner: tb.owner.clone(),
                                program_id: tb.program_id.clone(),
                            })
                            .collect(),
                    ),
                    post_token_balances: Some(
                        meta.post_token_balances
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
                                    decimals: tb.ui_token_amount.clone().unwrap_or_default().decimals as u8,
                                    amount: tb.ui_token_amount.clone().unwrap_or_default().amount,
                                    ui_amount_string: tb.ui_token_amount.clone().unwrap_or_default().ui_amount_string,
                                },
                                owner: tb.owner.clone(),
                                program_id: tb.program_id.clone(),
                            })
                            .collect(),
                    ),
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
                            .map(|addr| {
                                addr.clone()
                                    .try_into()
                                    .map(Pubkey::new_from_array)
                                    .map_err(|e| anyhow::anyhow!("Failed to convert writable address: {:?}", e))
                            })
                            .collect::<anyhow::Result<Vec<_>>>()?,
                        readonly: meta
                            .loaded_readonly_addresses
                            .iter()
                            .map(|addr| {
                                addr.clone()
                                    .try_into()
                                    .map(Pubkey::new_from_array)
                                    .map_err(|e| anyhow::anyhow!("Failed to convert readonly address: {:?}", e))
                            })
                            .collect::<anyhow::Result<Vec<_>>>()?,
                    },
                    return_data: meta.return_data.as_ref().map(|return_data| -> anyhow::Result<TransactionReturnData> {
                        Ok(TransactionReturnData {
                            program_id: return_data
                                .program_id
                                .clone()
                                .try_into()
                                .map(Pubkey::new_from_array)
                                .map_err(|e| anyhow::anyhow!("Failed to convert return_data program_id: {:?}", e))?,
                            data: return_data.data.clone(),
                        })
                    }).transpose()?,
                    compute_units_consumed: meta.compute_units_consumed,
                },
            }),
            block_time: Some(block_time),
        })
    }
    pub fn build_parsed_transaction(
        slot: u64,
        confirmed_txn: &ConfirmedTransactionWithStatusMeta,
        decoded_compiled: Vec<DecodedInstruction>,
        decoded_inner: Vec<DecodedInstruction>,
        block_time: i64,
    ) -> anyhow::Result<ParsedConfirmedTransactionWithStatusMeta> {
        match &confirmed_txn.tx_with_meta {
            TransactionWithStatusMeta::Complete(versioned_tx_with_meta) => {
                Ok(ParsedConfirmedTransactionWithStatusMeta {
                    slot,
                    transaction: ParsedTransaction {
                        signatures: versioned_tx_with_meta.transaction.signatures.clone(),
                        message: match &versioned_tx_with_meta.transaction.message {
                            VersionedMessage::V0(msg) => ParsedMessage {
                                header: msg.header.clone(),
                                account_keys: msg.account_keys.clone(),
                                recent_blockhash: msg.recent_blockhash.clone(),
                                instructions: decoded_compiled,
                                address_table_lookups: msg.address_table_lookups.clone(),
                            },
                            VersionedMessage::Legacy(msg) => ParsedMessage {
                                header: msg.header.clone(),
                                account_keys: msg.account_keys.clone(),
                                recent_blockhash: msg.recent_blockhash.clone(),
                                instructions: decoded_compiled,
                                address_table_lookups: vec![],
                            },
                        },
                    },
                    meta: ParsedTransactionStatusMeta {
                        status: versioned_tx_with_meta.meta.status.clone(),
                        fee: versioned_tx_with_meta.meta.fee,
                        pre_balances: versioned_tx_with_meta.meta.pre_balances.clone(),
                        post_balances: versioned_tx_with_meta.meta.post_balances.clone(),
                        inner_instructions: decoded_inner,
                        log_messages: versioned_tx_with_meta.meta.log_messages.clone(),
                        pre_token_balances: versioned_tx_with_meta.meta.pre_token_balances.clone(),
                        post_token_balances: versioned_tx_with_meta.meta.post_token_balances.clone(),
                        rewards: versioned_tx_with_meta.meta.rewards.clone(),
                        loaded_addresses: versioned_tx_with_meta.meta.loaded_addresses.clone(),
                        return_data: versioned_tx_with_meta.meta.return_data.clone(),
                        compute_units_consumed: versioned_tx_with_meta.meta.compute_units_consumed,
                    },
                    block_time: Some(block_time),
                })
            }
            _ => anyhow::bail!("Expected Complete variant"),
        }
    }

}