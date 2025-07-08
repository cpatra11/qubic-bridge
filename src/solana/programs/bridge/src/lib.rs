use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Transfer};

mod instructions;
mod state;
mod error;

use crate::instructions::*;
use crate::state::*;
use crate::error::*;

declare_id!("BridgeProgram1111111111111111111111111111112");

#[program]
pub mod qubic_bridge {
    use super::*;

    pub fn initialize_bridge(
        ctx: Context<InitializeBridge>,
        admin: Pubkey,
        validator_threshold: u8,
    ) -> Result<()> {
        instructions::initialize_bridge::handler(ctx, admin, validator_threshold)
    }

    pub fn lock_assets(
        ctx: Context<LockAssets>,
        amount: u64,
        qubic_destination: [u8; 32],
        bridge_id: u64,
    ) -> Result<()> {
        instructions::lock_assets::handler(ctx, amount, qubic_destination, bridge_id)
    }

    pub fn unlock_assets(
        ctx: Context<UnlockAssets>,
        amount: u64,
        bridge_id: u64,
        qubic_signatures: Vec<[u8; 64]>,
    ) -> Result<()> {
        instructions::unlock_assets::handler(ctx, amount, bridge_id, qubic_signatures)
    }

    pub fn add_validator(
        ctx: Context<AddValidator>,
        validator_pubkey: Pubkey,
        qubic_validator_id: [u8; 32],
    ) -> Result<()> {
        instructions::add_validator::handler(ctx, validator_pubkey, qubic_validator_id)
    }

    pub fn remove_validator(
        ctx: Context<RemoveValidator>,
        validator_pubkey: Pubkey,
    ) -> Result<()> {
        instructions::remove_validator::handler(ctx, validator_pubkey)
    }

    pub fn update_bridge_state(
        ctx: Context<UpdateBridgeState>,
        bridge_id: u64,
        status: u8,
        confirmations: u64,
    ) -> Result<()> {
        instructions::update_bridge_state::handler(ctx, bridge_id, status, confirmations)
    }

    pub fn emergency_pause(ctx: Context<EmergencyPause>) -> Result<()> {
        instructions::emergency_pause::handler(ctx)
    }

    pub fn emergency_unpause(ctx: Context<EmergencyUnpause>) -> Result<()> {
        instructions::emergency_unpause::handler(ctx)
    }
}