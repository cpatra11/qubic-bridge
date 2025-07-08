use anchor_lang::prelude::*;

#[account]
pub struct ValidatorState {
    pub validator_pubkey: Pubkey,
    pub qubic_validator_id: [u8; 32],
    pub is_active: bool,
}

#[account]
pub struct ValidationState {
    pub transaction_hash: [u8; 32],
    pub is_valid: bool,
}