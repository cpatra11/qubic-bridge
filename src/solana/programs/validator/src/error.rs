// src/solana/programs/validator/src/error.rs
use solana_program::program_error::ProgramError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ValidatorError {
    #[error("Invalid instruction")]
    InvalidInstruction,
    #[error("Invalid account")]
    InvalidAccount,
    #[error("Invalid argument")]
    InvalidArgument,
    #[error("Insufficient funds")]
    InsufficientFunds,
    #[error("Unauthorized")]
    Unauthorized,
}

impl From<ValidatorError> for ProgramError {
    fn from(e: ValidatorError) -> Self {
        match e {
            ValidatorError::InvalidInstruction => ProgramError::InvalidInstructionData,
            ValidatorError::InvalidAccount => ProgramError::InvalidAccountData,
            ValidatorError::InvalidArgument => ProgramError::InvalidArgument,
            ValidatorError::InsufficientFunds => ProgramError::InsufficientFunds,
            ValidatorError::Unauthorized => ProgramError::MissingRequiredSignature,
        }
    }
}
