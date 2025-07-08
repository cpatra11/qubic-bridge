use anchor_lang::prelude::*;
use crate::state::{BridgeState, BridgeTransaction};

pub fn handler(ctx: Context<GetBridgeInfo>) -> Result<BridgeInfoResponse> {
    let bridge_state = &ctx.accounts.bridge_state;

    Ok(BridgeInfoResponse {
        bridge_id: bridge_state.bridge_id,
        min_lock_amount: bridge_state.min_lock_amount,
        max_lock_amount: bridge_state.max_lock_amount,
        bridge_fee: bridge_state.bridge_fee,
        is_active: bridge_state.is_active,
        validator_count: bridge_state.active_validators as u64,
        required_signatures: bridge_state.validator_threshold as u64,
        total_locked: bridge_state.total_locked_tokens,
        total_unlocked: bridge_state.total_unlocked_tokens,
        total_transfers: bridge_state.total_bridge_transactions,
        total_validator_actions: bridge_state.total_validator_actions,
    })
}

pub fn get_lock_info_handler(ctx: Context<GetLockInfo>) -> Result<LockInfoResponse> {
    let bridge_transaction = &ctx.accounts.bridge_transaction;

    Ok(LockInfoResponse {
        locker: bridge_transaction.user,
        amount: bridge_transaction.amount,
        actual_amount: bridge_transaction.actual_amount,
        fee: bridge_transaction.fee,
        timestamp: bridge_transaction.created_at,
        qubic_destination: bridge_transaction.qubic_destination,
        memo: bridge_transaction.memo,
        is_processed: bridge_transaction.is_processed,
        status: bridge_transaction.status,
    })
}

#[derive(Accounts)]
pub struct GetBridgeInfo<'info> {
    #[account(
        seeds = [b"bridge_state"],
        bump = bridge_state.bump
    )]
    pub bridge_state: Account<'info, BridgeState>,
}

#[derive(Accounts)]
#[instruction(lock_id: u64)]
pub struct GetLockInfo<'info> {
    #[account(
        seeds = [b"bridge_tx", lock_id.to_le_bytes().as_ref()],
        bump = bridge_transaction.bump
    )]
    pub bridge_transaction: Account<'info, BridgeTransaction>,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct BridgeInfoResponse {
    pub bridge_id: u64,
    pub min_lock_amount: u64,
    pub max_lock_amount: u64,
    pub bridge_fee: u64,
    pub is_active: bool,
    pub validator_count: u64,
    pub required_signatures: u64,
    pub total_locked: u64,
    pub total_unlocked: u64,
    pub total_transfers: u64,
    pub total_validator_actions: u64,
}

#[derive(AnchorSerialize, AnchorDeserialize)]
pub struct LockInfoResponse {
    pub locker: Pubkey,
    pub amount: u64,
    pub actual_amount: u64,
    pub fee: u64,
    pub timestamp: i64,
    pub qubic_destination: [u8; 32],
    pub memo: [u8; 64],
    pub is_processed: bool,
    pub status: u8,
}
