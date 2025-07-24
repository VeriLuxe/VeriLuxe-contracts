# VeriLuxe 🌟

**VeriLuxe** is a luxury product authentication platform that uses blockchain technology to guarantee the authenticity of high-end fashion items. Built on Stellar Soroban smart contracts.

![License](https://img.shields.io/badge/license-MIT-blue.svg)
![Stellar](https://img.shields.io/badge/Stellar-Soroban-blue.svg)
![Next.js](https://img.shields.io/badge/Next.js-15.4.2-black.svg)
![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)

## 🎯 Key Features

- **🔐 Blockchain Authentication**: Immutable authenticity certificates on Stellar Soroban
- **👛 Wallet Integration**: Complete support for Freighter and Stellar-compatible wallets
- **🎨 Modern Interface**: Frontend built with Next.js 15 and Tailwind CSS
- **⚡ REST API**: Rust middleware for efficient smart contract interaction
- **🔄 Ownership Transfer**: Complete certificate management system
- **👨‍💼 Admin Panel**: Comprehensive tools for certificate issuance and revocation

## 🏗️ System Architecture

```
VeriLuxe/
├── 🎨 frontend/           # Next.js 15 + TypeScript + Tailwind
│   ├── src/
│   │   ├── app/           # Next.js App Router
│   │   ├── components/    # Modular React components
│   │   ├── contexts/      # Wallet management context
│   │   └── lib/           # Contract utilities and configuration
│   └── package.json       # Frontend dependencies
├── 🔧 contracts/          # Smart Contracts and API Backend
│   ├── contracts/         # Soroban Smart Contract (Rust)
│   │   ├── src/lib.rs     # Main FashionAuthContract
│   │   └── deploy-js/     # Deployment scripts
│   ├── api/               # REST API Middleware (Rust + Axum)
│   │   ├── src/           # API source code
│   │   └── tests/         # Integration tests
│   └── scripts/           # Keypair generation utilities
└── 📚 docs/               # GitBook Documentation
```

## 🚀 Quick Start

### Prerequisites

- **Node.js** 18+ and npm/yarn
- **Rust** 1.70+ with wasm32-unknown-unknown target
- **Soroban CLI** for contract deployment
- **Stellar Wallet** (Freighter recommended)

### 1. Clone Repository

```bash
git clone https://github.com/JosueBrenes/VeriLuxe.git
cd VeriLuxe
```

### 2. Setup Smart Contract

```bash
# Generate admin keypair
cd contracts/scripts
node generate_keypair.js

# Compile contract
cd ../contracts
cargo build --target wasm32-unknown-unknown --release

# Deploy contract
cd deploy-js
npm install
npm run deploy
```

### 3. Configure API Backend

```bash
cd contracts/api
cp .env.example .env
# Edit .env with your secret key and contract ID
cargo run
```

### 4. Setup Frontend

```bash
cd frontend
npm install
cp .env.example .env.local
# Configure environment variables
npm run dev
```

Application will be available at `http://localhost:3000`

## 🛠️ Technology Stack

### Frontend
- **Next.js 15.4.2** - React framework with App Router
- **TypeScript** - Static typing
- **Tailwind CSS 4** - Utility-first styling framework
- **Stellar SDK** - Stellar blockchain integration
- **Radix UI** - Accessible UI components
- **Lucide React** - Modern iconography

### Backend & Smart Contracts
- **Rust** - Primary language for contracts and API
- **Soroban SDK** - Stellar smart contracts
- **Axum** - Async web framework for REST API
- **Stellar SDK** - Stellar network interaction

### Blockchain
- **Stellar Soroban** - Smart contract platform
- **Testnet/Mainnet** - Supported networks
- **Freighter** - Primary supported wallet

## 📋 System Features

### For General Users
- ✅ **Certificate Verification**: Verify authenticity using certificate ID and metadata hash
- ✅ **Wallet Connection**: Secure integration with Stellar wallets
- ✅ **Ownership Transfer**: Transfer certificates between addresses
- ✅ **Transaction History**: View certificate movement history

### For Administrators
- ✅ **Certificate Issuance**: Create new authenticity certificates
- ✅ **Certificate Revocation**: Invalidate certificates when needed
- ✅ **Admin Panel**: Statistics and certificate management
- ✅ **Contract Initialization**: Initial system setup

## 🔧 REST API Endpoints

The REST API provides middleware between frontend and smart contracts:

```bash
# Health Check
GET /health

# Contract initialization
POST /init
{"admin_address": "GXXXXXXX..."}

# Certificate management
POST /certificates                    # Issue certificate
GET /certificates/:id                 # Get details
POST /certificates/:id/verify         # Verify authenticity
POST /certificates/:id/transfer       # Transfer ownership
POST /certificates/:id/revoke         # Revoke certificate
GET /certificates/:id/exists          # Check existence
```

## 🧪 Testing

### Smart Contract
```bash
cd contracts/contracts
cargo test
```

### API Backend
```bash
cd contracts/api
cargo test
cargo test --test integration_tests
```

### Frontend
```bash
cd frontend
npm run lint
npm run build
```

## 🌐 Network Configuration

### Testnet (Default)
```env
NEXT_PUBLIC_NETWORK=testnet
NEXT_PUBLIC_NETWORK_PASSPHRASE=Test SDF Network ; September 2015
NEXT_PUBLIC_RPC_URL=https://soroban-testnet.stellar.org
```

### Mainnet
```env
NEXT_PUBLIC_NETWORK=mainnet
NEXT_PUBLIC_NETWORK_PASSPHRASE=Public Global Stellar Network ; September 2015
NEXT_PUBLIC_RPC_URL=https://mainnet.sorobanrpc.com
```

## 📚 Documentation

- **[Smart Contracts](./contracts/README.md)** - Complete contract documentation
- **[REST API](./contracts/api/README.md)** - Complete API guide
- **[Frontend](./frontend/README.md)** - Frontend configuration and development
- **[Scripts](./contracts/scripts/README.md)** - Utilities and tools
- **[Deployment](./contracts/contracts/deploy-js/README.md)** - Deployment guide
- **[Complete Documentation](./docs/README.md)** - GitBook documentation

## 🔒 Security

- 🔐 **Private Keys**: Never stored on server
- 🛡️ **Authentication**: Based on Stellar cryptographic signatures
- 🔄 **Immutability**: Certificates stored on immutable blockchain
- ✅ **Validation**: Comprehensive input and transaction validation

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch (`git checkout -b feature/AmazingFeature`)
3. Commit your changes (`git commit -m 'Add some AmazingFeature'`)
4. Push to the branch (`git push origin feature/AmazingFeature`)
5. Open a Pull Request

## 📄 Project Status

- ✅ **Smart Contract**: Fully implemented and tested
- ✅ **REST API**: Complete structure with functional endpoints
- ✅ **Frontend**: Complete interface with wallet integration
- ✅ **Deployment**: Automated deployment scripts
- ⚠️ **Full Integration**: In optimization process
- 📚 **Documentation**: GitBook in development

## 🔗 Useful Links

- **[Stellar Laboratory](https://laboratory.stellar.org/)** - Stellar development tool
- **[Soroban Documentation](https://soroban.stellar.org/)** - Official Soroban docs
- **[Freighter Wallet](https://freighter.app/)** - Recommended wallet
- **[Stellar SDK Docs](https://stellar.github.io/js-stellar-sdk/)** - JavaScript SDK

## 📧 Support

For issues or questions:
1. Check browser console for error messages
2. Verify environment variable configuration
3. Ensure smart contract is properly deployed
4. Check network connectivity

## 📜 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

**Built with ❤️ for the luxury fashion community**

![VeriLuxe](https://img.shields.io/badge/VeriLuxe-Fashion%20Authentication-gold.svg)