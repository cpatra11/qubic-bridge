use anchor_lang::prelude::*;
use crate::state::*;

pub fn handler(
    ctx: Context<UpdateBridgeState>,
    _bridge_id: u64,
    status: u8,
    confirmations: u64,
) -> Result<()> {
    let bridge_transaction = &mut ctx.accounts.bridge_transaction;
    let validator_info = &ctx.accounts.validator_info;
    
    // Only allow active validators to update bridge state
    require!(validator_info.is_active, crate::error::BridgeError::UnauthorizedValidator);
    
    bridge_transaction.status = status;
    bridge_transaction.confirmations = confirmations;
    bridge_transaction.updated_at = Clock::get()?.unix_timestamp;
    
    Ok(())
}

#[derive(Accounts)]
#[instruction(bridge_id: u64)]
pub struct UpdateBridgeState<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump = bridge_state.bump
    )]
    pub bridge_state: Account<'info, BridgeState>,
    
    #[account(
        mut,
        seeds = [b"bridge_tx", bridge_id.to_le_bytes().as_ref()],
        bump = bridge_transaction.bump
    )]
    pub bridge_transaction: Account<'info, BridgeTransaction>,
    
    #[account(
        seeds = [b"validator", validator.key().as_ref()],
        bump = validator_info.bump
    )]
    pub validator_info: Account<'info, ValidatorInfo>,
    
    pub validator: Signer<'info>,
}
