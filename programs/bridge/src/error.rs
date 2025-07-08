use anchor_lang::prelude::*;

#[error_code]
pub enum BridgeError {
    #[msg("Invalid instruction")]
    InvalidInstruction,
    
    #[msg("Bridge is currently paused")]
    BridgePaused,
    
    #[msg("Bridge is not active")]
    BridgeNotActive,
    
    #[msg("Unauthorized admin")]
    UnauthorizedAdmin,
    
    #[msg("Invalid fee percentage")]
    InvalidFee,
    
    #[msg("Invalid validator threshold")]
    InvalidValidatorThreshold,
    
    #[msg("Invalid amount provided")]
    InvalidAmount,
    
    #[msg("Insufficient balance")]
    InsufficientBalance,
    
    #[msg("Invalid token account")]
    InvalidTokenAccount,
    
    #[msg("Unauthorized validator")]
    UnauthorizedValidator,
    
    #[msg("Invalid bridge transaction")]
    InvalidBridgeTransaction,
    
    #[msg("Bridge transaction already completed")]
    TransactionAlreadyCompleted,
    
    #[msg("Bridge transaction already failed")]
    TransactionAlreadyFailed,
    
    #[msg("Insufficient validator signatures")]
    InsufficientValidatorSignatures,
    
    #[msg("Invalid validator signature")]
    InvalidValidatorSignature,
    
    #[msg("Validator threshold not met")]
    ValidatorThresholdNotMet,
    
    #[msg("Maximum validators reached")]
    MaximumValidatorsReached,
    
    #[msg("Validator already exists")]
    ValidatorAlreadyExists,
    
    #[msg("Validator not found")]
    ValidatorNotFound,
    
    #[msg("Invalid Qubic address")]
    InvalidQubicAddress,
    
    #[msg("Invalid Solana address")]
    InvalidSolanaAddress,
    
    #[msg("Bridge transaction not found")]
    BridgeTransactionNotFound,
    
    #[msg("Bridge transaction expired")]
    BridgeTransactionExpired,
    
    #[msg("Invalid bridge state")]
    InvalidBridgeState,
    
    #[msg("Emergency pause active")]
    EmergencyPauseActive,
    
    #[msg("Only admin can perform this action")]
    OnlyAdminAllowed,
    
    #[msg("Bridge already initialized")]
    BridgeAlreadyInitialized,
    
    #[msg("Invalid configuration")]
    InvalidConfiguration,
    
    #[msg("Cross-chain communication failed")]
    CrossChainCommunicationFailed,
    
    #[msg("Invalid cross-chain message")]
    InvalidCrossChainMessage,
    
    #[msg("Replay attack detected")]
    ReplayAttackDetected,
    
    #[msg("Rate limit exceeded")]
    RateLimitExceeded,
    
    #[msg("Bridge maintenance mode")]
    BridgeMaintenanceMode,
}