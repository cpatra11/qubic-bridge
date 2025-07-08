use anchor_lang::prelude::*;
use crate::state::*;

#[derive(Accounts)]
#[instruction(validator_pubkey: Pubkey)]
pub struct RegisterValidator<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,
    #[account(
        init,
        payer = authority,
        space = 8 + 32 + 32 + 1,
        seeds = [b"validator", validator_pubkey.as_ref()],
        bump
    )]
    pub validator_state: Account<'info, ValidatorState>,
    pub system_program: Program<'info, System>,
}

pub fn handler(
    ctx: Context<RegisterValidator>,
    validator_pubkey: Pubkey,
    qubic_validator_id: [u8; 32],
) -> Result<()> {
    let validator_state = &mut ctx.accounts.validator_state;
    validator_state.validator_pubkey = validator_pubkey;
    validator_state.qubic_validator_id = qubic_validator_id;
    validator_state.is_active = true;
    
    Ok(())
}
