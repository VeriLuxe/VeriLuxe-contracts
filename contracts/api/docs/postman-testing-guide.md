# Postman Testing Guide for VeriLuxe API

This comprehensive guide will help you test the VeriLuxe REST API using Postman. It includes setup instructions, environment configuration, and detailed examples for each endpoint.

## 📋 Table of Contents

- [Setup & Configuration](#setup--configuration)
- [Environment Variables](#environment-variables)
- [Authentication](#authentication)
- [API Endpoints](#api-endpoints)
- [Testing Workflow](#testing-workflow)
- [Error Handling](#error-handling)
- [Tips & Best Practices](#tips--best-practices)

## 🚀 Setup & Configuration

### 1. Install Postman

Download and install [Postman](https://www.postman.com/downloads/) if you haven't already.

### 2. Import Collection

You can create a new collection or import the provided collection file:

1. Open Postman
2. Click "Import" 
3. Create a new collection named "VeriLuxe API"
4. Set the base URL as a collection variable

### 3. Start the API Server

Before testing, ensure the VeriLuxe API is running:

```bash
cd contracts/api
cargo run
```

The API should be available at `http://localhost:3000`

## 🔧 Environment Variables

Set up environment variables in Postman for easier testing:

### Create Environment

1. Click on "Environments" in Postman
2. Create a new environment called "VeriLuxe Local"
3. Add the following variables:

| Variable | Initial Value | Current Value |
|----------|---------------|---------------|
| `baseUrl` | `http://localhost:3000` | `http://localhost:3000` |
| `contractId` | `CXXXXXXXXXXXXXXX` | `CXXXXXXXXXXXXXXX` |
| `adminSecretKey` | `SXXXXXXXXXXXXXXX` | `SXXXXXXXXXXXXXXX` |
| `adminAddress` | `GXXXXXXXXXXXXXXX` | `GXXXXXXXXXXXXXXX` |
| `testCertId` | `CERT001` | `CERT001` |
| `testMetadataHash` | `QmExampleHash123456789` | `QmExampleHash123456789` |
| `testOwnerAddress` | `GXXXXXXXXXXXXXXX` | `GXXXXXXXXXXXXXXX` |

### For Production Testing

Create another environment "VeriLuxe Production":

| Variable | Value |
|----------|-------|
| `baseUrl` | `https://api.veriluxe.com/v1` |
| `contractId` | `Your production contract ID` |

## 🔐 Authentication

Currently, the API uses environment-based authentication. Future versions may include:

- Bearer tokens
- API keys
- Signature-based authentication

For now, ensure your `.env` file in the API directory contains the correct admin secret key.

## 📡 API Endpoints

### 1. Health Check

**Purpose**: Verify API server status

**Method**: `GET`  
**URL**: `{{baseUrl}}/health`

#### Postman Setup:
- **Method**: GET
- **URL**: `{{baseUrl}}/health`
- **Headers**: None required

#### Example Response:
```json
{
  "success": true,
  "data": {
    "status": "healthy",
    "uptime": "2h15m30s",
    "version": "1.0.0",
    "blockchain_status": "connected",
    "last_block": 12345678
  },
  "message": "API is healthy",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### 2. Initialize Contract

**Purpose**: Initialize the smart contract with an admin address (one-time operation)

**Method**: `POST`  
**URL**: `{{baseUrl}}/init`

#### Postman Setup:
- **Method**: POST
- **URL**: `{{baseUrl}}/init`
- **Headers**: 
  ```
  Content-Type: application/json
  ```
- **Body** (raw JSON):
  ```json
  {
    "admin_address": "{{adminAddress}}"
  }
  ```

#### Example Response:
```json
{
  "success": true,
  "data": {
    "transaction_id": "abc123def456ghi789",
    "admin_address": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "contract_id": "CXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
  },
  "message": "Contract initialized successfully",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### 3. Issue Certificate

**Purpose**: Create a new authenticity certificate (admin only)

**Method**: `POST`  
**URL**: `{{baseUrl}}/certificates`

#### Postman Setup:
- **Method**: POST
- **URL**: `{{baseUrl}}/certificates`
- **Headers**: 
  ```
  Content-Type: application/json
  ```
- **Body** (raw JSON):
  ```json
  {
    "cert_id": "{{testCertId}}",
    "metadata_hash": "{{testMetadataHash}}",
    "owner_address": "{{testOwnerAddress}}"
  }
  ```

#### Example Response:
```json
{
  "success": true,
  "data": {
    "certificate": {
      "id": "CERT001",
      "owner": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
      "metadata_hash": "QmExampleHash123456789",
      "is_valid": true,
      "created_at": "2024-01-15T10:30:00Z"
    },
    "transaction_id": "def456ghi789jkl012"
  },
  "message": "Certificate issued successfully",
  "timestamp": "2024-01-15T10:30:00Z"
}
```

### 4. Get Certificate Details

**Purpose**: Retrieve detailed information about a specific certificate

**Method**: `GET`  
**URL**: `{{baseUrl}}/certificates/{{testCertId}}`

#### Postman Setup:
- **Method**: GET
- **URL**: `{{baseUrl}}/certificates/{{testCertId}}`
- **Headers**: None required

#### Example Response:
```json
{
  "success": true,
  "data": {
    "certificate": {
      "id": "CERT001",
      "owner": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
      "metadata_hash": "QmExampleHash123456789",
      "is_valid": true,
      "created_at": "2024-01-15T10:30:00Z",
      "updated_at": "2024-01-15T10:30:00Z"
    },
    "transaction_history": [
      {
        "type": "issued",
        "timestamp": "2024-01-15T10:30:00Z",
        "transaction_id": "abc123def456",
        "from": null,
        "to": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
      }
    ]
  },
  "message": "Certificate details retrieved successfully",
  "timestamp": "2024-01-15T10:35:00Z"
}
```

### 5. Verify Certificate

**Purpose**: Verify the authenticity of a certificate

**Method**: `POST`  
**URL**: `{{baseUrl}}/certificates/{{testCertId}}/verify`

#### Postman Setup:
- **Method**: POST
- **URL**: `{{baseUrl}}/certificates/{{testCertId}}/verify`
- **Headers**: 
  ```
  Content-Type: application/json
  ```
- **Body** (raw JSON):
  ```json
  {
    "metadata_hash": "{{testMetadataHash}}"
  }
  ```

#### Example Response (Valid):
```json
{
  "success": true,
  "data": {
    "is_valid": true,
    "certificate": {
      "id": "CERT001",
      "owner": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
      "metadata_hash": "QmExampleHash123456789",
      "is_valid": true
    },
    "verification_timestamp": "2024-01-15T10:40:00Z"
  },
  "message": "Certificate is valid and authentic",
  "timestamp": "2024-01-15T10:40:00Z"
}
```

#### Example Response (Invalid):
```json
{
  "success": false,
  "error": {
    "code": "INVALID_METADATA_HASH",
    "message": "Metadata hash does not match certificate",
    "details": "Provided hash: QmWrongHash, Expected: QmExampleHash123456789"
  },
  "timestamp": "2024-01-15T10:40:00Z"
}
```

### 6. Transfer Certificate

**Purpose**: Transfer ownership of a certificate to another address

**Method**: `POST`  
**URL**: `{{baseUrl}}/certificates/{{testCertId}}/transfer`

#### Postman Setup:
- **Method**: POST
- **URL**: `{{baseUrl}}/certificates/{{testCertId}}/transfer`
- **Headers**: 
  ```
  Content-Type: application/json
  ```
- **Body** (raw JSON):
  ```json
  {
    "new_owner_address": "GYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY",
    "current_owner_secret_key": "SXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
  }
  ```

#### Example Response:
```json
{
  "success": true,
  "data": {
    "certificate": {
      "id": "CERT001",
      "owner": "GYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY",
      "metadata_hash": "QmExampleHash123456789",
      "is_valid": true
    },
    "transaction_id": "ghi789jkl012mno345",
    "previous_owner": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
    "new_owner": "GYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY"
  },
  "message": "Certificate transferred successfully",
  "timestamp": "2024-01-15T10:45:00Z"
}
```

### 7. Revoke Certificate

**Purpose**: Revoke a certificate, marking it as invalid (admin only)

**Method**: `POST`  
**URL**: `{{baseUrl}}/certificates/{{testCertId}}/revoke`

#### Postman Setup:
- **Method**: POST
- **URL**: `{{baseUrl}}/certificates/{{testCertId}}/revoke`
- **Headers**: 
  ```
  Content-Type: application/json
  ```
- **Body**: None required (admin authentication handled by environment)

#### Example Response:
```json
{
  "success": true,
  "data": {
    "certificate": {
      "id": "CERT001",
      "owner": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
      "metadata_hash": "QmExampleHash123456789",
      "is_valid": false
    },
    "transaction_id": "jkl012mno345pqr678",
    "revoked_at": "2024-01-15T12:00:00Z"
  },
  "message": "Certificate revoked successfully",
  "timestamp": "2024-01-15T12:00:00Z"
}
```

### 8. Check Certificate Existence

**Purpose**: Check if a certificate exists without revealing details

**Method**: `GET`  
**URL**: `{{baseUrl}}/certificates/{{testCertId}}/exists`

#### Postman Setup:
- **Method**: GET
- **URL**: `{{baseUrl}}/certificates/{{testCertId}}/exists`
- **Headers**: None required

#### Example Response:
```json
{
  "success": true,
  "data": {
    "exists": true,
    "cert_id": "CERT001"
  },
  "message": "Certificate existence checked",
  "timestamp": "2024-01-15T10:50:00Z"
}
```

### 9. Batch Verification

**Purpose**: Verify multiple certificates in a single request

**Method**: `POST`  
**URL**: `{{baseUrl}}/certificates/batch/verify`

#### Postman Setup:
- **Method**: POST
- **URL**: `{{baseUrl}}/certificates/batch/verify`
- **Headers**: 
  ```
  Content-Type: application/json
  ```
- **Body** (raw JSON):
  ```json
  {
    "certificates": [
      {
        "cert_id": "CERT001",
        "metadata_hash": "QmExampleHash123456789"
      },
      {
        "cert_id": "CERT002",
        "metadata_hash": "QmAnotherHash987654321"
      },
      {
        "cert_id": "CERT003",
        "metadata_hash": "QmThirdHash555666777"
      }
    ]
  }
  ```

#### Example Response:
```json
{
  "success": true,
  "data": {
    "results": [
      {
        "cert_id": "CERT001",
        "is_valid": true,
        "message": "Valid certificate"
      },
      {
        "cert_id": "CERT002",
        "is_valid": false,
        "message": "Certificate not found"
      },
      {
        "cert_id": "CERT003",
        "is_valid": false,
        "message": "Metadata hash mismatch"
      }
    ],
    "summary": {
      "total": 3,
      "valid": 1,
      "invalid": 2
    }
  },
  "message": "Batch verification completed",
  "timestamp": "2024-01-15T10:55:00Z"
}
```

### 10. Get Statistics

**Purpose**: Retrieve platform-wide statistics

**Method**: `GET`  
**URL**: `{{baseUrl}}/statistics`

#### Postman Setup:
- **Method**: GET
- **URL**: `{{baseUrl}}/statistics`
- **Headers**: None required

#### Example Response:
```json
{
  "success": true,
  "data": {
    "total_certificates": 1247,
    "active_certificates": 1190,
    "revoked_certificates": 57,
    "total_transfers": 2384,
    "unique_owners": 892,
    "last_updated": "2024-01-15T10:30:00Z"
  },
  "message": "Statistics retrieved successfully",
  "timestamp": "2024-01-15T11:00:00Z"
}
```

## 🔄 Testing Workflow

### Complete Testing Sequence

Follow this sequence to test the complete certificate lifecycle:

1. **Health Check** - Verify API is running
2. **Initialize Contract** - Set up admin (one-time only)
3. **Issue Certificate** - Create a test certificate
4. **Get Certificate Details** - Verify certificate was created
5. **Verify Certificate** - Test authentication
6. **Transfer Certificate** - Test ownership change
7. **Verify Again** - Confirm new ownership
8. **Revoke Certificate** - Test admin revocation
9. **Verify Revoked** - Confirm certificate is invalid
10. **Check Statistics** - View platform metrics

### Test Data Preparation

Before testing, prepare these test values:

```javascript
// Test Certificate Data
const testData = {
  certId: "CERT_TEST_" + Date.now(),
  metadataHash: "QmTestHash" + Math.random().toString(36).substr(2, 9),
  ownerAddress: "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX",
  newOwnerAddress: "GYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYYY"
};
```

## ❌ Error Handling

### Common Error Responses

#### 400 Bad Request
```json
{
  "success": false,
  "error": {
    "code": "VALIDATION_ERROR",
    "message": "Invalid request data",
    "details": "Missing required field: cert_id"
  },
  "timestamp": "2024-01-15T10:30:00Z"
}
```

#### 404 Not Found
```json
{
  "success": false,
  "error": {
    "code": "CERTIFICATE_NOT_FOUND",
    "message": "Certificate with ID 'CERT999' does not exist",
    "details": null
  },
  "timestamp": "2024-01-15T10:30:00Z"
}
```

#### 401 Unauthorized
```json
{
  "success": false,
  "error": {
    "code": "UNAUTHORIZED",
    "message": "Insufficient permissions for this operation",
    "details": "Admin access required"
  },
  "timestamp": "2024-01-15T10:30:00Z"
}
```

#### 500 Internal Server Error
```json
{
  "success": false,
  "error": {
    "code": "INTERNAL_ERROR",
    "message": "An internal server error occurred",
    "details": "Contact support if this persists"
  },
  "timestamp": "2024-01-15T10:30:00Z"
}
```

## 💡 Tips & Best Practices

### 1. Environment Management

- Use different environments for testing (local, staging, production)
- Keep sensitive data (secret keys) secure
- Use environment variables for all configurable values

### 2. Request Organization

- Group related requests in folders
- Use descriptive names for requests
- Add detailed descriptions for each endpoint

### 3. Testing Strategies

- **Positive Testing**: Test with valid data
- **Negative Testing**: Test with invalid data
- **Edge Cases**: Test boundary conditions
- **Error Scenarios**: Test error handling

### 4. Data Management

- Use unique certificate IDs for each test
- Clean up test data when possible
- Use meaningful test data that's easy to identify

### 5. Automation

Consider creating test scripts for repeated testing:

```javascript
// Pre-request Script Example
pm.globals.set("timestamp", Date.now());
pm.globals.set("testCertId", "CERT_AUTO_" + pm.globals.get("timestamp"));
```

### 6. Response Validation

Add tests to validate responses:

```javascript
// Test Script Example
pm.test("Status code is 200", function () {
    pm.response.to.have.status(200);
});

pm.test("Response has success field", function () {
    pm.expect(pm.response.json()).to.have.property('success');
});

pm.test("Certificate ID matches request", function () {
    const response = pm.response.json();
    pm.expect(response.data.certificate.id).to.eql(pm.globals.get("testCertId"));
});
```

## 📥 Postman Collection Export

You can create a complete Postman collection with all these endpoints and share it with your team. The collection should include:

- All endpoint configurations
- Environment variables
- Pre-request scripts
- Test scripts
- Documentation for each endpoint

## 🔗 Additional Resources

- [VeriLuxe API Documentation](../contracts/api/README.md)
- [Smart Contract Documentation](../contracts/contracts/README.md)
- [Postman Learning Center](https://learning.postman.com/)
- [REST API Best Practices](https://restfulapi.net/)

---

**Happy Testing! 🚀**

For issues or questions about API testing, please refer to the [troubleshooting guide](troubleshooting/common-issues.md) or contact the development team.