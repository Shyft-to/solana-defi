
use solana_program::{pubkey::Pubkey, program_option::COption};
use spl_token::instruction::{AuthorityType, TokenInstruction};
use serde::{Serialize, Deserialize};
use serde_with::{serde_as, DisplayFromStr};

// Helper function to convert COption<Pubkey> to Option<Pubkey>
fn convert_coption<T>(coption: COption<T>) -> Option<T> {
    match coption {
        COption::Some(value) => Some(value),
        COption::None => None,
    }
}

fn convert_set_authority(
    authority_type: AuthorityType,
    new_authority: COption<Pubkey>,
) -> SerializableSetAuthority {
    SerializableSetAuthority {
        authority_type: match authority_type {
            AuthorityType::MintTokens => "MintTokens".to_string(),
            AuthorityType::FreezeAccount => "FreezeAccount".to_string(),
            AuthorityType::AccountOwner => "AccountOwner".to_string(),
            AuthorityType::CloseAccount => "CloseAccount".to_string(),
        },
        new_authority: new_authority.into(), // Convert COption<Pubkey> to Option<Pubkey>
    }
}


#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct SerializableInitializeMint {
    decimals: u8,
    #[serde_as(as = "DisplayFromStr")]
    mint_authority: Pubkey,
    #[serde_as(as = "Option<DisplayFromStr>")]
    freeze_authority: Option<Pubkey>,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct SerializableInitializeAccount3 {
    #[serde_as(as = "DisplayFromStr")]
    owner: Pubkey,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct SerializableSetAuthority {
    authority_type: String,
    #[serde_as(as = "Option<DisplayFromStr>")]
    new_authority: Option<Pubkey>,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct SerializableTransfer {
    amount: u64,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub enum SerializableTokenInstruction {
    InitializeMint(SerializableInitializeMint),
    InitializeAccount,
    InitializeMultisig { m: u8 },
    Transfer(SerializableTransfer),
    Approve { amount: u64 },
    Revoke,
    SetAuthority(SerializableSetAuthority),
    MintTo { amount: u64 },
    Burn { amount: u64 },
    CloseAccount,
    FreezeAccount,
    ThawAccount,
    TransferChecked { amount: u64, decimals: u8 },
    ApproveChecked { amount: u64, decimals: u8 },
    MintToChecked { amount: u64, decimals: u8 },
    BurnChecked { amount: u64, decimals: u8 },
    InitializeAccount2 { #[serde_as(as = "DisplayFromStr")] owner: Pubkey },
    SyncNative,
    InitializeAccount3(SerializableInitializeAccount3),
    InitializeMultisig2 { m: u8 },
    InitializeMint2(SerializableInitializeMint),
    GetAccountDataSize,
    InitializeImmutableOwner,
    AmountToUiAmount { amount: u64 },
    UiAmountToAmount { ui_amount: String },
}


// Convert TokenInstruction to SerializableTokenInstruction
pub fn convert_to_serializable(ix: TokenInstruction) -> SerializableTokenInstruction {
    match ix {
        TokenInstruction::InitializeMint { decimals, mint_authority, freeze_authority } => {
            SerializableTokenInstruction::InitializeMint(SerializableInitializeMint {
                decimals,
                mint_authority,
                freeze_authority: freeze_authority.into(), // Convert COption to Option
            })
        }
        TokenInstruction::InitializeAccount => SerializableTokenInstruction::InitializeAccount,
        TokenInstruction::InitializeMultisig { m } => SerializableTokenInstruction::InitializeMultisig { m },
        TokenInstruction::Transfer { amount } => SerializableTokenInstruction::Transfer(SerializableTransfer { amount }),
        TokenInstruction::Approve { amount } => SerializableTokenInstruction::Approve { amount },
        TokenInstruction::Revoke => SerializableTokenInstruction::Revoke,
        TokenInstruction::SetAuthority { authority_type, new_authority } => {
            SerializableTokenInstruction::SetAuthority(convert_set_authority(authority_type, new_authority))
        }
        TokenInstruction::MintTo { amount } => SerializableTokenInstruction::MintTo { amount },
        TokenInstruction::Burn { amount } => SerializableTokenInstruction::Burn { amount },
        TokenInstruction::CloseAccount => SerializableTokenInstruction::CloseAccount,
        TokenInstruction::FreezeAccount => SerializableTokenInstruction::FreezeAccount,
        TokenInstruction::ThawAccount => SerializableTokenInstruction::ThawAccount,
        TokenInstruction::TransferChecked { amount, decimals } => {
            SerializableTokenInstruction::TransferChecked { amount, decimals }
        }
        TokenInstruction::ApproveChecked { amount, decimals } => {
            SerializableTokenInstruction::ApproveChecked { amount, decimals }
        }
        TokenInstruction::MintToChecked { amount, decimals } => {
            SerializableTokenInstruction::MintToChecked { amount, decimals }
        }
        TokenInstruction::BurnChecked { amount, decimals } => {
            SerializableTokenInstruction::BurnChecked { amount, decimals }
        }
        TokenInstruction::InitializeAccount2 { owner } => {
            SerializableTokenInstruction::InitializeAccount2 { owner }
        }
        TokenInstruction::SyncNative => SerializableTokenInstruction::SyncNative,
        TokenInstruction::InitializeAccount3 { owner } => {
            SerializableTokenInstruction::InitializeAccount3(SerializableInitializeAccount3 { owner })
        }
        TokenInstruction::InitializeMultisig2 { m } => {
            SerializableTokenInstruction::InitializeMultisig2 { m }
        }
        TokenInstruction::InitializeMint2 { decimals, mint_authority, freeze_authority } => {
            SerializableTokenInstruction::InitializeMint2(SerializableInitializeMint {
                decimals,
                mint_authority,
                freeze_authority: freeze_authority.into(),
            })
        }
        TokenInstruction::GetAccountDataSize => SerializableTokenInstruction::GetAccountDataSize,
        TokenInstruction::InitializeImmutableOwner => SerializableTokenInstruction::InitializeImmutableOwner,
        TokenInstruction::AmountToUiAmount { amount } => SerializableTokenInstruction::AmountToUiAmount { amount },
        TokenInstruction::UiAmountToAmount { ui_amount } => {
            SerializableTokenInstruction::UiAmountToAmount { ui_amount: ui_amount.to_string() }
        }
    }
}