// filepath: /solana-qubic-bridge/solana-qubic-bridge/src/qubic/contracts/AssetLock.h
using namespace QPI;

struct AssetLock
{
public:
    struct LockInput
    {
        uint64 assetId;
        uint64 amount;
        address recipient;
    };

    struct LockOutput
    {
        bool success;
        string message;
    };

    struct UnlockInput
    {
        uint64 assetId;
        address recipient;
    };

    struct UnlockOutput
    {
        bool success;
        string message;
    };

private:
    mapping<uint64, uint64> lockedAssets; // assetId -> amount locked

    /**
    Lock assets on the Qubic Network
    */
    PUBLIC_PROCEDURE(Lock)
        require(input.amount > 0, "Amount must be greater than zero");
        lockedAssets[input.assetId] += input.amount;
        output.success = true;
        output.message = "Assets locked successfully.";
    _

    /**
    Unlock assets on the Qubic Network
    */
    PUBLIC_PROCEDURE(Unlock)
        require(lockedAssets[input.assetId] > 0, "No assets locked for this ID");
        lockedAssets[input.assetId] = 0; // Reset the locked amount
        output.success = true;
        output.message = "Assets unlocked successfully.";
    _

    REGISTER_USER_FUNCTIONS_AND_PROCEDURES

        REGISTER_USER_PROCEDURE(Lock, 1);
        REGISTER_USER_PROCEDURE(Unlock, 2);
    _

    INITIALIZE
        // Initialization logic if needed
    _
};