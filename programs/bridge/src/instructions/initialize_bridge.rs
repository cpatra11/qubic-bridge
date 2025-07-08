use anchor_lang::prelude::*;
use crate::state::*;

pub fn handler(
    ctx: Context<InitializeBridge>,
    admin: Pubkey,
    validator_threshold: u8,
) -> Result<()> {
    let bridge_state = &mut ctx.accounts.bridge_state;
    
    // Initialize admin and pause state
    bridge_state.admin = admin;
    bridge_state.is_paused = false;
    
    // Initialize bridge configuration (matching Qubic contract defaults)
    bridge_state.bridge_id = 1; // Solana-Qubic Bridge ID
    bridge_state.min_lock_amount = 1000000;      // 1M tokens minimum (adjust for token decimals)
    bridge_state.max_lock_amount = 1000000000000; // 1T tokens maximum
    bridge_state.bridge_fee = 100;               // 1% fee (100 basis points)
    bridge_state.is_active = true;
    
    // Initialize validator management
    bridge_state.validator_threshold = validator_threshold;
    bridge_state.active_validators = 0;
    
    // Initialize chain configuration
    bridge_state.qubic_chain_id = 1; // Qubic mainnet
    
    // Initialize statistics
    bridge_state.total_locked_tokens = 0;
    bridge_state.total_unlocked_tokens = 0;
    bridge_state.total_bridge_transactions = 0;
    bridge_state.total_validator_actions = 0;
    bridge_state.next_lock_id = 1;
    
    bridge_state.last_qubic_block = 0;
    bridge_state.emergency_pause_timestamp = 0;
    bridge_state.bump = ctx.bumps.bridge_state;
    
    msg!(
        "Bridge initialized: admin={}, threshold={}, min_amount={}, max_amount={}, fee={}",
        admin,
        validator_threshold,
        bridge_state.min_lock_amount,
        bridge_state.max_lock_amount,
        bridge_state.bridge_fee
    );
    
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
