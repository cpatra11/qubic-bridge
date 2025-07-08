// filepath: /solana-qubic-bridge/solana-qubic-bridge/src/qubic/types/AssetTypes.h
#ifndef ASSET_TYPES_H
#define ASSET_TYPES_H

#include <string>
#include <vector>

namespace Qubic {

struct AssetMetadata {
    std::string assetName;
    std::string assetSymbol;
    uint64_t totalSupply;
    std::string ownerAddress;
};

struct AssetState {
    uint64_t lockedAmount;
    uint64_t unlockedAmount;
    std::vector<std::string> transactionHistory;
};

} // namespace Qubic

#endif // ASSET_TYPES_H