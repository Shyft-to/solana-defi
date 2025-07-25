
use solana_sdk::{instruction::AccountMeta, message::v0::LoadedAddresses, message::VersionedMessage};

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
