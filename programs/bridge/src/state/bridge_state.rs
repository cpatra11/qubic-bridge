use anchor_lang::prelude::*;

pub const BRIDGE_STATUS_PENDING: u8 = 0;
pub const BRIDGE_STATUS_CONFIRMED: u8 = 1;
pub const BRIDGE_STATUS_COMPLETED: u8 = 2;
pub const BRIDGE_STATUS_FAILED: u8 = 3;

#[account]
pub struct BridgeState {
    pub admin: Pubkey,
    pub is_paused: bool,
    
    // Bridge configuration matching Qubic contract
    pub bridge_id: u64,
    pub min_lock_amount: u64,
    pub max_lock_amount: u64,
    pub bridge_fee: u64,              // Fee in basis points (100 = 1%)
    pub is_active: bool,
    
    // Validator management
    pub validator_threshold: u8,      // Required signatures (same as requiredSignatures in Qubic)
    pub active_validators: u8,
    
    // Chain configuration
    pub qubic_chain_id: u64,         // Qubic chain identifier
    
    // Statistics matching Qubic contract
    pub total_locked_tokens: u64,    // Same as totalLocked in Qubic
    pub total_unlocked_tokens: u64,  // Same as totalUnlocked in Qubic
    pub total_bridge_transactions: u64, // Same as totalTransfers in Qubic
    pub total_validator_actions: u64, // Same as totalValidatorActions in Qubic
    pub next_lock_id: u64,           // Same as nextLockId in Qubic
    
    pub last_qubic_block: u64,
    pub emergency_pause_timestamp: i64,
    pub bump: u8,
}

impl BridgeState {
    pub const INIT_SPACE: usize = 32 + 1 + 8 + 8 + 8 + 8 + 1 + 1 + 1 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 8 + 1;
    
    pub fn is_emergency_paused(&self) -> bool {
        self.is_paused
    }
    
    pub fn can_bridge(&self, amount: u64) -> bool {
        self.is_active && !self.is_paused && amount >= self.min_lock_amount && amount <= self.max_lock_amount
    }
    
    pub fn calculate_fee(&self, amount: u64) -> (u64, u64) {
        let fee = (amount * self.bridge_fee) / 10000; // Fee in basis points
        let net_amount = amount - fee;
        (net_amount, fee)
    }
}

#[account]
pub struct BridgeTransaction {
    pub id: u64,                     // Same as lockId in Qubic
    pub user: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,                 // Original amount before fees
    pub actual_amount: u64,          // Net amount after fees (same as Qubic)
    pub fee: u64,                    // Fee deducted
    pub qubic_destination: [u8; 32], // Qubic recipient address
    pub memo: [u8; 64],              // Optional memo (same as Qubic)
    pub status: u8,                  // 0: Pending, 1: Confirmed, 2: Completed, 3: Failed
    pub confirmations: u64,
    pub created_at: i64,
    pub updated_at: i64,
    pub completed_at: i64,
    pub qubic_tx_hash: [u8; 32],
    pub validator_signatures: Vec<[u8; 64]>,
    pub is_processed: bool,          // Same as lockProcessed in Qubic
    pub bump: u8,
}

impl BridgeTransaction {
    // Updated space calculation: id(8) + user(32) + token_mint(32) + amount(8) + actual_amount(8) + fee(8) + 
    // qubic_destination(32) + memo(64) + status(1) + confirmations(8) + created_at(8) + updated_at(8) + 
    // completed_at(8) + qubic_tx_hash(32) + validator_signatures(4 + 64*10) + is_processed(1) + bump(1)
    pub const INIT_SPACE: usize = 8 + 32 + 32 + 8 + 8 + 8 + 32 + 64 + 1 + 8 + 8 + 8 + 8 + 32 + 4 + (64 * 10) + 1 + 1;
    
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