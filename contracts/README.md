# VeriLuxe Contracts

This directory contains both the Soroban smart contract and the REST API that acts as middleware.

## Project Structure

```
contracts/
├── contracts/          # Soroban Smart Contract
│   ├── src/
│   │   └── lib.rs      # FashionAuthContract code
│   ├── deploy-js/      # Deployment scripts
│   │   ├── deploy.js   # JavaScript deployment
│   │   ├── python-deploy.py  # Python deployment
│   │   └── README.md   # Complete deployment guide
│   └── Cargo.toml      # Contract dependencies
├── api/                # REST API in Rust
│   ├── src/            # API source code
│   ├── tests/          # Integration tests
│   ├── Cargo.toml      # API dependencies
│   └── README.md       # Detailed API documentation
└── scripts/            # Utility scripts
    ├── generate_keypair.js   # Generate keypair with Node.js
    ├── generate_keypair.py   # Generate keypair with Python
    └── README.md             # Scripts documentation
```

## Components

### 1. Smart Contract (`contracts/`)
- **Language**: Rust with Soroban SDK
- **Functions**: init, issue_certificate, verify, transfer, revoke, etc.
- **Main file**: `src/lib.rs`

### 2. REST API (`api/`)
- **Language**: Rust with axum
- **Port**: 3000 (configurable)
- **Function**: Middleware between frontend and smart contract
- **Complete documentation**: See `api/README.md`

### 3. Utility Scripts (`scripts/`)
- **Generate keypairs**: For creating admin keys
- **Support**: Node.js and Python
- **Usage**: See `scripts/README.md`

## Quick Start

### 1. Generate Admin Secret Key
```bash
cd scripts
node generate_keypair.js
# or
python generate_keypair.py
```

### 2. Configure API
```bash
cd api
cp .env.example .env
# Edit .env with your secret key and contract ID
```

### 3. Run API
```bash
cd api
cargo run
```

### 4. Deploy Smart Contract
```bash
cd contracts/deploy-js
npm install
npm run deploy
```

### 5. Compile Smart Contract
```bash
cd contracts
cargo build --target wasm32-unknown-unknown --release
```

## Useful Links

- **API Documentation**: [api/README.md](api/README.md)
- **Smart Contract Code**: [contracts/src/lib.rs](contracts/src/lib.rs)
- **Deployment Guide**: [contracts/deploy-js/README.md](contracts/deploy-js/README.md)
- **Scripts Documentation**: [scripts/README.md](scripts/README.md)
- **Stellar Laboratory**: https://laboratory.stellar.org/
- **Soroban Documentation**: https://soroban.stellar.org/

## Project Status

- ✅ **Smart Contract**: Fully implemented and tested
- ✅ **REST API**: Complete structure with mock implementation
- ✅ **Scripts**: Utilities for key generation
- ⚠️ **Real Integration**: Pending complete Soroban→API implementation

See `api/IMPLEMENTATION_GUIDE.md` for details on complete implementation.