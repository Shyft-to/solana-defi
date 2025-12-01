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
use yellowstone_grpc_proto::prelude as yp;


impl TransactionProcessor {
    pub fn build_confirmed_transaction(
        slot: u64,
        signature: Signature,
        raw_message: yp::Message,
        maybe_meta: Option<yp::TransactionStatusMeta>,
        block_time: i64,
    ) -> anyhow::Result<ConfirmedTransactionWithStatusMeta> {
        let header = raw_message.header.context("header empty")?;
        let meta = if let Some(m) = maybe_meta {
            m
        } else {
            yp::TransactionStatusMeta {
                fee: 0,
                pre_balances: vec![],
                post_balances: vec![],
                inner_instructions: vec![],
                log_messages: vec![],
                pre_token_balances: vec![],
                post_token_balances: vec![],
                rewards: vec![],
                loaded_writable_addresses: vec![],
                loaded_readonly_addresses: vec![],
                return_data: None,
                compute_units_consumed: None,
                ..Default::default()
            }
        };

        let pre_token_balances: Vec<TransactionTokenBalance> = meta
            .pre_token_balances
            .iter()
            .map(|tb| {
                let ui = tb.ui_token_amount.clone().unwrap_or_default();
                TransactionTokenBalance {
                    account_index: tb.account_index as u8,
                    mint: tb.mint.clone(),
                    ui_token_amount: solana_account_decoder_client_types::token::UiTokenAmount {
                        ui_amount:Some(ui.ui_amount),
                        decimals: ui.decimals as u8,
                        amount: ui.amount,
                        ui_amount_string: ui.ui_amount_string,
                    },
                    owner: tb.owner.clone(),
                    program_id: tb.program_id.clone(),
                }
            })
            .collect();

        let post_token_balances: Vec<TransactionTokenBalance> = meta
            .post_token_balances
            .iter()
            .map(|tb| {
                let ui = tb.ui_token_amount.clone().unwrap_or_default();
                TransactionTokenBalance {
                    account_index: tb.account_index as u8,
                    mint: tb.mint.clone(),
                    ui_token_amount: solana_account_decoder_client_types::token::UiTokenAmount {
                        ui_amount: Some(ui.ui_amount),
                        decimals: ui.decimals as u8,
                        amount: ui.amount,
                        ui_amount_string: ui.ui_amount_string,
                    },
                    owner: tb.owner.clone(),
                    program_id: tb.program_id.clone(),
                }
            })
            .collect();

        let rewards: Vec<Reward> = meta
            .rewards
            .iter()
            .map(|r| Reward {
                pubkey: r.pubkey.clone(),
                lamports: r.lamports,
                post_balance: r.post_balance,
                reward_type: match r.reward_type {
                    0 => Some(RewardType::Fee),
                    1 => Some(RewardType::Rent),
                    2 => Some(RewardType::Staking),
                    3 => Some(RewardType::Voting),
                    _ => None,
            },
                commission: r.commission.parse::<u8>().ok(),
             })
            .collect();
        Ok(ConfirmedTransactionWithStatusMeta {
            slot,
            tx_with_meta: TransactionWithStatusMeta::Complete(VersionedTransactionWithStatusMeta {
                meta: TransactionStatusMeta {
                    status: Ok(()),
                    fee: meta.fee,
                    pre_balances: meta.pre_balances.clone(),
                    post_balances: meta.post_balances.clone(),
                    inner_instructions: Some(
                        meta.inner_instructions
                            .iter()
                            .enumerate()
                            .map(|(i, _)| InnerInstructions {
                                index: i as u8,
                                instructions: vec![],
                            })
                            .collect(),
                    ),
                    log_messages: Some(meta.log_messages.clone()),
                    pre_token_balances: Some(pre_token_balances),
                    post_token_balances: Some(post_token_balances),
                    rewards: Some(rewards),
                    loaded_addresses: LoadedAddresses {
                        writable: meta
                            .loaded_writable_addresses
                            .iter()
                            .filter_map(|a| a.clone().try_into().ok().map(Pubkey::new_from_array))
                            .collect(),
                        readonly: meta
                            .loaded_readonly_addresses
                            .iter()
                            .filter_map(|a| a.clone().try_into().ok().map(Pubkey::new_from_array))
                            .collect(),
                    },
                    return_data: None,
                    compute_units_consumed: meta.compute_units_consumed,
                },
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
                                let arr: [u8; 32] = k
                                    .clone()
                                    .try_into()
                                    .map_err(|e: Vec<u8>| anyhow::anyhow!(
                                        "Failed to convert account key: {:?}", e
                                    ))?;
                                Ok(Pubkey::new_from_array(arr))
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
                                let arr: [u8; 32] = l
                                    .account_key
                                    .clone()
                                    .try_into()
                                    .map_err(|e: Vec<u8>| anyhow::anyhow!(
                                        "Failed to convert address_table_lookup key: {:?}", e
                                    ))?;
                                Ok(MessageAddressTableLookup {
                                    account_key: Pubkey::new_from_array(arr),
                                    writable_indexes: l.writable_indexes.clone(),
                                    readonly_indexes: l.readonly_indexes.clone(),
                                })
                            })
                            .collect::<anyhow::Result<Vec<_>>>()?,
                    }),
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
                    block_time: Some(block_time),
                })
            }
            _ => anyhow::bail!("Expected Complete variant"),
        }
    }

}