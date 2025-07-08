use anchor_lang::prelude::*;

mod instructions;
mod state;
mod error;

use crate::instructions::*;

declare_id!("4LXpG5445Kmv879vWzCFx5bTc7miCJAY96iQDMJB1zaP");

#[program]
pub mod validator {
    use super::*;

    pub fn register_validator(
        ctx: Context<RegisterValidator>,
        validator_pubkey: Pubkey,
        qubic_validator_id: [u8; 32],
    ) -> Result<()> {
        instructions::register_validator::handler(ctx, validator_pubkey, qubic_validator_id)
    }

    pub fn update_validator_status(
        ctx: Context<UpdateValidatorStatus>,
        validator_pubkey: Pubkey,
        is_active: bool,
    ) -> Result<()> {
        instructions::update_validator_status::handler(ctx, validator_pubkey, is_active)
    }

    pub fn validate_transaction(
        ctx: Context<ValidateTransaction>,
        transaction_hash: [u8; 32],
        is_valid: bool,
    ) -> Result<()> {
        instructions::validate_transaction::handler(ctx, transaction_hash, is_valid)
    }
}