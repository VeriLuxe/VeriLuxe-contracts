# VeriLuxe API

A REST API middleware for the VeriLuxe Fashion Authentication smart contract built on Soroban (Stellar).

## Overview

This API acts as a bridge between frontend applications and the Soroban smart contract, handling all complex blockchain interactions including transaction building, signing, and submission.

## Features

- **Contract Initialization**: Initialize the smart contract with an admin address
- **Certificate Management**: Issue, verify, transfer, and revoke authenticity certificates
- **RESTful Interface**: Simple HTTP endpoints for all smart contract functions
- **Error Handling**: Comprehensive error responses with proper HTTP status codes
- **CORS Support**: Cross-origin requests enabled for frontend integration
- **Logging**: Structured logging with configurable levels

## Prerequisites

- Rust 1.70 or later
- A deployed Soroban smart contract
- Access to a Soroban RPC endpoint (testnet or mainnet)
- Admin secret key for contract operations

## Installation

1. Clone the repository and navigate to the API directory:
```bash
cd contracts/api
```

2. Copy the environment configuration:
```bash
cp .env.example .env
```

3. Configure your environment variables in `.env`:
```env
SOROBAN_NETWORK_PASSPHRASE=Test SDF Network ; September 2015
SOROBAN_RPC_URL=https://soroban-testnet.stellar.org:443
FASHION_AUTH_CONTRACT_ID=your_contract_id_here
ADMIN_SECRET_KEY=your_admin_secret_key_here
API_HOST=127.0.0.1
API_PORT=3000
RUST_LOG=info
```

4. Install dependencies and build:
```bash
cargo build --release
```

## Running the API

### Development Mode
```bash
cargo run
```

### Production Mode
```bash
cargo run --release
```

The API will start on `http://127.0.0.1:3000` (or your configured host/port).

## API Endpoints

### Health Check
- **GET** `/health`
- Returns API health status

### Contract Initialization
- **POST** `/init`
- Initialize the smart contract with an admin address
- **Body**: `{"admin_address": "GXXXXXXX..."}`

### Certificate Management

#### Issue Certificate
- **POST** `/certificates`
- Issue a new authenticity certificate
- **Body**: 
```json
{
  "cert_id": "CERT001",
  "metadata_hash": "QmHash123...",
  "owner_address": "GXXXXXXX..."
}
```

#### Get Certificate Details
- **GET** `/certificates/:id`
- Retrieve certificate information by ID

#### Verify Certificate
- **POST** `/certificates/:id/verify`
- Verify certificate authenticity
- **Body**: `{"metadata_hash": "QmHash123..."}`

#### Transfer Certificate
- **POST** `/certificates/:id/transfer`
- Transfer certificate ownership
- **Body**: 
```json
{
  "new_owner_address": "GXXXXXXX...",
  "current_owner_secret_key": "secret_key_hex"
}
```

#### Revoke Certificate
- **POST** `/certificates/:id/revoke`
- Revoke a certificate (admin only)

#### Check Certificate Existence
- **GET** `/certificates/:id/exists`
- Check if a certificate exists

## Response Format

All responses follow a consistent format:

### Success Response
```json
{
  "success": true,
  "data": { ... },
  "message": "Operation completed successfully"
}
```

### Error Response
```json
{
  "success": false,
  "error": "Error description",
  "code": 400
}
```

## Testing

Run the test suite:
```bash
cargo test
```

Run integration tests:
```bash
cargo test --test integration_tests
```

## Example Usage

### Using curl

1. **Initialize Contract**:
```bash
curl -X POST http://localhost:3000/init \
  -H "Content-Type: application/json" \
  -d '{"admin_address": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"}'
```

2. **Issue Certificate**:
```bash
curl -X POST http://localhost:3000/certificates \
  -H "Content-Type: application/json" \
  -d '{
    "cert_id": "CERT001",
    "metadata_hash": "QmHash123456",
    "owner_address": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
  }'
```

3. **Verify Certificate**:
```bash
curl -X POST http://localhost:3000/certificates/CERT001/verify \
  -H "Content-Type: application/json" \
  -d '{"metadata_hash": "QmHash123456"}'
```

4. **Get Certificate Details**:
```bash
curl http://localhost:3000/certificates/CERT001
```

### Using JavaScript/TypeScript

```typescript
const API_BASE = 'http://localhost:3000';

// Issue a certificate
const issueCertificate = async (certData) => {
  const response = await fetch(`${API_BASE}/certificates`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify(certData)
  });
  
  return response.json();
};

// Verify a certificate
const verifyCertificate = async (certId, metadataHash) => {
  const response = await fetch(`${API_BASE}/certificates/${certId}/verify`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ metadata_hash: metadataHash })
  });
  
  return response.json();
};
```

## Configuration

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `SOROBAN_NETWORK_PASSPHRASE` | Network passphrase for transaction signing | `Test SDF Network ; September 2015` |
| `SOROBAN_RPC_URL` | Soroban RPC endpoint URL | `https://soroban-testnet.stellar.org:443` |
| `FASHION_AUTH_CONTRACT_ID` | Smart contract address | Required |
| `ADMIN_SECRET_KEY` | Admin secret key (hex format) | Required |
| `API_HOST` | API server host | `127.0.0.1` |
| `API_PORT` | API server port | `3000` |
| `RUST_LOG` | Logging level | `info` |

### Network Configuration

For **Testnet**:
```env
SOROBAN_NETWORK_PASSPHRASE=Test SDF Network ; September 2015
SOROBAN_RPC_URL=https://soroban-testnet.stellar.org:443
```

For **Mainnet**:
```env
SOROBAN_NETWORK_PASSPHRASE=Public Global Stellar Network ; September 2015
SOROBAN_RPC_URL=https://soroban-mainnet.stellar.org:443
```

## Security Considerations

- Keep your admin secret key secure and never commit it to version control
- Use environment variables for all sensitive configuration
- Consider implementing rate limiting for production deployments
- Validate all input data before processing
- Use HTTPS in production environments

## Development

### Project Structure
```
api/
├── src/
│   ├── main.rs              # Application entry point
│   ├── lib.rs               # Library exports
│   ├── config.rs            # Configuration management
│   ├── handlers.rs          # HTTP request handlers
│   ├── models.rs            # Data models and types
│   ├── routes.rs            # Route definitions
│   └── soroban_client.rs    # Soroban blockchain client
├── tests/
│   └── integration_tests.rs # Integration tests
├── Cargo.toml               # Dependencies and metadata
├── .env.example             # Example environment configuration
└── README.md                # This file
```

### Adding New Endpoints

1. Define the request/response models in `models.rs`
2. Implement the handler function in `handlers.rs`
3. Add the route in `routes.rs`
4. Add tests in `tests/integration_tests.rs`

## Troubleshooting

### Common Issues

1. **Contract Not Found**: Verify your `FASHION_AUTH_CONTRACT_ID` is correct
2. **Authentication Errors**: Check that your `ADMIN_SECRET_KEY` is valid
3. **Network Issues**: Ensure the `SOROBAN_RPC_URL` is accessible
4. **Transaction Failures**: Check Soroban logs for detailed error information

### Logging

Enable debug logging for more detailed information:
```bash
RUST_LOG=debug cargo run
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests for new functionality
5. Run the test suite
6. Submit a pull request

## License

This project is licensed under the MIT License - see the LICENSE file for details.