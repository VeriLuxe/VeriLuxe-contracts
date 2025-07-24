# VeriLuxe Smart Contract Deployment Guide

Complete deployment guide for the VeriLuxe Fashion Authentication smart contract built on Soroban (Stellar). This guide provides multiple deployment methods to ensure successful contract deployment regardless of CLI limitations or environment preferences.

## 🚀 Available Deployment Methods

### 1. JavaScript/Node.js Deployment (Recommended)

The most reliable method using the official Stellar JavaScript SDK.

**Prerequisites:**
- Node.js 16+ installed
- npm or yarn package manager

**Setup:**
```bash
cd contracts/contracts/deploy-js
npm install
```

**Configure Environment:**
```bash
cp .env.example .env
```

Edit `.env` with your settings:
```env
NETWORK=testnet                    # or futurenet
SECRET_KEY=your_secret_key_here    # Admin account secret key
CONTRACT_PATH=../target/wasm32-unknown-unknown/release/fashion_auth_contract.wasm
```

**Deploy Contract:**
```bash
npm run deploy
```

**Verify Deployment:**
```bash
npm run test
```

**Features:**
- ✅ Latest Stellar JavaScript SDK v12+
- ✅ Automatic account funding detection
- ✅ Contract initialization with admin setup
- ✅ Built-in deployment verification
- ✅ Cross-platform compatibility (Windows/Mac/Linux)
- ✅ Detailed logging and error handling
- ✅ Network switching support

### 2. Python Deployment

Alternative method using the Stellar Python SDK for users who prefer Python.

**Prerequisites:**
- Python 3.8+ installed
- pip package manager

**Setup:**
```bash
pip install stellar-sdk python-dotenv
```

**Configure:**
Create a `.env` file with your settings:
```env
NETWORK=testnet
SECRET_KEY=your_secret_key_here
```

**Deploy:**
```bash
python python-deploy.py testnet
# or for futurenet
python python-deploy.py futurenet
```

**Features:**
- ✅ Stellar Python SDK v8+ integration
- ✅ Mnemonic phrase and secret key support
- ✅ Multiple network support (testnet/futurenet)
- ✅ Comprehensive error handling and logging
- ✅ Contract verification after deployment
- ✅ Account balance checking
- ✅ Transaction fee estimation

### 3. Stellar Laboratory (Web Interface)

User-friendly web interface for manual deployment and testing.

**Prerequisites:**
- Web browser
- Compiled WASM file
- Funded Stellar account

**Steps:**
1. **Prepare WASM File:**
   - Ensure you have: `../target/wasm32-unknown-unknown/release/fashion_auth_contract.wasm`
   - File size should be ~5-6KB

2. **Access Laboratory:**
   - Go to [Stellar Laboratory](https://laboratory.stellar.org)
   - Switch to "Testnet" in the top-right corner

3. **Deploy Contract:**
   - Navigate to "Build Transaction" → "Upload Contract Wasm"
   - Upload your WASM file
   - Set source account (your funded account)
   - Review transaction parameters
   - Sign with your account keypair

4. **Submit & Verify:**
   - Submit the transaction
   - Copy the contract ID from the response
   - Use "Invoke Contract" to initialize with admin address

**Benefits:**
- ✅ No local development environment required
- ✅ Visual, step-by-step interface
- ✅ Real-time transaction debugging
- ✅ Network explorer integration
- ✅ Transaction history and status tracking
- ✅ Built-in XDR viewer and decoder

### 4. Freighter Wallet Integration

Browser extension wallet deployment for enhanced security.

**Prerequisites:**
- Chrome/Firefox browser
- [Freighter Wallet](https://freighter.app) extension
- Funded account in Freighter

**Setup Process:**
1. **Install Wallet:**
   - Add Freighter extension to your browser
   - Create or import account with recovery phrase
   - Switch to Testnet/Futurenet network

2. **Connect & Deploy:**
   - Use web applications that integrate with Freighter
   - Or use Laboratory with Freighter connection
   - Sign transactions directly in the wallet popup

3. **Security Benefits:**
   - Private keys never leave the wallet
   - Transaction approval required for each operation
   - Network switching protection

**Features:**
- ✅ Enhanced security with local key storage
- ✅ Browser-based transaction signing
- ✅ Network switching protection
- ✅ Integration with dApps and Laboratory
- ✅ Transaction history tracking
- ✅ Multi-account support

### 5. Local Development Network

Complete local development environment using Stellar Quickstart.

**Prerequisites:**
- Docker installed and running
- 8GB+ available RAM
- Port 8000 available

**Setup Local Network:**
```bash
# Start local Stellar network with Soroban support
docker run --rm -it -p 8000:8000 \
  stellar/quickstart:latest \
  --local \
  --enable-soroban-rpc \
  --enable-soroban-diagnostic-events
```

**Deploy to Local:**
```bash
# Configure local network
soroban network add local \
  --rpc-url http://localhost:8000/soroban/rpc \
  --network-passphrase "Standalone Network ; February 2017"

# Deploy contract
soroban contract deploy \
  --wasm ../target/wasm32-unknown-unknown/release/fashion_auth_contract.wasm \
  --source admin \
  --network local
```

**Features:**
- ✅ Complete local blockchain simulation
- ✅ Fast development and testing cycles
- ✅ No network fees or rate limits
- ✅ Full Soroban RPC and diagnostic support
- ✅ Reset-able blockchain state
- ✅ Immediate transaction confirmation

### 6. Soroban CLI (When Working)

Traditional CLI deployment method.

**Prerequisites:**
- Soroban CLI installed (`cargo install soroban-cli`)
- Network configured
- Funded account

**Quick Deploy:**
```bash
# Build contract
cd ../contracts
cargo build --target wasm32-unknown-unknown --release

# Deploy and initialize
soroban contract deploy \
  --wasm target/wasm32-unknown-unknown/release/fashion_auth_contract.wasm \
  --source admin \
  --network testnet

# Initialize with admin
soroban contract invoke \
  --id CONTRACT_ID \
  --source admin \
  --network testnet \
  -- init \
  --admin ADMIN_ADDRESS
```

**Note:** Use alternative methods if experiencing XDR or CLI issues.

## 📋 Pre-funded Test Accounts

**⚠️ Important Security Notice:**
- These accounts are for **TESTNET ONLY** - never use on mainnet
- These are publicly known accounts - funds may disappear
- For production, always generate new, secure keypairs

### Account 1 (Primary Admin)
- **Network:** Testnet
- **Address:** `GDQQJUWWKZCJDVTEODWJV6Q464WUECCJX7NWNSV43SX3ZTVI2YF6N57U`
- **Secret Key:** `SXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX`
- **Mnemonic:** `planet clutch exchange night upset panic catch acid history saddle dinner turn grass middle hospital awesome spice there basic obscure pigeon clutch satisfy firm`
- **Balance:** ~10,000 XLM (refreshed daily)
- **Usage:** Contract deployment and admin operations

### Account 2 (Secondary)
- **Network:** Testnet  
- **Address:** `GAE4GSKFNNLO3DWX2ABGZKBOEDZI5SH53HXS5HYD7DGF2F3NOQTF`
- **Mnemonic:** `coin idle bus total sense awful picture dial stick between erode expose stairs they swing account august indicate cruel nasty inherit vocal veteran deal`
- **Balance:** ~5,000 XLM
- **Usage:** Certificate owner operations and testing

### Account 3 (Testing)
- **Network:** Testnet
- **Address:** `GDMZLP4DVXQH7MLQGRGPCV75ZE6NSH547J4CKOAB5KOAB5KJ7QICJKBNBYTAR`
- **Mnemonic:** `valve exhaust situate ghost zone enhance cabbage clutch pigeon surface section glad kite aunt just angle text this pact merit oak inner mercy pioneer`
- **Balance:** ~1,000 XLM
- **Usage:** Multi-user testing scenarios

### Generate Your Own Account
For security, generate your own keypair using the provided scripts:
```bash
cd ../../scripts
node generate_keypair.js
```

Then fund it at: [Stellar Laboratory - Account Creator](https://laboratory.stellar.org/#account-creator?network=test)

## 🔧 Contract Technical Details

### WASM Binary Information
- **File Path:** `../target/wasm32-unknown-unknown/release/fashion_auth_contract.wasm`
- **Expected Size:** ~5-6KB (varies by optimization)
- **Target:** `wasm32-unknown-unknown`
- **Optimization:** Release build with `--release` flag

### Build Instructions
```bash
cd ..
cargo build --target wasm32-unknown-unknown --release
```

### Contract Metadata
- **Name:** FashionAuthContract
- **Version:** 1.0.0
- **Language:** Rust with Soroban SDK
- **License:** MIT

## 🧪 Contract Functions Reference

Complete function specification for the deployed contract:

### Admin Functions
```rust
// Initialize contract (one-time setup)
init(admin: Address)

// Revoke any certificate (admin only)
revoke(cert_id: String)

// Get current admin address
get_admin() -> Address
```

### Certificate Management
```rust
// Issue new authenticity certificate
issue_certificate(
    cert_id: String,           // Unique certificate identifier
    metadata_hash: String,     // IPFS or SHA256 hash of item metadata
    owner: Address            // Initial owner of the certificate
)

// Verify certificate authenticity
verify(
    cert_id: String,          // Certificate to verify
    metadata_hash: String     // Expected metadata hash
) -> bool                     // Returns true if valid and matches

// Transfer certificate ownership
transfer(
    cert_id: String,          // Certificate to transfer
    new_owner: Address        // New owner address
)

// Get complete certificate information
get_certificate_details(cert_id: String) -> Certificate {
    owner: Address,           // Current owner
    metadata_hash: String,    // Item metadata hash
    is_valid: bool           // Whether certificate is still valid
}

// Check if certificate exists
certificate_exists(cert_id: String) -> bool
```

### Usage Examples
```javascript
// Initialize contract
await contract.init(adminAddress);

// Issue certificate
await contract.issue_certificate(
    "CERT_LV_001", 
    "QmHash123...", 
    ownerAddress
);

// Verify authenticity
const isValid = await contract.verify("CERT_LV_001", "QmHash123...");

// Transfer ownership
await contract.transfer("CERT_LV_001", newOwnerAddress);
```

## 🌍 Network Configuration & Endpoints

### Testnet (Recommended for Development)
- **RPC URL:** `https://soroban-testnet.stellar.org:443`
- **Network Passphrase:** `Test SDF Network ; September 2015`
- **Explorer:** [stellar.expert/explorer/testnet](https://stellar.expert/explorer/testnet)
- **Friendbot (Funding):** `https://friendbot.stellar.org`
- **Account Creator:** [laboratory.stellar.org/#account-creator?network=test](https://laboratory.stellar.org/#account-creator?network=test)

### Futurenet (Bleeding Edge)
- **RPC URL:** `https://rpc-futurenet.stellar.org:443`
- **Network Passphrase:** `Test SDF Future Network ; October 2022`
- **Explorer:** [stellar.expert/explorer/futurenet](https://stellar.expert/explorer/futurenet)
- **Features:** Latest Soroban features and updates

### Mainnet (Production)
- **RPC URL:** `https://soroban-mainnet.stellar.org:443`
- **Network Passphrase:** `Public Global Stellar Network ; September 2015`
- **Explorer:** [stellar.expert/explorer/public](https://stellar.expert/explorer/public)
- **⚠️ Warning:** Real XLM required, permanent transactions

## 🔍 Troubleshooting & Common Issues

### Deployment Issues

#### 1. XDR Processing Errors
**Symptoms:** CLI fails with XDR-related messages
**Solutions:**
- ✅ Use JavaScript deployment method (#1)
- ✅ Try Python deployment (#2)
- ✅ Use Stellar Laboratory web interface (#3)
- ✅ Switch to a different network

#### 2. Account Funding Issues
**Symptoms:** "Insufficient balance" or account not found
**Solutions:**
- ✅ Visit [Stellar Laboratory Account Creator](https://laboratory.stellar.org/#account-creator?network=test)
- ✅ Use pre-funded test accounts provided above
- ✅ Check account balance: `soroban keys address ACCOUNT_NAME`
- ✅ Ensure you're on the correct network (testnet/futurenet)

#### 3. WASM File Not Found
**Symptoms:** "No such file or directory" errors
**Solutions:**
- ✅ Build contract first: `cargo build --target wasm32-unknown-unknown --release`
- ✅ Check file path: `../target/wasm32-unknown-unknown/release/`
- ✅ Verify WASM file exists and has expected size (~5-6KB)

#### 4. Network Connection Issues
**Symptoms:** Timeout or connection refused errors
**Solutions:**
- ✅ Check internet connection
- ✅ Try different RPC endpoints
- ✅ Switch between testnet and futurenet
- ✅ Use local development network (#5)

#### 5. CLI Command Failures
**Symptoms:** Soroban CLI commands not working
**Solutions:**
- ✅ Update CLI: `cargo install soroban-cli --force`
- ✅ Check version: `soroban --version` (should be 21.0.0+)
- ✅ Use alternative deployment methods (#1-#3)
- ✅ Clear CLI cache: `rm -rf ~/.config/soroban`

### Contract Interaction Issues

#### 6. Contract Not Initialized
**Symptoms:** Function calls fail with "not initialized"
**Solutions:**
- ✅ Call `init(admin_address)` after deployment
- ✅ Verify admin address is correct
- ✅ Check contract deployment status

#### 7. Authorization Errors  
**Symptoms:** "require_auth" failures
**Solutions:**
- ✅ Use correct account for operations
- ✅ Admin functions require admin account
- ✅ Transfer requires current owner account
- ✅ Check account signatures and authorization

### Development Workflow Issues

#### 8. Hot Reload Problems
**Symptoms:** Changes not reflected after redeployment
**Solutions:**
- ✅ Use different contract IDs for each deployment
- ✅ Clear browser cache and local storage
- ✅ Restart development servers
- ✅ Check contract address in frontend configuration

#### Getting Help
- 🌐 **Stellar Discord:** [discord.gg/stellar](https://discord.gg/stellar)
- 📚 **Soroban Docs:** [soroban.stellar.org](https://soroban.stellar.org)
- 🐛 **CLI Issues:** [github.com/stellar/stellar-cli/issues](https://github.com/stellar/stellar-cli/issues)
- 💬 **Stack Overflow:** Tag questions with `soroban` and `stellar`

## 🚀 Post-Deployment Next Steps

### 1. Contract Integration
After successful deployment:

```bash
# Save your contract ID
export CONTRACT_ID="CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"

# Initialize contract with admin
soroban contract invoke \
  --id $CONTRACT_ID \
  --source admin \
  --network testnet \
  -- init \
  --admin ADMIN_ADDRESS
```

### 2. Frontend Integration
Update your frontend configuration:

```javascript
// contracts/api/.env
FASHION_AUTH_CONTRACT_ID=CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX

// frontend/src/lib/contract-config.ts
export const CONTRACT_ID = "CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX";
```

### 3. API Configuration
Configure the REST API middleware:

```bash
cd ../../api
cp .env.example .env
# Update with your contract ID and admin secret key
cargo run
```

### 4. Testing Checklist
- [ ] Contract deployed successfully
- [ ] Contract initialized with admin
- [ ] API connects to contract
- [ ] Frontend loads contract functions
- [ ] Can issue test certificate
- [ ] Can verify certificate
- [ ] Can transfer ownership
- [ ] Admin can revoke certificates

## 📈 Production Considerations

### Security Best Practices
- ✅ Generate unique admin keypairs for production
- ✅ Store secret keys securely (environment variables)
- ✅ Use hardware wallets for high-value operations
- ✅ Implement multi-signature for admin functions
- ✅ Regular security audits and updates

### Monitoring & Maintenance
- 📊 Monitor contract usage and transaction costs
- 🔄 Keep track of certificate issuance and transfers
- 📝 Maintain logs of admin operations
- 🚨 Set up alerts for unusual activity
- 📊 Track network congestion and fees

### Scaling Considerations
- 🏗️ Consider contract upgrades for new features
- 💾 Plan for data archival and cleanup
- 🌐 Multi-network deployment strategy
- 🔄 Backup and disaster recovery procedures

## 📚 Additional Resources

### Documentation
- 📖 **Soroban by Example:** [soroban.stellar.org/docs/getting-started/setup](https://soroban.stellar.org/docs/getting-started/setup)
- 🛠️ **Smart Contract Best Practices:** [soroban.stellar.org/docs/smart-contracts/best-practices](https://soroban.stellar.org/docs/smart-contracts/best-practices)
- 🔧 **Stellar SDK Reference:** [stellar.github.io/js-stellar-sdk/](https://stellar.github.io/js-stellar-sdk/)

### Community & Support
- 💬 **Stellar Discord:** [discord.gg/stellar](https://discord.gg/stellar) - `#soroban` channel
- 🐛 **Report Issues:** [github.com/stellar/stellar-cli/issues](https://github.com/stellar/stellar-cli/issues)
- 📚 **Developer Portal:** [developers.stellar.org](https://developers.stellar.org)
- 🎓 **Stellar Quest:** [quest.stellar.org](https://quest.stellar.org) - Interactive tutorials

### Fashion Industry Integration
- 🏷️ **IPFS Integration:** For storing item metadata and images
- 📱 **QR Code Generation:** For physical item authentication
- 🛒 **E-commerce Integration:** APIs for marketplace platforms
- 📊 **Analytics Dashboard:** Track authentication requests and transfers

---

## 🎉 Congratulations!

Your **VeriLuxe Fashion Authentication Contract** is now ready for production use! 

The contract enables:
- ✅ **Luxury Item Authentication** - Verify genuine fashion items
- ✅ **Ownership Tracking** - Complete provenance and transfer history  
- ✅ **Anti-Counterfeiting** - Cryptographic proof of authenticity
- ✅ **Decentralized Trust** - No central authority required

**Happy Building!** 🌟