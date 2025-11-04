use super::types::*;

use crate::processor::models::mapper::{
    instruction::InstructionAccountMapper,
    event::{DecodedEvent},
};
use crate::processor::models::serialize::token_serializable::convert_to_serializable;
use pumpfun_amm_interface::instructions::PumpfunAmmProgramIx;
use spl_token::instruction::TokenInstruction;
use crate::TransactionProcessor;

impl TransactionProcessor{
    pub fn decode_instructions(
        &self,
        compiled_instructions: &[TransactionInstructionWithParent],
        inner_instructions: &[TransactionInstructionWithParent],
        decoded_event: &Option<DecodedEvent>,
    ) -> anyhow::Result<(Vec<DecodedInstruction>, Vec<DecodedInstruction>)> {
        let mut decoded_compiled = Vec::new();
        let mut decoded_inner = Vec::new();

        for instruction in compiled_instructions {
            if let Some(decoded) = self.decode_single_instruction(instruction, decoded_event)? {
                decoded_compiled.push(decoded);
            }
        }

        for instruction in inner_instructions {
            if let Some(decoded) = self.decode_single_instruction(instruction, decoded_event)? {
                decoded_inner.push(decoded);
            }
        }

        Ok((decoded_compiled, decoded_inner))
    }

    pub fn decode_single_instruction(
        &self,
        instruction: &TransactionInstructionWithParent,
        decoded_event: &Option<DecodedEvent>,
    ) -> anyhow::Result<Option<DecodedInstruction>> {
        if instruction.instruction.program_id == self.pumpfun_program_id {
            self.decode_pumpfun_instruction(instruction, decoded_event)
        } else if instruction.instruction.program_id == self.token_program_id {
            self.decode_token_instruction(instruction)
        } else {
            Ok(None)
        }
    }

    pub fn decode_pumpfun_instruction(
        &self,
        instruction: &TransactionInstructionWithParent,
        decoded_event: &Option<DecodedEvent>,
    ) -> anyhow::Result<Option<DecodedInstruction>> {
        match PumpfunAmmProgramIx::deserialize(&instruction.instruction.data) {
            Ok(decoded_ix) => {
                let mapped_accounts = self.pumpfun_idl.map_accounts(
                    &instruction.instruction.accounts,
                    &decoded_ix.name().to_string(),
                )?;

                let data = serde_json::to_value(&decoded_ix)
                    .map_err(|e| anyhow::anyhow!("Failed to serialize ix data: {:?}", e))?;

                Ok(Some(DecodedInstruction {
                    name: decoded_ix.name().to_string(),
                    accounts: mapped_accounts,
                    data,
                    event: decoded_event.clone(),
                    program_id: instruction.instruction.program_id,
                    parent_program_id: instruction.parent_program_id,
                }))
            }
            Err(e) => {
                Ok(None)
            }
        }
    }

    pub fn decode_token_instruction(
        &self,
        instruction: &TransactionInstructionWithParent,
    ) -> anyhow::Result<Option<DecodedInstruction>> {
        match TokenInstruction::unpack(&instruction.instruction.data) {
            Ok(decoded_ix) => {
                let ix_name = self.get_instruction_name_with_typename(&decoded_ix);
                let serializable_ix = convert_to_serializable(decoded_ix);

                let mapped_accounts = self.token_idl.map_accounts(
                    &instruction.instruction.accounts,
                    &ix_name,
                )?;

                let data = serde_json::to_value(serializable_ix)
                    .map_err(|e| anyhow::anyhow!("Failed to serialize token ix data: {:?}", e))?;

                Ok(Some(DecodedInstruction {
                    name: ix_name,
                    accounts: mapped_accounts,
                    data,
                    event: None,
                    program_id: instruction.instruction.program_id,
                    parent_program_id: instruction.parent_program_id,
                }))
            }
            Err(_) => {
                Ok(None)
            }
        }
    }
}