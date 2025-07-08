// filepath: /solana-qubic-bridge/solana-qubic-bridge/src/solana/programs/validator/src/lib.rs
use solana_program::{account_info::AccountInfo, entrypoint::ProgramResult, pubkey::Pubkey};

pub mod error;
pub mod instructions;
pub mod state;

#[cfg(not(feature = "no-entrypoint"))]
pub mod entrypoint {
    use super::*;

    pub fn process_instruction(
        program_id: &Pubkey,
        accounts: &[AccountInfo],
        instruction_data: &[u8],
    ) -> ProgramResult {
        instructions::process_instruction(program_id, accounts, instruction_data)
    }
}