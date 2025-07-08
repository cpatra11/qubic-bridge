# 🚀 READY TO DEPLOY TO QUBIC TESTNET!

Your HM25 contract is committed and ready for deployment. Follow these final steps:

## Step 1: Create GitHub Repository

1. Go to **GitHub.com** and create a new **PUBLIC** repository named `solana-qubic-bridge`
2. **DO NOT** initialize with README, .gitignore, or license (we already have these)

## Step 2: Push to GitHub

Run these commands in your terminal:

```powershell
# Add your GitHub remote (replace YOUR_USERNAME with your actual GitHub username)
git remote add origin https://github.com/YOUR_USERNAME/solana-qubic-bridge.git

# Rename branch to main
git branch -M main

# Push to GitHub
git push -u origin main
```

## Step 3: Deploy to Qubic Testnet

Once pushed to GitHub, connect to the testnet and deploy:

```bash
# SSH to testnet
ssh roots@67.222.157.68
# Password: 1vg2tQdZQ!

# Navigate to deployment directory
cd /root/qubic/qubic-docker

# Deploy your HM25 contract (replace YOUR_USERNAME)
./deploy.sh https://github.com/YOUR_USERNAME/solana-qubic-bridge/tree/main/qubic-core-madrid-2025
```

## Step 4: Verify Success

Look for this message:
```
Deployment completed successfully
```

## 🎯 Your HM25 Contract Details

- **Contract Index:** 12
- **Contract Name:** HM25
- **Construction Epoch:** 152
- **Network:** Qubic Testnet

## 🧪 Test Your Deployed Contract

Once deployed, test with:
```bash
# Query bridge info
curl -X POST http://67.222.157.68:8080/v1/contracts/HM25/GetBridgeInfo \
  -H "Content-Type: application/json" \
  -d '{}'
```

## ✅ Files Included in Deployment

- ✅ HM25 smart contract (`qubic-core-madrid-2025/src/contracts/HM25.h`)
- ✅ Contract registration (`qubic-core-madrid-2025/src/contract_core/contract_def.h`)
- ✅ Complete Qubic core with bridge functionality
- ✅ Solana bridge programs (for future integration)
- ✅ Frontend demo
- ✅ Documentation and pitch deck

**Your repository is ready for deployment! 🎉**
