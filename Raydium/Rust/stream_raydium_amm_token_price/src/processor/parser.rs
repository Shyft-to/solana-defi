use {
    crate::{
        processor::models::mapper::event::DecodedEvent,
        processor::types::RaydiumSwapParsed,
        ParsedConfirmedTransactionWithStatusMeta,
        ParsedEventTransaction,
        ParsedTransaction,
        ParsedMessage,
        ParsedTransactionStatusMeta,
        TransactionEvent,
        TransactionProcessor,
        DecodedInstruction,
        TransactionTokenBalance,
    },
    solana_sdk::{
        instruction::AccountMeta,
        message::{v0::LoadedAddresses, VersionedMessage},
    },
    std::vec::Vec,
    std::fmt,
};

// Implement custom Debug for RaydiumSwapParsed to show formatted values
impl fmt::Debug for RaydiumSwapParsed {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RaydiumSwapParsed")
            .field("base_mint", &self.base_mint)
            .field("quote_mint", &self.quote_mint)
            .field("base_decimals", &self.base_decimals)
            .field("quote_decimals", &self.quote_decimals)
            .field("amount_in", &self.amount_in)
            .field("amount_out", &self.amount_out)
            .field("amount_in_formatted", &self.amount_in_formatted)
            .field("amount_out_formatted", &self.amount_out_formatted)
            .field("pool_base_reserves", &self.pool_base_reserves)
            .field("pool_quote_reserves", &self.pool_quote_reserves)
            .field("pool_base_reserves_decimal", &self.pool_base_reserves_decimal)
            .field("pool_quote_reserves_decimal", &self.pool_quote_reserves_decimal)
            .field("base_token_price", &self.base_token_price)
            .field("quote_token_price", &self.quote_token_price)
            .field("pool_base_token_price", &self.pool_base_token_price)
            .field("pool_quote_token_price", &self.pool_quote_token_price)
            .field("swap_type", &self.swap_type)
            .field("direction", &self.direction)
            .field("pool_price", &self.pool_price)
            .field("price_impact", &self.price_impact)
            .finish()
    }
}

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
    
    pub fn parse_raydium_swap(
        &self,
        decoded_instructions: &[DecodedInstruction],
        events: &[DecodedEvent],
        pre_token_balances: &[TransactionTokenBalance],
        post_token_balances: &[TransactionTokenBalance],
    ) -> Option<RaydiumSwapParsed> {   
        const SOL_MINT: &str = "So11111111111111111111111111111111111111112"; 
        const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
        const USDT_MINT: &str = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";

        // Find the swap instruction
        let swap_ix = decoded_instructions.iter().find(|ix| {
            ix.program_id == self.RAYDIUM_PROGRAM_ID &&
            (ix.name == "swapBaseIn" 
             || ix.name == "swapBaseOut" 
             || ix.name == "swapBaseIn2"
             || ix.name == "swapBaseOut2"
            )
        })?; 
        
        // Extract event data from the instruction
        let (amount_in, amount_out, direction, pool_coin, pool_pc) = match &swap_ix.event {
            Some(DecodedEvent::SwapBaseInLog(data)) => Some((
                data.amount_in,
                data.out_amount,
                data.direction,
                data.pool_coin,
                data.pool_pc,
            )),
            Some(DecodedEvent::SwapBaseOutLog(data)) => Some((
                data.max_in,
                data.amount_out,
                data.direction,
                data.pool_coin,
                data.pool_pc,
            )),
            _ => None,
        }?;

        // Collect all token balances
        let all_balances: Vec<_> = pre_token_balances
            .iter()
            .chain(post_token_balances.iter())
            .cloned()
            .collect();

        // Get unique mints
        let unique_mints: Vec<String> = all_balances
            .iter()
            .map(|b| b.mint.clone())
            .collect::<std::collections::HashSet<_>>()
            .into_iter()
            .collect();

        if unique_mints.len() < 2 {
            return None;
        }

        // Determine quote mint (prioritize SOL, USDC, USDT)
        let quote_candidates = vec![SOL_MINT, USDC_MINT, USDT_MINT];
        let quote_mint = unique_mints
            .iter()
            .find(|m| quote_candidates.contains(&m.as_str()))
            .cloned()
            .unwrap_or(unique_mints[1].clone());

        // Base mint is the other one
        let base_mint = unique_mints
            .iter()
            .find(|m| *m != &quote_mint)
            .cloned()?;

        // Get balances for base and quote
        let base_balance = all_balances.iter().find(|b| b.mint == base_mint)?;
        let quote_balance = all_balances.iter().find(|b| b.mint == quote_mint)?;

        // Get decimals from the actual token balances
        let base_decimals = base_balance.ui_token_amount.decimals as i32;
        let quote_decimals = quote_balance.ui_token_amount.decimals as i32;

        // Calculate decimal amounts
        let amount_in_decimal = amount_in as f64 / 10f64.powi(base_decimals);
        let amount_out_decimal = amount_out as f64 / 10f64.powi(quote_decimals);

        if amount_out_decimal == 0.0 {
            return None;
        }

        // Calculate pool reserves in decimal
        let pool_base_decimal = pool_coin as f64 / 10f64.powi(base_decimals);
        let pool_quote_decimal = pool_pc as f64 / 10f64.powi(quote_decimals);

        // Calculate swap prices based on direction
        let (swap_price_quote_per_base, swap_price_base_per_quote) = if direction == 1 {
            // direction 1: swapping base -> quote (e.g., token -> SOL)
            let quote_per_base = amount_out_decimal / amount_in_decimal;  // SOL per token
            let base_per_quote = amount_in_decimal / amount_out_decimal;  // tokens per SOL
            (quote_per_base, base_per_quote)
        } else {
            // direction 2: swapping quote -> base (e.g., SOL -> token)
            let quote_per_base = amount_in_decimal / amount_out_decimal;  // SOL per token
            let base_per_quote = amount_out_decimal / amount_in_decimal;  // tokens per SOL
            (quote_per_base, base_per_quote)
        };

        // Calculate pool prices
        let pool_price_quote_per_base = if pool_base_decimal > 0.0 && pool_quote_decimal > 0.0 {
            pool_quote_decimal / pool_base_decimal  // SOL per token
        } else {
            0.0
        };
        
        let pool_price_base_per_quote = if pool_price_quote_per_base > 0.0 {
            pool_base_decimal / pool_quote_decimal  // tokens per SOL
        } else {
            0.0
        };

        // Format all prices to NEVER use scientific notation
        let base_token_price_str = Self::format_decimal(swap_price_quote_per_base);
        let quote_token_price_str = Self::format_decimal(swap_price_base_per_quote);
        let pool_base_token_price_str = Self::format_decimal(pool_price_quote_per_base);
        let pool_quote_token_price_str = Self::format_decimal(pool_price_base_per_quote);

        // Format pool price based on quote mint
        let pool_price = if quote_mint == SOL_MINT {
            format!("{} SOL", Self::format_small_price(pool_price_quote_per_base))
        } else if quote_mint == USDC_MINT || quote_mint == USDT_MINT {
            format!("${}", Self::format_small_price(pool_price_quote_per_base))
        } else {
            format!("{} {}", Self::format_small_price(pool_price_quote_per_base), Self::get_token_symbol(&quote_mint))
        };

        // Calculate price impact
        let price_impact = if pool_price_quote_per_base > 0.0 {
            let impact = ((swap_price_quote_per_base - pool_price_quote_per_base) / pool_price_quote_per_base) * 100.0;
            format!("{:.2}%", impact)
        } else {
            "N/A".to_string()
        };

        // Determine source and destination for formatted output
        let (source_mint, dest_mint, source_amount, dest_amount) = if direction == 1 {
            (&base_mint, &quote_mint, amount_in_decimal, amount_out_decimal)
        } else {
            (&quote_mint, &base_mint, amount_out_decimal, amount_in_decimal)
        };

        Some(RaydiumSwapParsed {
            base_mint: base_mint.clone(),
            quote_mint: quote_mint.clone(),
            base_decimals,
            quote_decimals,

            amount_in,
            amount_out,

            amount_in_formatted: format!(
                "{} {}",
                Self::format_number(source_amount),
                Self::get_token_symbol(source_mint)
            ),

            amount_out_formatted: format!(
                "{} {}",
                Self::format_number(dest_amount),
                Self::get_token_symbol(dest_mint)
            ),

            pool_base_reserves: pool_coin,
            pool_quote_reserves: pool_pc,

            pool_base_reserves_decimal: pool_base_decimal,
            pool_quote_reserves_decimal: pool_quote_decimal,

            // All price fields are now Strings with proper formatting
            base_token_price: base_token_price_str,
            quote_token_price: quote_token_price_str,
            pool_base_token_price: pool_base_token_price_str,
            pool_quote_token_price: pool_quote_token_price_str,

            swap_type: swap_ix.name.clone(),
            direction,

            pool_price,
            price_impact,
        })
    }
    
    // Format decimal numbers to NEVER use scientific notation
    fn format_decimal(num: f64) -> String {
        if num == 0.0 {
            return "0".to_string();
        }
        
        // Handle very small numbers with appropriate precision
        let abs_num = num.abs();
        
        if abs_num < 0.000000001 {
            // For extremely tiny numbers (like 6.33e-9)
            format!("{:.12}", num)
        } else if abs_num < 0.000001 {
            // For numbers like 0.00000633
            format!("{:.12}", num)
        } else if abs_num < 0.0001 {
            // For numbers like 0.000633
            format!("{:.10}", num)
        } else if abs_num < 0.01 {
            // For numbers like 0.00633
            format!("{:.8}", num)
        } else if abs_num < 1.0 {
            // For numbers like 0.633
            format!("{:.6}", num)
        } else if abs_num < 1000.0 {
            // For numbers like 633.45
            format!("{:.4}", num)
        } else if abs_num < 1000000.0 {
            // For numbers like 63345.67
            format!("{:.2}", num)
        } else {
            // For very large numbers
            format!("{:.2}", num)
        }
    }

    // Format small prices with appropriate decimal places (no scientific notation)
    fn format_small_price(price: f64) -> String {
        if price == 0.0 {
            return "0".to_string();
        }
        
        let abs_price = price.abs();
        
        if abs_price >= 1.0 {
            format!("{:.2}", price)
        } else if abs_price >= 0.01 {
            format!("{:.4}", price)
        } else if abs_price >= 0.0001 {
            format!("{:.6}", price)
        } else if abs_price >= 0.000001 {
            format!("{:.8}", price)
        } else if abs_price >= 0.00000001 {
            format!("{:.10}", price)
        } else {
            // For extremely small numbers, use fixed precision
            format!("{:.12}", price)
        }
    }

    // Format numbers with appropriate decimal places
    fn format_number(num: f64) -> String {
        if num == 0.0 {
            return "0".to_string();
        }
        
        let abs_num = num.abs();
        
        if abs_num >= 1000.0 {
            format!("{:.2}", num)
        } else if abs_num >= 1.0 {
            format!("{:.2}", num)
        } else if abs_num > 0.0 {
            // For numbers less than 1, show more decimals
            if abs_num < 0.000001 {
                format!("{:.12}", num)
            } else if abs_num < 0.001 {
                format!("{:.10}", num)
            } else {
                format!("{:.8}", num)
            }
        } else {
            "0".to_string()
        }
    }

    // Get token symbol for known mints
    fn get_token_symbol(mint: &str) -> String {
        match mint {
            "So11111111111111111111111111111111111111112" => "SOL".to_string(),
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v" => "USDC".to_string(),
            "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB" => "USDT".to_string(),
            "DezXAZ8z7PnrnRJjz3wXBoRgixCa6xjnB7YaB1pPB263" => "BONK".to_string(),
            "AZsHEMXd36Bj1EMNXhowJajpUXzrKcK57wW4ZGXVa7yR" => "GUAC".to_string(),
            _ => format!("{}...{}", &mint[..4], &mint[mint.len()-4..]),
        }
    }
}