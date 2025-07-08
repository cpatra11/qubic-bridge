use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(transaction_hash: [u8; 32])]
pub struct ValidateTransaction<'info> {
    #[account(mut)]
    pub validator: Signer<'info>,
    #[account(
        seeds = [b"validator", validator.key().as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,
    #[account(
        init_if_needed,
        payer = validator,
        space = 8 + 32 + 1,
        seeds = [b"validation", transaction_hash.as_ref()],
        bump
    )]
    pub validation_state: Account<'info, ValidationState>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<ValidateTransaction>,
    transaction_hash: [u8; 32],
    is_valid: bool,
) -> Result<()> {
    let validation_state = &mut ctx.accounts.validation_state;
    validation_state.transaction_hash = transaction_hash;
    validation_state.is_valid = is_valid;
    
    Ok(())
}
