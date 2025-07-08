use anchor_lang::prelude::*;
use crate::state::*;

pub fn handler(ctx: Context<EmergencyPause>) -> Result<()> {
    let bridge_state = &mut ctx.accounts.bridge_state;
    
    bridge_state.is_paused = true;
    bridge_state.emergency_pause_timestamp = Clock::get()?.unix_timestamp;
    
    Ok(())
}

#[derive(Accounts)]
pub struct EmergencyPause<'info> {
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
