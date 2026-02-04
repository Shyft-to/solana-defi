use {
    crate::{
        processor::models::mapper::event::DecodedEvent,
        processor::types::PriceInfo,
        processor::types::PoolState,
        processor::types::TradeBehavior,
        processor::types::TransactionEvent,
        processor::types::FeeInfo,
        processor::types::FeePart,
        ParsedConfirmedTransactionWithStatusMeta,
        ParsedEventTransaction,
        ParsedTransaction,
        ParsedMessage,
        ParsedTransactionStatusMeta,
        TransactionProcessor,
    },
    solana_sdk::{
        instruction::AccountMeta,
        message::{v0::LoadedAddresses, VersionedMessage},
    },
    std::vec::Vec,
    chrono::NaiveDateTime,
};

impl TransactionProcessor {
    pub fn parse_transaction_accounts(
        &self,
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

    
    pub fn parsed_pump_txn(
    &self,
    original: ParsedConfirmedTransactionWithStatusMeta,
    ) -> Option<TransactionEvent> {
     let meta = &original.meta;
     let tx = &original.transaction;

     let mut all_instructions = tx
         .message
         .instructions
         .iter()
         .chain(meta.inner_instructions.iter());

     let swap_instruction = all_instructions.find(|ix| {
        matches!(
            ix.name.as_str(),
            "buy" | "sell" | "buyExactSolIn"
        )
     })?;

     let ix_type = swap_instruction.name.clone();

     let bonding_curve = swap_instruction
        .accounts
        .iter()
        .find(|a| a.name == "bonding_curve")
        .map(|a| a.pubkey.to_string());

     let user_pubkey = swap_instruction
        .accounts
        .iter()
        .find(|a| a.name == "user")
        .map(|a| a.pubkey.to_string());

     let mint = swap_instruction
        .accounts
        .iter()
        .find(|a| a.name == "mint")
        .map(|a| a.pubkey.to_string());

     let trade_event = meta
        .inner_instructions
        .iter()
        .chain(tx.message.instructions.iter())
        .find_map(|ix| {
            if let Some(DecodedEvent::TradeEvent(event)) = &ix.event {
                Some(event)
            } else {
                None
            }
        })?;

     let sol_amount = trade_event.sol_amount;
     let token_amount = trade_event.token_amount;
     let timestamp = trade_event.timestamp;

     let virtual_sol_reserves = trade_event.virtual_sol_reserves;
     let virtual_token_reserves = trade_event.virtual_token_reserves;
     let real_sol_reserves = trade_event.real_sol_reserves;
     let real_token_reserves = trade_event.real_token_reserves;

     let fee = trade_event.fee;
     let fee_basis_points = trade_event.fee_basis_points;
     let creator_fee = trade_event.creator_fee;
     let creator_fee_basis_points = trade_event.creator_fee_basis_points;

     let is_buy = matches!(ix_type.as_str(), "buy" | "buyExactSolIn");

     let in_amount = if is_buy {
        sol_amount
     } else {
        token_amount
     };

     let out_amount = if is_buy {
        token_amount
     } else {
        sol_amount
     };

     let readable_time = NaiveDateTime::from_timestamp_opt(timestamp as i64, 0)
        .map(|t| t.to_string());

    // === prices ===
     let price_sol_per_token = if sol_amount > 0 && token_amount > 0 {
        Some(sol_amount as f64 / token_amount as f64)
     } else {
        None
     };

     let market_price =
        calculate_pump_fun_price(virtual_sol_reserves, virtual_token_reserves);

     let large_trade = sol_amount > 5_000_000_000;
     let early_pool = virtual_sol_reserves < 20_000_000_000;

     Some(TransactionEvent {
        event_type: Some(ix_type),
        user: user_pubkey,
        mint,
        bonding_curve,

        amount_in: Some(in_amount),
        amount_out: Some(out_amount),

        timestamp_in_blockchain: Some(timestamp),
        readable_time_of_trade: readable_time,

        price: Some(PriceInfo {
            sol_per_token: price_sol_per_token,
            token_per_sol: price_sol_per_token.map(|p| 1.0 / p),
            market_price,
        }),

        pool_state: Some(PoolState {
            virtual_sol: virtual_sol_reserves,
            virtual_token: virtual_token_reserves,
            real_sol: real_sol_reserves,
            real_token: real_token_reserves,
        }),

        fees: Some(FeeInfo {
            protocol: FeePart {
                amount: fee,
                bps: fee_basis_points,
            },
            creator: FeePart {
                amount: creator_fee,
                bps: creator_fee_basis_points,
            },
        }),

        behavior: Some(TradeBehavior {
            is_buy,
            is_sell: !is_buy,
            large_trade,
            early_pool,
        }),
    })
 }
}
fn calculate_pump_fun_price(
    virtual_sol_reserves: u64,
    virtual_token_reserves: u64,
) -> Option<String> {
    if virtual_token_reserves == 0 {
        return None;
    }

    let sol = virtual_sol_reserves as f64 / 1_000_000_000.0;
    let tokens = virtual_token_reserves as f64 / 1_000_000.0;

    let price = sol / tokens;
    Some(format!("{:.20}", price).trim_end_matches('0').to_string())
}