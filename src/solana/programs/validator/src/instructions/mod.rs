// src/solana/programs/validator/src/instructions/mod.rs
pub mod validate;

use crate::error::ValidatorError;
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey, program_error::ProgramError};

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // For now, we'll just process all instructions as validate instructions
    // In a real implementation, you'd parse the instruction_data to determine the instruction type
    process_validate_instruction(program_id, accounts, instruction_data)
}

pub fn process_validate_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    // Logic for processing the validate instruction will be implemented in validate.rs
    validate::process_validate(program_id, accounts, instruction_data)
}