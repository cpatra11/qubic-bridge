use anchor_lang::prelude::*;
use crate::state::BridgeState;
use crate::error::BridgeError;

pub fn handler(
    ctx: Context<UpdateConfig>,
    new_min_lock_amount: u64,
    new_max_lock_amount: u64,
    new_bridge_fee: u64,
    new_required_signatures: u8,
    new_is_active: bool,
) -> Result<()> {
    let bridge_state = &mut ctx.accounts.bridge_state;

    // Only admin can update configuration (matching Qubic contract admin check)
    require!(
        ctx.accounts.admin.key() == bridge_state.admin,
        BridgeError::UnauthorizedAdmin
    );

    // Validate configuration parameters
    require!(new_min_lock_amount > 0, BridgeError::InvalidAmount);
    require!(new_max_lock_amount >= new_min_lock_amount, BridgeError::InvalidAmount);
    require!(new_bridge_fee <= 1000, BridgeError::InvalidFee); // Max 10%
    require!(new_required_signatures > 0, BridgeError::InvalidValidatorThreshold);

    // Update configuration (matching Qubic UpdateConfig procedure)
    bridge_state.min_lock_amount = new_min_lock_amount;
    bridge_state.max_lock_amount = new_max_lock_amount;
    bridge_state.bridge_fee = new_bridge_fee;
    bridge_state.validator_threshold = new_required_signatures;
    bridge_state.is_active = new_is_active;

    msg!(
        "Bridge config updated: min={}, max={}, fee={}, threshold={}, active={}",
        new_min_lock_amount,
        new_max_lock_amount,
        new_bridge_fee,
        new_required_signatures,
        new_is_active
    );

    Ok(())
}

#[derive(Accounts)]
pub struct UpdateConfig<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump = bridge_state.bump
    )]
    pub bridge_state: Account<'info, BridgeState>,
    
    #[account(mut)]
    pub admin: Signer<'info>,
    
    pub system_program: Program<'info, System>,
}
