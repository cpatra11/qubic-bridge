// filepath: /solana-qubic-bridge/solana-qubic-bridge/src/solana/programs/bridge/src/instructions/validate_bridge.rs
use anchor_lang::prelude::*;
use crate::state::{BridgeState, BridgeTransaction, ValidatorInfo};
use crate::error::BridgeError;

pub fn handler(
    ctx: Context<ValidateBridge>,
    bridge_id: u64,
    qubic_tx_hash: [u8; 32],
) -> Result<()> {
    let bridge_state = &mut ctx.accounts.bridge_state;
    let bridge_transaction = &mut ctx.accounts.bridge_transaction;
    let validator_info = &ctx.accounts.validator_info;

    // Check if bridge is paused
    require!(!bridge_state.is_paused, BridgeError::BridgePaused);
    
    // Validate validator
    require!(validator_info.is_active, BridgeError::UnauthorizedValidator);
    
    // Validate transaction exists and is pending
    require!(
        bridge_transaction.status == 0, // BRIDGE_STATUS_PENDING
        BridgeError::InvalidBridgeTransaction
    );

    // Update the bridge transaction with Qubic tx hash
    bridge_transaction.qubic_tx_hash = qubic_tx_hash;
    bridge_transaction.confirmations += 1;
    bridge_transaction.updated_at = Clock::get()?.unix_timestamp;

    // Update validator's last activity
    let validator_info = &mut ctx.accounts.validator_info;
    validator_info.last_activity = Clock::get()?.unix_timestamp;

    // Update bridge state
    bridge_state.last_qubic_block += 1;

    msg!(
        "Bridge transaction {} validated by validator {}",
        bridge_id,
        validator_info.pubkey
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(bridge_id: u64)]
pub struct ValidateBridge<'info> {
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
        mut,
        seeds = [b"validator", validator.key().as_ref()],
        bump = validator_info.bump
    )]
    pub validator_info: Account<'info, ValidatorInfo>,
    
    pub validator: Signer<'info>,
}