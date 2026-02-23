use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

pub const PUMP_FUN_PROGRAM: &str = "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P";

pub mod seeds {
    pub const GLOBAL_SEED: &[u8] = b"global";

    pub const MINT_AUTHORITY_SEED: &[u8] = b"mint-authority";

    pub const BONDING_CURVE_SEED: &[u8] = b"bonding-curve";

    pub const ASSOCIARTED_BONDING_CURVE_SEED: &[u8] = b"associated-bonding-curve";

    pub const METADATA_SEED: &[u8] = b"metadata";

    pub const CREATOR_VAULT_SEED: &[u8] = b"creator-vault";
}

pub mod accounts {
    use solana_sdk::{pubkey, pubkey::Pubkey};

    pub const PUMPFUN: Pubkey = pubkey!("6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P");

    pub const MPL_TOKEN_METADATA: Pubkey = pubkey!("metaqbxxUerdq28cj1RbAWkYQm3ybzjb6a8bt518x1s");

    pub const EVENT_AUTHORITY: Pubkey = pubkey!("Ce6TQqeHC9p8KetsN6JsjHK7UTZk7nasjjnr7XxXp9F1");

    pub const SYSTEM_PROGRAM: Pubkey = pubkey!("11111111111111111111111111111111");

    pub const TOKEN_PROGRAM: Pubkey = pubkey!("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA");

    pub const ASSOCIATED_TOKEN_PROGRAM: Pubkey =
        pubkey!("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL");

    pub const RENT: Pubkey = pubkey!("SysvarRent111111111111111111111111111111111");
}
pub fn get_program_id() -> Pubkey {
    Pubkey::from_str(PUMP_FUN_PROGRAM).unwrap()
}
