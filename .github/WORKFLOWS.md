# GitHub Workflows Documentation

This document describes the CI/CD workflows for the VeriLuxe Contracts repository.

## 🔄 Workflows Overview

### 1. Smart Contracts CI (`contracts-ci.yml`)
**Triggers:** Push/PR to `main`/`develop` with changes in `/contracts/**`

**Jobs:**
- **Lint**: Format check, Clippy analysis
- **Test**: Unit tests, contract building, optimization
- **Security**: Cargo audit, dependency checks
- **Coverage**: Code coverage with Tarpaulin

### 2. API CI (`api-ci.yml`)
**Triggers:** Push/PR to `main`/`develop` with changes in `/contracts/api/**`

**Jobs:**
- **Lint**: Rust formatting, Clippy
- **Test**: Unit & integration tests with Stellar testnet
- **Build**: Release build, artifacts upload
- **Docker**: Container image build and push
- **OpenAPI**: Spec validation, documentation generation

### 3. Deployment (`deploy.yml`)
**Triggers:** 
- Push to `main`
- Manual workflow dispatch
- Git tags (`v*`)

**Jobs:**
- **Deploy Contracts**: Soroban contract deployment to Stellar
- **Deploy API**: Docker deployment to production servers
- **Notifications**: Slack alerts, deployment status

## 🔧 Required Secrets

### Stellar/Soroban
- `STELLAR_SECRET_KEY`: Deployment account secret key
- `ADMIN_PUBLIC_KEY`: Contract admin public key
- `SOROBAN_RPC_URL`: Stellar RPC endpoint
- `SOROBAN_NETWORK_PASSPHRASE`: Network passphrase

### Docker Registry
- `DOCKER_USERNAME`: Docker Hub username
- `DOCKER_PASSWORD`: Docker Hub password (or token)

### Deployment
- `HOST`: Production server host
- `USERNAME`: SSH username
- `SSH_KEY`: Private SSH key for deployment
- `API_URL`: Production API URL

### Monitoring
- `SLACK_WEBHOOK`: Slack webhook for notifications

## 🏗️ Build Process

### Smart Contracts
1. **Dependencies**: Cargo dependencies cached
2. **Linting**: `cargo fmt --check` + `cargo clippy`
3. **Testing**: `cargo test`
4. **Building**: `cargo build --target wasm32-unknown-unknown --release`
5. **Optimization**: `soroban contract optimize`

### API
1. **Dependencies**: Cargo dependencies cached
2. **Testing**: Unit + integration tests with Stellar testnet
3. **Building**: `cargo build --release`
4. **Docker**: Multi-stage build for optimized container
5. **OpenAPI**: Spec validation and documentation

## 🚀 Deployment Flow

### Staging Deployment
- Triggered on push to `develop`
- Deploys to testnet
- Runs smoke tests

### Production Deployment
- Triggered on push to `main` or manual dispatch
- Deploys to mainnet
- Requires manual approval
- Full health checks

## 🔒 Security Features

### Code Security
- **Cargo Audit**: Vulnerability scanning
- **Dependency Scanning**: Outdated packages check
- **Secrets Scanning**: TruffleHog integration

### Container Security
- **Distroless Base**: Minimal attack surface
- **Non-root User**: Security best practices
- **Health Checks**: Application monitoring

## 📊 Quality Gates

### Required Checks
- All tests pass
- Linting passes
- Security audit passes
- Code coverage > 80%

### Optional Checks
- Performance benchmarks
- Bundle size analysis
- Documentation updates

## 🐛 Troubleshooting

### Common Issues

**Cargo Build Failures:**
```bash
# Clear cache and rebuild
cargo clean
rm -rf ~/.cargo/registry
cargo build
```

**Soroban CLI Issues:**
```bash
# Reinstall Soroban CLI
cargo install --locked soroban-cli --features opt --force
```

**Docker Build Failures:**
```bash
# Clean Docker cache
docker system prune -a
docker builder prune
```

### Debug Commands

**Local Contract Testing:**
```bash
cd contracts/contracts
cargo test -- --nocapture
```

**Local API Testing:**
```bash
cd contracts/api
cargo test --test integration_tests -- --nocapture
```

**Manual Deployment Testing:**
```bash
# Test contract deployment
soroban contract deploy --wasm contract.wasm --source deployer --network testnet

# Test API deployment
docker build -t veriluxe-api .
docker run -p 3000:3000 veriluxe-api
```

## 📝 Workflow Customization

### Environment Variables
```yaml
env:
  CARGO_TERM_COLOR: always
  RUST_BACKTRACE: 1
  SOROBAN_NETWORK: testnet
```

### Job Dependencies
```yaml
jobs:
  test:
    needs: [lint]
  deploy:
    needs: [test, build]
```

### Conditional Execution
```yaml
if: github.event_name == 'push' && github.ref == 'refs/heads/main'
```

## 🔄 Maintenance

### Weekly Tasks
- Review dependency updates
- Check workflow performance
- Update base images
- Security patch review

### Monthly Tasks
- Workflow optimization
- Cache strategy review
- Secret rotation
- Performance analysis

## 📞 Support

For workflow issues:
1. Check workflow logs in GitHub Actions
2. Review this documentation
3. Contact DevOps team
4. Create issue in repository