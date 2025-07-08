use anchor_lang::prelude::*;
use crate::state::*;

pub fn handler(
    ctx: Context<RemoveValidator>,
    _validator_pubkey: Pubkey,
) -> Result<()> {
    let bridge_state = &mut ctx.accounts.bridge_state;
    let validator_info = &mut ctx.accounts.validator_info;
    
    validator_info.is_active = false;
    bridge_state.active_validators -= 1;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(validator_pubkey: Pubkey)]
pub struct RemoveValidator<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump = bridge_state.bump,
        has_one = admin
    )]
    pub bridge_state: Account<'info, BridgeState>,
    
    #[account(
        mut,
        seeds = [b"validator", validator_pubkey.as_ref()],
        bump = validator_info.bump
    )]
    pub validator_info: Account<'info, ValidatorInfo>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
}
