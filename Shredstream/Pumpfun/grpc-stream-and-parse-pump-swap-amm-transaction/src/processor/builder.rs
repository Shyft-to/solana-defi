
use solana_sdk::{instruction::{AccountMeta, CompiledInstruction, Instruction}, pubkey::Pubkey};

pub fn compiled_instruction_to_instruction(
    ci: &CompiledInstruction,
    parsed_accounts: Vec<AccountMeta>,
) -> Instruction {
    let program_id = if (ci.program_id_index as usize) < parsed_accounts.len() {
        parsed_accounts[ci.program_id_index as usize].pubkey
    } else {
        log::error!(
            "⚠️ Program ID index {} out of bounds (parsed_accounts.len() = {})",
            ci.program_id_index,
            parsed_accounts.len()
        );
        Pubkey::default()
    };

    let accounts: Vec<AccountMeta> = ci
        .accounts
        .iter()
        .filter_map(|&index| {
            if (index as usize) < parsed_accounts.len() {
                Some(parsed_accounts[index as usize].clone())
            } else {
                None
            }
        })
        .collect();

    Instruction {
        program_id,
        accounts,
        data: ci.data.clone(),
    }
}