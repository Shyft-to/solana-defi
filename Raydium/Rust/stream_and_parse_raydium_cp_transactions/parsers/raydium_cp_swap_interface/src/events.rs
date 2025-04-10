use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::pubkey::Pubkey;
pub const LP_CHANGE_EVENT_EVENT_DISCM: [u8; 8] = [121, 163, 205, 201, 57, 218, 117, 60];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct LpChangeEvent {
    pool_id: Pubkey,
    lp_amount_before: u64,
    token0_vault_before: u64,
    token1_vault_before: u64,
    token0_amount: u64,
    token1_amount: u64,
    token0_transfer_fee: u64,
    token1_transfer_fee: u64,
    change_type: u8,
}
#[derive(Clone, Debug, PartialEq)]
pub struct LpChangeEventEvent(pub LpChangeEvent);
impl BorshSerialize for LpChangeEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        LP_CHANGE_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl LpChangeEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != LP_CHANGE_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        LP_CHANGE_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(LpChangeEvent::deserialize(buf)?))
    }
}
pub const SWAP_EVENT_EVENT_DISCM: [u8; 8] = [64, 198, 205, 232, 38, 8, 113, 226];
#[derive(Clone, Debug, PartialEq, BorshDeserialize, BorshSerialize)]
pub struct SwapEvent {
    pool_id: Pubkey,
    input_vault_before: u64,
    output_vault_before: u64,
    input_amount: u64,
    output_amount: u64,
    input_transfer_fee: u64,
    output_transfer_fee: u64,
    base_input: bool,
}
#[derive(Clone, Debug, PartialEq)]
pub struct SwapEventEvent(pub SwapEvent);
impl BorshSerialize for SwapEventEvent {
    fn serialize<W: std::io::Write>(&self, writer: &mut W) -> std::io::Result<()> {
        SWAP_EVENT_EVENT_DISCM.serialize(writer)?;
        self.0.serialize(writer)
    }
}
impl SwapEventEvent {
    pub fn deserialize(buf: &mut &[u8]) -> std::io::Result<Self> {
        let maybe_discm = <[u8; 8]>::deserialize(buf)?;
        if maybe_discm != SWAP_EVENT_EVENT_DISCM {
            return Err(
                std::io::Error::new(
                    std::io::ErrorKind::Other,
                    format!(
                        "discm does not match. Expected: {:?}. Received: {:?}",
                        SWAP_EVENT_EVENT_DISCM, maybe_discm
                    ),
                ),
            );
        }
        Ok(Self(SwapEvent::deserialize(buf)?))
    }
}
