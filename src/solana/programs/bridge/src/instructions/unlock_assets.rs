// src/solana/programs/bridge/src/instructions/unlock_assets.rs
use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};
use crate::state::{BridgeState, BridgeTransaction, ValidatorInfo};
use crate::error::BridgeError;

pub fn handler(
    ctx: Context<UnlockAssets>,
    amount: u64,
    bridge_id: u64,
    qubic_signatures: Vec<[u8; 64]>,
) -> Result<()> {
    // Store account info and bump before mutable borrow
    let bridge_account_info = ctx.accounts.bridge_state.to_account_info();
    let bridge_bump = ctx.accounts.bridge_state.bump;
    
    let bridge_state = &mut ctx.accounts.bridge_state;
    let bridge_transaction = &mut ctx.accounts.bridge_transaction;
    let validator_info = &ctx.accounts.validator_info;

    // Check if bridge is paused
    require!(!bridge_state.is_paused, BridgeError::BridgePaused);
    
    // Validate validator
    require!(validator_info.is_active, BridgeError::UnauthorizedValidator);
    
    // Validate amount
    require!(amount > 0, BridgeError::InvalidAmount);
    
    // Validate signatures (simplified for now)
    require!(
        qubic_signatures.len() >= bridge_state.validator_threshold as usize,
        BridgeError::InsufficientValidatorSignatures
    );

    // Transfer tokens from bridge to user
    let bridge_seeds = &[b"bridge_state".as_ref(), &[bridge_bump]];
    let bridge_signer = &[&bridge_seeds[..]];
    
    let transfer_instruction = Transfer {
        from: ctx.accounts.bridge_token_account.to_account_info(),
        to: ctx.accounts.user_token_account.to_account_info(),
        authority: bridge_account_info,
    };

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
        bridge_signer,
    );

    token::transfer(cpi_ctx, amount)?;

    // Update bridge transaction
    bridge_transaction.status = 1; // Completed
    bridge_transaction.updated_at = Clock::get()?.unix_timestamp;
    bridge_transaction.completed_at = Clock::get()?.unix_timestamp;
    bridge_transaction.validator_signatures = qubic_signatures;

    // Update bridge state
    bridge_state.total_locked_tokens -= amount;

    msg!(
        "Assets unlocked: {} tokens from Bridge ID: {}",
        amount,
        bridge_id
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(amount: u64, bridge_id: u64)]
pub struct UnlockAssets<'info> {
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
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    pub validator: Signer<'info>,
    
    #[account(
        mut,
        constraint = user_token_account.owner == user.key()
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = bridge_token_account.mint == user_token_account.mint
    )]
    pub bridge_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
}