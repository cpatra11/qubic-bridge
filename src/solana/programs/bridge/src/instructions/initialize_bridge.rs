use anchor_lang::prelude::*;
use crate::state::*;

pub fn handler(
    ctx: Context<InitializeBridge>,
    admin: Pubkey,
    validator_threshold: u8,
) -> Result<()> {
    let bridge_state = &mut ctx.accounts.bridge_state;
    
    bridge_state.admin = admin;
    bridge_state.is_paused = false;
    bridge_state.validator_threshold = validator_threshold;
    bridge_state.active_validators = 0;
    bridge_state.total_locked_tokens = 0;
    bridge_state.total_bridge_transactions = 0;
    bridge_state.next_bridge_id = 1;
    bridge_state.last_qubic_block = 0;
    bridge_state.emergency_pause_timestamp = 0;
    bridge_state.bump = ctx.bumps.bridge_state;
    
    Ok(())
}

#[derive(Accounts)]
pub struct InitializeBridge<'info> {
    #[account(
        init,
        payer = admin,
        space = 8 + BridgeState::INIT_SPACE,
        seeds = [b"bridge_state"],
        bump
    )]
    pub bridge_state: Account<'info, BridgeState>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
