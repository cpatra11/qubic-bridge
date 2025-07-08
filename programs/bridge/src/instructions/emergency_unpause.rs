use anchor_lang::prelude::*;
use crate::state::*;

pub fn handler(ctx: Context<EmergencyUnpause>) -> Result<()> {
    let bridge_state = &mut ctx.accounts.bridge_state;
    
    bridge_state.is_paused = false;
    bridge_state.emergency_pause_timestamp = 0;
    
    Ok(())
}

#[derive(Accounts)]
pub struct EmergencyUnpause<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump = bridge_state.bump,
        has_one = admin
    )]
    pub bridge_state: Account<'info, BridgeState>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
}
