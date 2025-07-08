// filepath: /solana-qubic-bridge/solana-qubic-bridge/src/solana/programs/bridge/src/state/validator_set.rs
use anchor_lang::prelude::*;

#[account]
pub struct ValidatorSet {
    pub validators: Vec<Pubkey>,
    pub active: bool,
}

impl ValidatorSet {
    pub const LEN: usize = 4 + (32 * 100); // 4 bytes for the active flag and space for 100 Pubkeys
}