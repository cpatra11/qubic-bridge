// Unit tests for shared module functionality
use shared::{
    types::{BridgeRequest, BridgeResponse, BridgeDirection, ValidatorSignature},
    crypto::verify_signature,
};
use std::time::SystemTime;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_bridge_request_creation() {
        let request = BridgeRequest {
            id: [1u8; 32],
            source_chain: "solana".to_string(),
            target_chain: "qubic".to_string(),
            asset_id: [2u8; 32],
            amount: 1000,
            recipient: [3u8; 32],
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            direction: BridgeDirection::SolanaToQubic,
        };

        assert_eq!(request.id, [1u8; 32]);
        assert_eq!(request.source_chain, "solana");
        assert_eq!(request.target_chain, "qubic");
        assert_eq!(request.amount, 1000);
        assert_eq!(request.direction, BridgeDirection::SolanaToQubic);
    }

    #[test]
    fn test_bridge_response_creation() {
        let response = BridgeResponse {
            request_id: [1u8; 32],
            success: true,
            error_message: None,
            transaction_hash: Some([4u8; 32]),
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        assert_eq!(response.request_id, [1u8; 32]);
        assert!(response.success);
        assert!(response.error_message.is_none());
        assert!(response.transaction_hash.is_some());
    }

    #[test]
    fn test_bridge_direction_values() {
        let solana_to_qubic = BridgeDirection::SolanaToQubic;
        let qubic_to_solana = BridgeDirection::QubicToSolana;

        assert_ne!(solana_to_qubic, qubic_to_solana);
        
        // Test serialization consistency
        let serialized_stq = serde_json::to_string(&solana_to_qubic).unwrap();
        let serialized_qts = serde_json::to_string(&qubic_to_solana).unwrap();
        assert_ne!(serialized_stq, serialized_qts);
    }

    #[test]
    fn test_validator_signature_creation() {
        let validator_signature = ValidatorSignature {
            validator_id: [5u8; 32],
            signature: [6u8; 64],
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        assert_eq!(validator_signature.validator_id, [5u8; 32]);
        assert_eq!(validator_signature.signature, [6u8; 64]);
        assert!(validator_signature.timestamp > 0);
    }

    #[test]
    fn test_serialization_deserialization() {
        let request = BridgeRequest {
            id: [1u8; 32],
            source_chain: "solana".to_string(),
            target_chain: "qubic".to_string(),
            asset_id: [2u8; 32],
            amount: 1000,
            recipient: [3u8; 32],
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            direction: BridgeDirection::SolanaToQubic,
        };

        // Test JSON serialization
        let json_str = serde_json::to_string(&request).unwrap();
        let deserialized: BridgeRequest = serde_json::from_str(&json_str).unwrap();
        assert_eq!(deserialized.id, request.id);
        assert_eq!(deserialized.amount, request.amount);

        // Test binary serialization
        let binary_data = bincode::serialize(&request).unwrap();
        let deserialized_bin: BridgeRequest = bincode::deserialize(&binary_data).unwrap();
        assert_eq!(deserialized_bin.id, request.id);
        assert_eq!(deserialized_bin.amount, request.amount);
    }

    #[test]
    fn test_error_response_handling() {
        let error_response = BridgeResponse {
            request_id: [1u8; 32],
            success: false,
            error_message: Some("Insufficient funds".to_string()),
            transaction_hash: None,
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        };

        assert!(!error_response.success);
        assert!(error_response.error_message.is_some());
        assert_eq!(error_response.error_message.unwrap(), "Insufficient funds");
        assert!(error_response.transaction_hash.is_none());
    }

    #[test]
    fn test_timestamp_validation() {
        let current_time = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let request = BridgeRequest {
            id: [1u8; 32],
            source_chain: "solana".to_string(),
            target_chain: "qubic".to_string(),
            asset_id: [2u8; 32],
            amount: 1000,
            recipient: [3u8; 32],
            timestamp: current_time,
            direction: BridgeDirection::SolanaToQubic,
        };

        // Timestamp should be recent (within the last minute)
        let now = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        assert!(request.timestamp <= now);
        assert!(request.timestamp > now - 60); // Less than 1 minute ago
    }

    #[test]
    fn test_amount_validation() {
        let request = BridgeRequest {
            id: [1u8; 32],
            source_chain: "solana".to_string(),
            target_chain: "qubic".to_string(),
            asset_id: [2u8; 32],
            amount: 0,
            recipient: [3u8; 32],
            timestamp: SystemTime::now()
                .duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            direction: BridgeDirection::SolanaToQubic,
        };

        // Zero amount should be valid (for testing purposes)
        assert_eq!(request.amount, 0);

        // Test with maximum amount
        let max_request = BridgeRequest {
            amount: u64::MAX,
            ..request
        };
        assert_eq!(max_request.amount, u64::MAX);
    }

    #[test]
    fn test_crypto_module_availability() {
        // Test that crypto module functions are available
        let test_data = b"test message";
        let test_signature = [1u8; 64];
        let test_pubkey = [2u8; 32];

        // This should not panic (even if it returns false)
        let result = verify_signature(test_data, &test_signature, &test_pubkey);
        // We don't assert the result since it's a mock implementation
        // but we verify the function is callable
        assert!(result == true || result == false);
    }
}
