use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(_validator_pubkey: Pubkey)]
pub struct UpdateValidatorStatus<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        mut,
        seeds = [b"validator", _validator_pubkey.as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,
}

pub fn handler(
    ctx: Context<UpdateValidatorStatus>,
    _validator_pubkey: Pubkey,
    is_active: bool,
) -> Result<()> {
    let validator_state = &mut ctx.accounts.validator_state;
    validator_state.is_active = is_active;
    
    Ok(())
}
