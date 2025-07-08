#ifndef CRYPTO_H
#define CRYPTO_H

#include <cstdint>
#include <vector>
#include <string>

namespace Qubic {

class Crypto {
public:
    static std::vector<uint8_t> hash(const std::string& data);
    static bool verifySignature(const std::vector<uint8_t>& data, const std::vector<uint8_t>& signature, const std::vector<uint8_t>& publicKey);
    static std::vector<uint8_t> sign(const std::vector<uint8_t>& data, const std::vector<uint8_t>& privateKey);
};

} // namespace Qubic

#endif // CRYPTO_H