use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;

pub const GLOBAL_CONFIG_ACCOUNT_DISCM: [u8; 8] = [149, 8, 156, 202, 160, 252, 176, 217];
pub const POOL_ACCOUNT_DISCM: [u8; 8] = [241, 154, 109, 4, 17, 177, 109, 188];

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct GlobalConfig {
    pub admin: Pubkey,

    pub lp_fee_basis_points: u64,

    pub protocol_fee_basis_points: u64,
    pub disable_flags: u8,
    pub protocol_fee_recipients: [Pubkey; 8],
}

#[derive(Clone, Debug, PartialEq)]
pub struct GlobalConfigAccount(pub GlobalConfig);

impl GlobalConfigAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != GLOBAL_CONFIG_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        GLOBAL_CONFIG_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(GlobalConfig::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&GLOBAL_CONFIG_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}

#[derive(Clone, Debug, BorshDeserialize, BorshSerialize, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pool {
    pub pool_bump: u8,

    pub index: u16,
    pub creator: Pubkey,
    pub base_mint: Pubkey,
    pub quote_mint: Pubkey,

    pub lp_mint: Pubkey,

    pub pool_base_token_account: Pubkey,

    pub pool_quote_token_account: Pubkey,

    pub lp_supply: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PoolAccount(pub Pool);

impl PoolAccount {
    pub fn deserialize(buf: &[u8]) -> std::io::Result<Self> {
        use std::io::Read;
        let mut reader = buf;
        let mut maybe_discm = [0u8; 8];
        reader.read_exact(&mut maybe_discm)?;
        if maybe_discm != POOL_ACCOUNT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        POOL_ACCOUNT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(Pool::deserialize(&mut reader)?))
    }

    pub fn serialize<W: std::io::Write>(&self, mut writer: W) -> std::io::Result<()> {
        writer.write_all(&POOL_ACCOUNT_DISCM)?;
        self.0.serialize(&mut writer)
    }

    pub fn try_to_vec(&self) -> std::io::Result<Vec<u8>> {
        let mut data = Vec::new();
        self.serialize(&mut data)?;
        Ok(data)
    }
}