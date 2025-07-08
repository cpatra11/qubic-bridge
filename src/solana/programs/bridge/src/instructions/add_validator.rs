use anchor_lang::prelude::*;
use crate::state::*;

pub fn handler(
    ctx: Context<AddValidator>,
    validator_pubkey: Pubkey,
    qubic_validator_id: [u8; 32],
) -> Result<()> {
    let bridge_state = &mut ctx.accounts.bridge_state;
    let validator_info = &mut ctx.accounts.validator_info;
    
    validator_info.pubkey = validator_pubkey;
    // Convert [u8; 32] to String (hex representation)
    validator_info.qubic_validator_id = hex::encode(&qubic_validator_id);
    validator_info.is_active = true;
    validator_info.stake = 0;
    validator_info.last_activity = Clock::get()?.unix_timestamp;
    validator_info.bump = ctx.bumps.validator_info;
    
    bridge_state.active_validators += 1;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(validator_pubkey: Pubkey)]
pub struct AddValidator<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump = bridge_state.bump,
        has_one = admin
    )]
    pub bridge_state: Account<'info, BridgeState>,
    
    #[account(
        init,
        payer = admin,
        space = 8 + ValidatorInfo::INIT_SPACE,
        seeds = [b"validator", validator_pubkey.as_ref()],
        bump
    )]
    pub validator_info: Account<'info, ValidatorInfo>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
