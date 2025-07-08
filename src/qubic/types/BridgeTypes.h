#pragma once

#include <cstdint>

// Chain IDs
static const uint8 QUBIC_CHAIN_ID = 1;
static const uint8 SOLANA_CHAIN_ID = 2;

// Bridge status constants
static const uint8 BRIDGE_STATUS_PENDING = 0;
static const uint8 BRIDGE_STATUS_CONFIRMED = 1;
static const uint8 BRIDGE_STATUS_COMPLETED = 2;
static const uint8 BRIDGE_STATUS_FAILED = 3;

// Maximum constants
static const uint32 MAX_BRIDGE_TRANSACTIONS = 10000;
static const uint32 MAX_ASSETS = 1000;
static const uint32 MAX_VALIDATORS = 100;
static const uint8 MIN_VALIDATOR_SIGNATURES = 3;

// Asset structure
struct Asset {
    uint64 id;
    uint8 tokenType; // 0: Native, 1: Token, 2: NFT
    uint8 decimals;
    uint8 reserved[5]; // For future use
};

// Asset balance structure
struct AssetBalance {
    Asset asset;
    uint64 amount;
};

// Bridge transaction structure
struct BridgeTransaction {
    uint64 id;
    Asset asset;
    uint64 amount;
    uint8 sourceChain;
    uint8 destinationChain;
    uint8 sourceAddress[32];
    uint8 destinationAddress[32];
    uint64 timestamp;
    uint8 status;
    uint64 confirmations;
    uint8 validatorSignatures[MAX_VALIDATORS][64];
    uint8 validatorCount;
};

// Validator signature structure
struct ValidatorSignature {
    uint8 signature[64];
    uint8 publicKey[32];
    uint64 timestamp;
};

// Cross-chain message structure
struct CrossChainMessage {
    uint64 messageId;
    uint8 sourceChain;
    uint8 destinationChain;
    uint8 messageType; // 0: Asset transfer, 1: State sync, 2: Emergency
    uint8 payload[256];
    uint32 payloadSize;
    uint64 timestamp;
    uint8 status;
};
    std::string to; // Address of the recipient
    uint64_t amount; // Amount of the asset being transferred
    uint64_t timestamp; // Time of the transaction
};

struct BridgeStatus {
    uint64_t totalLockedAssets; // Total amount of assets locked in the bridge
    uint64_t totalUnlockedAssets; // Total amount of assets unlocked from the bridge
    std::vector<Transaction> transactionHistory; // History of transactions processed by the bridge
};

} // namespace Qubic

#endif // BRIDGE_TYPES_H