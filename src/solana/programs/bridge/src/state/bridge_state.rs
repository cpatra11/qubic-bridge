use anchor_lang::prelude::*;

pub const BRIDGE_STATUS_PENDING: u8 = 0;
pub const BRIDGE_STATUS_CONFIRMED: u8 = 1;
pub const BRIDGE_STATUS_COMPLETED: u8 = 2;
pub const BRIDGE_STATUS_FAILED: u8 = 3;

#[account]
pub struct BridgeState {
    pub admin: Pubkey,
    pub is_paused: bool,
    pub validator_threshold: u8,
    pub active_validators: u8,
    pub total_locked_tokens: u64,
    pub total_bridge_transactions: u64,
    pub next_bridge_id: u64,
    pub last_qubic_block: u64,
    pub emergency_pause_timestamp: i64,
    pub bump: u8,
}

impl BridgeState {
    pub const INIT_SPACE: usize = 32 + 1 + 1 + 1 + 8 + 8 + 8 + 8 + 8 + 1;
    
    pub fn is_emergency_paused(&self) -> bool {
        self.is_paused
    }
    
    pub fn can_bridge(&self, amount: u64) -> bool {
        !self.is_paused && amount > 0
    }
}

#[account]
pub struct BridgeTransaction {
    pub id: u64,
    pub user: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    pub qubic_destination: [u8; 32],
    pub status: u8, // 0: Pending, 1: Confirmed, 2: Completed, 3: Failed
    pub confirmations: u64,
    pub created_at: i64,
    pub updated_at: i64,
    pub completed_at: i64,
    pub qubic_tx_hash: [u8; 32],
    pub validator_signatures: Vec<[u8; 64]>,
    pub bump: u8,
}

impl BridgeTransaction {
    pub const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 32 + 1 + 8 + 8 + 8 + 8 + 32 + 4 + (64 * 10) + 1;
    
    pub fn is_completed(&self) -> bool {
        self.status == BRIDGE_STATUS_COMPLETED
    }
    
    pub fn is_pending(&self) -> bool {
        self.status == BRIDGE_STATUS_PENDING
    }
}

#[account]
pub struct ValidatorInfo {
    pub pubkey: Pubkey,
    pub qubic_validator_id: String,
    pub is_active: bool,
    pub stake: u64,
    pub last_activity: i64,
    pub bump: u8,
}

impl ValidatorInfo {
    pub const INIT_SPACE: usize = 32 + 4 + 32 + 1 + 8 + 8 + 1; // Pubkey + String (max 32) + bool + u64 + i64 + u8
    
    pub fn update_reputation(&mut self, success: bool) {
        if success {
            self.stake += 1;
        } else if self.stake > 0 {
            self.stake -= 1;
        }
        self.last_activity = Clock::get().unwrap().unix_timestamp;
    }
}

// Bridge direction constants
pub const BRIDGE_DIRECTION_SOLANA_TO_QUBIC: u8 = 0;
pub const BRIDGE_DIRECTION_QUBIC_TO_SOLANA: u8 = 1;