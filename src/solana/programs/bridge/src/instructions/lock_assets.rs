use anchor_lang::prelude::*;
use anchor_spl::token::{self, Transfer, Token, TokenAccount};
use crate::state::{BridgeState, BridgeTransaction, BRIDGE_STATUS_PENDING};
use crate::error::BridgeError;

pub fn handler(
    ctx: Context<LockAssets>,
    amount: u64,
    qubic_destination: [u8; 32],
    _bridge_id: u64,
) -> Result<()> {
    let bridge_state = &mut ctx.accounts.bridge_state;
    let bridge_transaction = &mut ctx.accounts.bridge_transaction;
    let clock = Clock::get()?;

    // Check if bridge is paused
    require!(!bridge_state.is_paused, BridgeError::BridgePaused);
    
    // Validate amount
    require!(amount > 0, BridgeError::InvalidAmount);
    
    // Validate token account ownership
    require!(
        ctx.accounts.user_token_account.owner == ctx.accounts.user.key(),
        BridgeError::InvalidTokenAccount
    );
    
    // Check sufficient balance
    require!(
        ctx.accounts.user_token_account.amount >= amount,
        BridgeError::InsufficientBalance
    );

    // Transfer tokens from user to bridge
    let transfer_instruction = Transfer {
        from: ctx.accounts.user_token_account.to_account_info(),
        to: ctx.accounts.bridge_token_account.to_account_info(),
        authority: ctx.accounts.user.to_account_info(),
    };

    let cpi_ctx = CpiContext::new(
        ctx.accounts.token_program.to_account_info(),
        transfer_instruction,
    );

    token::transfer(cpi_ctx, amount)?;

    // Initialize bridge transaction
    bridge_transaction.id = bridge_state.next_bridge_id;
    bridge_transaction.user = ctx.accounts.user.key();
    bridge_transaction.token_mint = ctx.accounts.user_token_account.mint;
    bridge_transaction.amount = amount;
    bridge_transaction.qubic_destination = qubic_destination;
    bridge_transaction.status = BRIDGE_STATUS_PENDING;
    bridge_transaction.confirmations = 0;
    bridge_transaction.created_at = clock.unix_timestamp;
    bridge_transaction.updated_at = clock.unix_timestamp;
    bridge_transaction.completed_at = 0;
    bridge_transaction.qubic_tx_hash = [0; 32];
    bridge_transaction.validator_signatures = Vec::new();
    bridge_transaction.bump = ctx.bumps.bridge_transaction;

    // Update bridge state
    bridge_state.total_locked_tokens += amount;
    bridge_state.total_bridge_transactions += 1;
    bridge_state.next_bridge_id += 1;

    msg!(
        "Assets locked: {} tokens to Qubic address {:?}, Bridge ID: {}",
        amount,
        qubic_destination,
        bridge_transaction.id
    );

    Ok(())
}

#[derive(Accounts)]
#[instruction(amount: u64, qubic_destination: [u8; 32], bridge_id: u64)]
pub struct LockAssets<'info> {
    #[account(
        mut,
        seeds = [b"bridge_state"],
        bump = bridge_state.bump
    )]
    pub bridge_state: Account<'info, BridgeState>,
    
    #[account(
        init,
        payer = user,
        space = 8 + BridgeTransaction::INIT_SPACE,
        seeds = [b"bridge_tx", bridge_id.to_le_bytes().as_ref()],
        bump
    )]
    pub bridge_transaction: Account<'info, BridgeTransaction>,
    
    #[account(mut)]
    pub user: Signer<'info>,
    
    #[account(
        mut,
        constraint = user_token_account.owner == user.key(),
        constraint = user_token_account.amount >= amount
    )]
    pub user_token_account: Account<'info, TokenAccount>,
    
    #[account(
        mut,
        constraint = bridge_token_account.mint == user_token_account.mint
    )]
    pub bridge_token_account: Account<'info, TokenAccount>,
    
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}