use anchor_lang::prelude::*;

mod instructions;
mod state;
mod error;

use crate::instructions::*;

declare_id!("4LXpG5445Kmv879vWzCFx5bTc7miCJAY96iQDMJB1zaP");

// QuantumLink - Solana-Qubic Cross-Chain Bridge
// Connecting the power of Solana with Qubic's quantum computing capabilities

#[program]
pub mod quantum_link {
    use super::*;

    /// Initialize the QuantumLink bridge with admin and validator settings

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
        memo: [u8; 64],
    ) -> Result<()> {
        instructions::lock_assets::handler(ctx, amount, qubic_destination, memo)
    }

    pub fn unlock_assets(
        ctx: Context<UnlockAssets>,
        lock_id: u64,
        recipient: Pubkey,
        amount: u64,
        qubic_signatures: Vec<[u8; 64]>,
    ) -> Result<()> {
        instructions::unlock_assets::handler(ctx, lock_id, recipient, amount, qubic_signatures)
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

    pub fn update_config(
        ctx: Context<UpdateConfig>,
        new_min_lock_amount: u64,
        new_max_lock_amount: u64,
        new_bridge_fee: u64,
        new_required_signatures: u8,
        new_is_active: bool,
    ) -> Result<()> {
        instructions::update_config::handler(
            ctx,
            new_min_lock_amount,
            new_max_lock_amount,
            new_bridge_fee,
            new_required_signatures,
            new_is_active,
        )
    }

    pub fn get_bridge_info(ctx: Context<GetBridgeInfo>) -> Result<BridgeInfoResponse> {
        instructions::query::handler(ctx)
    }

    pub fn get_lock_info(ctx: Context<GetLockInfo>, _lock_id: u64) -> Result<LockInfoResponse> {
        instructions::query::get_lock_info_handler(ctx)
    }
}