use solana_sdk::pubkey::Pubkey;

pub fn serialize_pubkey<S>(value: &Pubkey, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&value.to_string())
}

pub fn serialize_option_pubkey<S>(value: &Option<Pubkey>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match value {
        Some(pubkey) => serializer.serialize_str(&pubkey.to_string()),
        None => serializer.serialize_none(),
    }
}
