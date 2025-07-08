use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};
use crate::state::{BridgeState, BridgeTransaction, ValidatorInfo, BRIDGE_STATUS_COMPLETED};
use crate::error::BridgeError;

pub fn handler(
    ctx: Context<UnlockAssets>,
    lock_id: u64,
    recipient: Pubkey,
    amount: u64,
    qubic_signatures: Vec<[u8; 64]>,
) -> Result<()> {
    // Store account info and bump before mutable borrow
    let bridge_account_info = ctx.accounts.bridge_state.to_account_info();
    let bridge_bump = ctx.accounts.bridge_state.bump;
    
    let bridge_state = &mut ctx.accounts.bridge_state;
    let bridge_transaction = &mut ctx.accounts.bridge_transaction;
    let validator_info = &ctx.accounts.validator_info;

    // Check if bridge is active and not paused (matching Qubic contract validation)
    require!(bridge_state.is_active, BridgeError::BridgeNotActive);
    require!(!bridge_state.is_paused, BridgeError::BridgePaused);
    
    // Validate validator
    require!(validator_info.is_active, BridgeError::UnauthorizedValidator);
    
    // Validate lock exists and hasn't been processed (matching Qubic contract)
    require!(!bridge_transaction.is_processed, BridgeError::TransactionAlreadyCompleted);
    require!(bridge_transaction.id == lock_id, BridgeError::InvalidBridgeTransaction);
    
    // Validate amount
    require!(amount > 0, BridgeError::InvalidAmount);
    
    // Validate signature count (matching Qubic contract requiredSignatures check)
    require!(
        qubic_signatures.len() >= bridge_state.validator_threshold as usize,
        BridgeError::InsufficientValidatorSignatures
    );

    // TODO: In production, validate actual signatures against registered validators
    // For now, we trust the validator calling this function has verified the signatures

    // Transfer tokens from bridge to recipient
    let bridge_seeds = &[b"bridge_state".as_ref(), &[bridge_bump]];
    let bridge_signer = &[&bridge_seeds[..]];
    
    let transfer_instruction = Transfer {
        from: ctx.accounts.bridge_token_account.to_account_info(),
        to: ctx.accounts.recipient_token_account.to_account_info(),
        authority: bridge_account_info,
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        bridge_signer,
    );

    token::transfer(cpi_ctx, amount)?;

    // Update bridge transaction (matching Qubic contract behavior)
    bridge_transaction.status = BRIDGE_STATUS_COMPLETED;
    bridge_transaction.updated_at = Clock::get()?.unix_timestamp;
    bridge_transaction.completed_at = Clock::get()?.unix_timestamp;
    bridge_transaction.validator_signatures = qubic_signatures;
    bridge_transaction.is_processed = true;

    // Update bridge state statistics (matching Qubic contract)
    bridge_state.total_unlocked_tokens += amount;
    bridge_state.total_validator_actions += 1;

    msg!(
        "Assets unlocked: {} tokens from Lock ID: {} to recipient: {}",
        amount,
        lock_id,
        recipient
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(lock_id: u64, recipient: Pubkey)]
pub struct UnlockAssets<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump = bridge_state.bump
    )]
    pub bridge_state: Account<'info, BridgeState>,
    
    #[account(
        mut,
        seeds = [b"bridge_tx", lock_id.to_le_bytes().as_ref()],
        bump = bridge_transaction.bump
    )]
    pub bridge_transaction: Account<'info, BridgeTransaction>,
    
    #[account(
        seeds = [b"validator", validator.key().as_ref()],
        bump = validator_info.bump
    )]
    pub validator_info: Account<'info, ValidatorInfo>,
    
    #[account(mut)]
    pub validator: Signer<'info>,
    
    #[account(
        mut,
        constraint = recipient_token_account.owner == recipient
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = bridge_token_account.mint == recipient_token_account.mint
    )]
    pub bridge_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}