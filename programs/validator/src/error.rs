use anchor_lang::prelude::*;

#[error_code]
pub enum ValidatorError {
    #[msg("Invalid instruction")]
    InvalidInstruction,
    #[msg("Invalid account")]
    InvalidAccount,
    #[msg("Invalid argument")]
    InvalidArgument,
    #[msg("Insufficient funds")]
    InsufficientFunds,
    #[msg("Unauthorized")]
    Unauthorized,
    #[msg("Validator not found")]
    ValidatorNotFound,
    #[msg("Validator already exists")]
    ValidatorAlreadyExists,
    #[msg("Invalid validator state")]
    InvalidValidatorState,
}
