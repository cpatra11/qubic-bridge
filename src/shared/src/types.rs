use serde::{Deserialize, Serialize};
use solana_sdk::pubkey::Pubkey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeTransaction {
    pub id: u64,
    pub user: Pubkey,
    pub token_mint: Pubkey,
    pub amount: u64,
    #[serde(with = "serde_bytes")]
    pub qubic_destination: [u8; 32],
    pub solana_destination: Pubkey,
    pub status: BridgeStatus,
    pub confirmations: u64,
    pub created_at: i64,
    pub completed_at: Option<i64>,
    #[serde(with = "serde_bytes")]
    pub qubic_tx_hash: Option<[u8; 32]>,
    pub solana_tx_signature: Option<String>,
    pub validator_signatures: Vec<ValidatorSignature>,
    pub direction: BridgeDirection,
    pub retry_count: u32,
    pub last_retry_at: Option<i64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BridgeStatus {
    Pending,
    Confirmed,
    Completed,
    Failed,
    Expired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum BridgeDirection {
    SolanaToQubic,
    QubicToSolana,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorSignature {
    pub validator_pubkey: Pubkey,
    #[serde(with = "serde_bytes")]
    pub signature: [u8; 64],
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Asset {
    pub id: u64,
    pub token_type: AssetType,
    pub decimals: u8,
    pub symbol: String,
    pub name: String,
    pub total_supply: u64,
    pub circulating_supply: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum AssetType {
    Native,
    Token,
    NFT,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrossChainMessage {
    pub message_id: u64,
    pub source_chain: ChainId,
    pub destination_chain: ChainId,
    pub message_type: MessageType,
    pub payload: Vec<u8>,
    pub timestamp: i64,
    pub status: MessageStatus,
    pub retry_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum ChainId {
    Solana,
    Qubic,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageType {
    AssetTransfer,
    StateSync,
    Emergency,
    Heartbeat,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MessageStatus {
    Pending,
    Sent,
    Delivered,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ValidatorInfo {
    pub pubkey: Pubkey,
    #[serde(with = "serde_bytes")]
    pub qubic_validator_id: [u8; 32],
    pub is_active: bool,
    pub reputation_score: u64,
    pub total_validations: u64,
    pub successful_validations: u64,
    pub last_validation_timestamp: i64,
    pub stake_amount: u64,
    pub commission_rate: u16, // Basis points (0-10000)
    pub joined_at: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BridgeConfig {
    pub min_confirmations: u64,
    pub max_confirmations: u64,
    pub validator_threshold: u8,
    pub max_validators: u8,
    pub emergency_pause_enabled: bool,
    pub fee_rate: u64, // Basis points
    pub min_bridge_amount: u64,
    pub max_bridge_amount: u64,
    pub bridge_timeout_seconds: u64,
}

// Error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BridgeError {
    InsufficientBalance,
    InvalidAmount,
    InvalidDestination,
    BridgePaused,
    ValidatorThresholdNotMet,
    TransactionExpired,
    InvalidSignature,
    ChainCommunicationError,
    UnknownError(String),
}

impl std::fmt::Display for BridgeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BridgeError::InsufficientBalance => write!(f, "Insufficient balance"),
            BridgeError::InvalidAmount => write!(f, "Invalid amount"),
            BridgeError::InvalidDestination => write!(f, "Invalid destination"),
            BridgeError::BridgePaused => write!(f, "Bridge is paused"),
            BridgeError::ValidatorThresholdNotMet => write!(f, "Validator threshold not met"),
            BridgeError::TransactionExpired => write!(f, "Transaction expired"),
            BridgeError::InvalidSignature => write!(f, "Invalid signature"),
            BridgeError::ChainCommunicationError => write!(f, "Chain communication error"),
            BridgeError::UnknownError(msg) => write!(f, "Unknown error: {}", msg),
        }
    }
}

impl std::error::Error for BridgeError {}

// Helper functions
impl BridgeTransaction {
    pub fn new(
        id: u64,
        user: Pubkey,
        token_mint: Pubkey,
        amount: u64,
        direction: BridgeDirection,
    ) -> Self {
        Self {
            id,
            user,
            token_mint,
            amount,
            qubic_destination: [0; 32],
            solana_destination: Pubkey::default(),
            status: BridgeStatus::Pending,
            confirmations: 0,
            created_at: chrono::Utc::now().timestamp(),
            completed_at: None,
            qubic_tx_hash: None,
            solana_tx_signature: None,
            validator_signatures: Vec::new(),
            direction,
            retry_count: 0,
            last_retry_at: None,
        }
    }

    pub fn is_expired(&self, timeout_seconds: u64) -> bool {
        let now = chrono::Utc::now().timestamp();
        (now - self.created_at) as u64 > timeout_seconds
    }

    pub fn add_validator_signature(&mut self, signature: ValidatorSignature) {
        self.validator_signatures.push(signature);
    }

    pub fn has_enough_signatures(&self, threshold: u8) -> bool {
        self.validator_signatures.len() >= threshold as usize
    }
}