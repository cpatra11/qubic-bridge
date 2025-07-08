// Solana-Qubic Bridge Frontend JavaScript

// ===========================================
// OFFLINE TESTING / MOCK MODE CONFIGURATION
// ===========================================

// Enable mock mode for offline testing
window.MOCK_MODE = true; // Set to false for real deployment

// Mock API responses for testing
const mockQubicAPI = {
    getBridgeInfo: () => ({
        bridgeId: 1,
        minLockAmount: 1000000,
        maxLockAmount: 1000000000000,
        bridgeFee: 100, // 1% in basis points
        isActive: true,
        validatorCount: 3,
        requiredSignatures: 2,
        totalLocked: 25000000000, // 25B QU
        totalUnlocked: 18000000000, // 18B QU  
        totalTransfers: 1547,
        totalValidatorActions: 892
    }),
    
    getValidators: () => ({
        validators: [
            'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA',
            'BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB',
            'CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC'
        ],
        validatorCount: 3,
        requiredSignatures: 2
    }),
    
    getLockInfo: (lockId) => ({
        locker: 'DDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDDD',
        amount: 5000000000, // 5B QU
        timestamp: Math.floor(Date.now() / 1000) - 3600, // 1 hour ago
        solanaDestination: '9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM',
        isProcessed: Math.random() > 0.5 // Random for demo
    }),
    
    lockAssets: (amount, destination) => {
        const fee = Math.floor(amount * 0.01); // 1% fee
        const actualAmount = amount - fee;
        return {
            lockId: Math.floor(Math.random() * 1000000) + 100000,
            actualAmount: actualAmount,
            fee: fee,
            success: true
        };
    },
    
    unlockAssets: (lockId, recipient, amount) => ({
        success: Math.random() > 0.2, // 80% success rate for demo
        actualAmount: amount * 0.99, // Small processing fee
        transactionHash: 'mock_tx_' + Math.random().toString(36).substr(2, 16)
    })
};

// Mock Solana API responses
const mockSolanaAPI = {
    getConnection: () => ({
        getBalance: async () => Math.floor(Math.random() * 10000000000), // Random SOL balance
        getLatestBlockhash: async () => ({
            blockhash: 'mock_blockhash_' + Math.random().toString(36).substr(2, 16),
            lastValidBlockHeight: Math.floor(Math.random() * 1000000) + 150000000
        })
    }),
    
    getBridgeState: async () => ({
        authority: '9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM',
        totalLocked: 1250000000000, // 1.25T lamports
        totalUnlocked: 890000000000, // 890B lamports
        validatorCount: 3,
        isActive: true,
        fee: 50 // 0.5% in basis points
    }),
    
    bridgeToQubic: async (amount, qubicDestination) => {
        // Simulate transaction
        await new Promise(resolve => setTimeout(resolve, 2000));
        return {
            signature: 'mock_signature_' + Math.random().toString(36).substr(2, 32),
            success: Math.random() > 0.1, // 90% success rate
            amount: amount,
            fee: Math.floor(amount * 0.005) // 0.5% fee
        };
    }
};

// Mock transaction history
const mockTransactions = [
    {
        id: 'tx_001',
        type: 'sol_to_qubic',
        amount: 2.5,
        asset: 'SOL',
        status: 'completed',
        timestamp: Date.now() - 3600000,
        from: '9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM',
        to: 'AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA',
        fee: 0.0125,
        hash: 'sol_mock_hash_001'
    },
    {
        id: 'tx_002', 
        type: 'qubic_to_sol',
        amount: 1000000000, // 1B QU
        asset: 'QU',
        status: 'pending',
        timestamp: Date.now() - 1800000,
        from: 'BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB',
        to: '7fUAJdStEuGbc3sM84cKRL6yYaaSstyLSU4ve5oovLS7',
        fee: 10000000, // 10M QU

  const activeButton = document.querySelector(
    `[data-direction="${currentDirection}"]`
  );
  if (activeButton) {
    activeButton.classList.add("active");
  }

  updateBridgeSummary();
  updateBalanceDisplay();
}

// Update bridge summary
function updateBridgeSummary() {
  const amount = bridgeState.amount;
  const asset = bridgeState.asset;
  const fee = bridgeState.fee;
  const networkFee = bridgeState.networkFee;
  const total = amount + fee + networkFee;

  document.getElementById("summary-amount").textContent = `${amount.toFixed(
    6
  )} ${asset}`;
  document.getElementById("summary-fee").textContent = `${fee.toFixed(
    6
  )} ${asset}`;
  document.getElementById(
    "summary-network-fee"
  ).textContent = `~${networkFee.toFixed(6)} ${asset}`;
  document.getElementById("summary-total").textContent = `${total.toFixed(
    6
  )} ${asset}`;

  // Update estimated time based on amount and network
  const estimatedTime = amount > 1 ? "3-7 minutes" : "2-5 minutes";
  document.getElementById("summary-time").textContent = `~${estimatedTime}`;
}

// Update balance display
function updateBalanceDisplay() {
  const balanceSpan = document.getElementById("wallet-balance");
  const tokenSpan = document.getElementById("balance-token");

  // Mock balance data
  const balances = {
    SOL: currentDirection === "sol-to-qubic" ? 2.456789 : 0.123456,
    QU: currentDirection === "sol-to-qubic" ? 0.0 : 1000.0,
    USDC: 150.0,
  };

  const balance = balances[bridgeState.asset] || 0;
  balanceSpan.textContent = balance.toFixed(6);
  tokenSpan.textContent = bridgeState.asset;
}

// Set maximum amount
function setMaxAmount() {
  const balanceText = document.getElementById("wallet-balance").textContent;
  const balance = parseFloat(balanceText);
  const maxAmount = Math.max(
    0,
    balance - bridgeState.fee - bridgeState.networkFee
  );

  document.getElementById("amount-input").value = maxAmount.toFixed(6);
  bridgeState.amount = maxAmount;
  updateBridgeSummary();
  validateBridgeForm();
}

// Use connected wallet address
function useConnectedWallet() {
  const mockAddresses = {
    "sol-to-qubic": "BZFPUANP4AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA2LZBQ",
    "qubic-to-sol": "4xKrGHbJu9hJ8JqTKvqyhsP8Y8kV2Fn8w",
  };

  document.getElementById("destination-address").value =
    mockAddresses[currentDirection];
  bridgeState.destinationAddress = mockAddresses[currentDirection];
  validateBridgeForm();
}

// Generate new address
function generateAddress() {
  const mockNewAddresses = {
    "sol-to-qubic":
      "BZFPUANP" +
      Math.random().toString(36).substr(2, 28).toUpperCase() +
      "LZBQ",
    "qubic-to-sol":
      Math.random().toString(36).substr(2, 8) +
      "..." +
      Math.random().toString(36).substr(2, 4),
  };

  document.getElementById("destination-address").value =
    mockNewAddresses[currentDirection];
  bridgeState.destinationAddress = mockNewAddresses[currentDirection];
  validateBridgeForm();
}

// Validate bridge form
function validateBridgeForm() {
  const bridgeBtn = document.querySelector(".bridge-btn");
  const isValid =
    bridgeState.amount > 0 &&
    bridgeState.destinationAddress.length > 0 &&
    bridgeState.amount <=
      parseFloat(document.getElementById("wallet-balance").textContent);

  bridgeBtn.disabled = !isValid;
  bridgeBtn.style.opacity = isValid ? "1" : "0.6";
}

// Initiate bridge transaction
function initiateBridge() {
  if (bridgeState.amount <= 0) {
    showNotification("Please enter a valid amount", "error");
    return;
  }

  if (!bridgeState.destinationAddress) {
    showNotification("Please enter a destination address", "error");
    return;
  }

  // Show loading state
  const bridgeBtn = document.querySelector(".bridge-btn");
  const originalContent = bridgeBtn.innerHTML;
  bridgeBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Processing...';
  bridgeBtn.disabled = true;

  // Simulate transaction processing
  setTimeout(() => {
    showBridgeModal();
    bridgeBtn.innerHTML = originalContent;
    bridgeBtn.disabled = false;

    // Add to transaction history
    addTransactionToHistory();

    showNotification("Bridge transaction initiated successfully!", "success");
  }, 2000);
}

// Show bridge progress modal
function showBridgeModal() {
  const modal = document.getElementById("bridge-modal");
  modal.classList.add("active");

  // Simulate progress updates
  simulateBridgeProgress();
}

// Close modal
function closeModal() {
  const modal = document.getElementById("bridge-modal");
  modal.classList.remove("active");
}

// Simulate bridge progress
function simulateBridgeProgress() {
  const steps = document.querySelectorAll(".step");
  let currentStep = 1;

  const progressInterval = setInterval(() => {
    if (currentStep < steps.length) {
      // Complete current step
      steps[currentStep - 1].classList.remove("active");
      steps[currentStep - 1].classList.add("completed");

      // Activate next step
      if (currentStep < steps.length) {
        steps[currentStep].classList.add("active");
      }

      currentStep++;
    } else {
      clearInterval(progressInterval);
      // All steps completed
      setTimeout(() => {
        closeModal();
        showNotification("Bridge transaction completed!", "success");
      }, 2000);
    }
  }, 3000);
}

// Wallet Connection Functions
function connectSolanaWallet() {
  const btn = document.querySelector(".wallet-card .connect-btn");
  btn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Connecting...';

  setTimeout(() => {
    connectedWallets.solana = "4xKrGHbJu9hJ8JqTKvqyhsP8Y8kV2Fn8w";
    btn.innerHTML = '<i class="fas fa-check"></i> Connected';
    btn.style.background = "#00d4aa";
    btn.style.color = "white";
    showNotification("Solana wallet connected!", "success");
    updateNetworkStatus("solana", true);
  }, 2000);
}

function connectQubicWallet() {
  const btn = document.querySelectorAll(".wallet-card .connect-btn")[1];
  btn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Connecting...';

  setTimeout(() => {
    connectedWallets.qubic = "BZFPUANP4AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA2LZBQ";
    btn.innerHTML = '<i class="fas fa-check"></i> Connected';
    btn.style.background = "#00d4aa";
    btn.style.color = "white";
    showNotification("Qubic wallet connected!", "success");
    updateNetworkStatus("qubic", true);
  }, 2000);
}

// Update network status
function updateNetworkStatus(network, connected) {
  const statusElement = document.getElementById(`${network}-status`);
  const statusDot = document.querySelector(`.${network}-status`);

  if (connected) {
    statusElement.textContent = "Connected";
    statusDot.style.background = "#00d4aa";
  } else {
    statusElement.textContent = "Disconnected";
    statusDot.style.background = "#ff6b6b";
  }
}

// Portfolio Functions
function loadPortfolioData() {
  // Simulate loading portfolio data
  console.log("Loading portfolio data...");

  // Update balances with mock data
  updatePortfolioBalances();
}

function updatePortfolioBalances() {
  // Mock portfolio update
  const balanceElements = document.querySelectorAll(
    ".balance-item span:last-child"
  );
  const mockBalances = ["2.456 SOL", "150.00 USDC", "1,000.00 QU"];

  balanceElements.forEach((element, index) => {
    if (mockBalances[index]) {
      element.textContent = mockBalances[index];
    }
  });
}

// Transaction History Functions
function loadTransactionHistory() {
  console.log("Loading transaction history...");
  // Transaction history is already in the HTML, but we could load more here
}

function addTransactionToHistory() {
  const transactionsList = document.querySelector(".transactions-list");
  const newTransaction = createTransactionElement();
  transactionsList.insertBefore(newTransaction, transactionsList.firstChild);
}

function createTransactionElement() {
  const div = document.createElement("div");
  div.className = "transaction-item pending";

  const route =
    currentDirection === "sol-to-qubic"
      ? '<img src="https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/So11111111111111111111111111111111111111112/logo.png" alt="Solana" class="chain-icon"><i class="fas fa-arrow-right"></i><div class="qubic-icon">Q</div>'
      : '<div class="qubic-icon">Q</div><i class="fas fa-arrow-right"></i><img src="https://raw.githubusercontent.com/solana-labs/token-list/main/assets/mainnet/So11111111111111111111111111111111111111112/logo.png" alt="Solana" class="chain-icon">';

  div.innerHTML = `
        <div class="transaction-status">
            <i class="fas fa-clock"></i>
            <span class="status-text">Pending</span>
        </div>
        <div class="transaction-details">
            <div class="transaction-route">
                ${route}
            </div>
            <div class="transaction-info">
                <div class="amount">${bridgeState.amount} ${bridgeState.asset}</div>
                <div class="time">Just now</div>
            </div>
        </div>
        <div class="transaction-progress">
            <div class="progress-bar">
                <div class="progress-fill" style="width: 20%"></div>
            </div>
            <span class="progress-text">Transaction Submitted</span>
        </div>
    `;

  return div;
}

function viewTransaction(txHash) {
  const explorerUrls = {
    "sol-to-qubic": `https://explorer.solana.com/tx/${txHash}`,
    "qubic-to-sol": `https://explorer.qubic.org/tx/${txHash}`,
  };

  window.open(explorerUrls[currentDirection], "_blank");
}

// Use Case Demo Functions
function loadUseCases() {
  console.log("Loading use cases...");
}

function startArbitrageDemo() {
  showNotification("Arbitrage demo: Analyzing price differences...", "info");

  setTimeout(() => {
    const profit = (Math.random() * 10 + 2).toFixed(2);
    showNotification(
      `Found opportunity: ${profit}% profit potential!`,
      "success"
    );
  }, 2000);
}

function startYieldDemo() {
  showNotification("Loading yield farming opportunities...", "info");

  setTimeout(() => {
    showNotification(
      "Found: QU-SOL LP pool with 12.5% APY on Solana!",
      "success"
    );
  }, 1500);
}

function startGamingDemo() {
  showNotification("Loading NFT marketplace...", "info");

  setTimeout(() => {
    showNotification(
      "Discovered: 156 cross-chain gaming NFTs available!",
      "success"
    );
  }, 1500);
}

function startComputeDemo() {
  showNotification("Checking Qubic compute availability...", "info");

  setTimeout(() => {
    showNotification(
      "Available: 2.4 PFLOPS of AI compute for 0.05 QU/hour!",
      "success"
    );
  }, 2000);
}

function startInstitutionalDemo() {
  showNotification("Loading institutional trading solutions...", "info");

  setTimeout(() => {
    showNotification(
      "Enterprise API access and custody solutions available!",
      "success"
    );
  }, 1500);
}

function startCommunityDemo() {
  showNotification("Connecting to Cross-Chain DAO...", "info");

  setTimeout(() => {
    showNotification(
      "Welcome to the Solana-Qubic Bridge DAO community!",
      "success"
    );
  }, 1500);
}

function startDemo() {
  const demoBtn = document.querySelector(".demo-btn");
  const originalContent = demoBtn.innerHTML;

  demoBtn.innerHTML = '<i class="fas fa-spinner fa-spin"></i> Running Demo...';
  demoBtn.disabled = true;

  setTimeout(() => {
    // Simulate demo scenario
    showNotification("Demo: Detected 7.3% arbitrage opportunity!", "success");

    setTimeout(() => {
      showNotification("Demo: Simulating bridge transaction...", "info");

      setTimeout(() => {
        showNotification(
          "Demo completed: $73 profit on $1000 trade!",
          "success"
        );
        demoBtn.innerHTML = originalContent;
        demoBtn.disabled = false;
      }, 3000);
    }, 2000);
  }, 2000);
}

// Utility Functions
function showNotification(message, type = "info") {
  // Create notification element
  const notification = document.createElement("div");
  notification.className = `notification ${type}`;
  notification.style.cssText = `
        position: fixed;
        top: 20px;
        right: 20px;
        background: ${
          type === "success"
            ? "#00d4aa"
            : type === "error"
            ? "#ff6b6b"
            : "#667eea"
        };
        color: white;
        padding: 15px 20px;
        border-radius: 10px;
        box-shadow: 0 4px 16px rgba(0,0,0,0.2);
        z-index: 1001;
        font-weight: 500;
        max-width: 300px;
        word-wrap: break-word;
        animation: slideInRight 0.3s ease;
    `;
  notification.textContent = message;

  // Add to page
  document.body.appendChild(notification);

  // Remove after 4 seconds
  setTimeout(() => {
    notification.style.animation = "slideOutRight 0.3s ease";
    setTimeout(() => {
      if (notification.parentNode) {
        notification.parentNode.removeChild(notification);
      }
    }, 300);
  }, 4000);
}

// Add CSS animations for notifications
const style = document.createElement("style");
style.textContent = `
    @keyframes slideInRight {
        from {
            transform: translateX(100%);
            opacity: 0;
        }
        to {
            transform: translateX(0);
            opacity: 1;
        }
    }
    
    @keyframes slideOutRight {
        from {
            transform: translateX(0);
            opacity: 1;
        }
        to {
            transform: translateX(100%);
            opacity: 0;
        }
    }
`;
document.head.appendChild(style);

// Status checking function
function startStatusCheck() {
  // Simulate network status checking
  setInterval(() => {
    // Randomly update network status to simulate real connections
    const solanaConnected = Math.random() > 0.1; // 90% uptime
    const qubicConnected = Math.random() > 0.2; // 80% uptime

    updateNetworkStatus("solana", solanaConnected);
    updateNetworkStatus("qubic", qubicConnected);
  }, 30000); // Check every 30 seconds
}

// Handle modal clicks outside content
document.getElementById("bridge-modal").addEventListener("click", (e) => {
  if (e.target === e.currentTarget) {
    closeModal();
  }
});

// Keyboard shortcuts
document.addEventListener("keydown", (e) => {
  // Close modal with Escape key
  if (e.key === "Escape") {
    closeModal();
  }

  // Quick tab switching with number keys
  if (e.ctrlKey && e.key >= "1" && e.key <= "4") {
    const tabIndex = parseInt(e.key) - 1;
    const tabs = ["bridge", "portfolio", "transactions", "use-cases"];
    const tabBtn = document.querySelector(`[data-tab="${tabs[tabIndex]}"]`);
    if (tabBtn) {
      tabBtn.click();
    }
    e.preventDefault();
  }
});

// Initialize tooltips and help text
function initializeHelp() {
  const helpElements = document.querySelectorAll("[data-help]");
  helpElements.forEach((element) => {
    element.addEventListener("mouseenter", showTooltip);
    element.addEventListener("mouseleave", hideTooltip);
  });
}

function showTooltip(e) {
  const helpText = e.target.getAttribute("data-help");
  const tooltip = document.createElement("div");
  tooltip.className = "tooltip";
  tooltip.textContent = helpText;
  tooltip.style.cssText = `
        position: absolute;
        background: #333;
        color: white;
        padding: 8px 12px;
        border-radius: 6px;
        font-size: 0.8rem;
        z-index: 1002;
        pointer-events: none;
        max-width: 200px;
        word-wrap: break-word;
    `;

  document.body.appendChild(tooltip);

  const rect = e.target.getBoundingClientRect();
  tooltip.style.left = rect.left + "px";
  tooltip.style.top = rect.bottom + 5 + "px";

  e.target._tooltip = tooltip;
}

function hideTooltip(e) {
  if (e.target._tooltip) {
    e.target._tooltip.remove();
    delete e.target._tooltip;
  }
}

// Performance monitoring
function trackBridgeMetrics() {
  const metrics = {
    bridgeCount: 0,
    totalVolume: 0,
    averageTime: 0,
    successRate: 0.98,
  };

  // This would normally send to analytics
  console.log("Bridge Metrics:", metrics);
}

// Error handling
window.addEventListener("error", (e) => {
  console.error("Frontend Error:", e.error);
  showNotification("An unexpected error occurred. Please try again.", "error");
});

// Initialize help system when DOM is ready
document.addEventListener("DOMContentLoaded", initializeHelp);

console.log("Solana-Qubic Bridge Frontend Initialized");
console.log(
  "Features: Cross-chain transfers, Portfolio management, Transaction history, Use case demos"
);
