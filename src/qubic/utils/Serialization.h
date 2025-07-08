#ifndef SERIALIZATION_H
#define SERIALIZATION_H

#include <cstdint>
#include <vector>
#include <string>

namespace Qubic {

class Serialization {
public:
    // Serialize a uint64 value to a byte array
    static std::vector<uint8_t> serializeUint64(uint64_t value);

    // Deserialize a byte array to a uint64 value
    static uint64_t deserializeUint64(const std::vector<uint8_t>& data, size_t& offset);

    // Serialize a string to a byte array
    static std::vector<uint8_t> serializeString(const std::string& value);

    // Deserialize a byte array to a string
    static std::string deserializeString(const std::vector<uint8_t>& data, size_t& offset);
};

} // namespace Qubic

#endif // SERIALIZATION_H