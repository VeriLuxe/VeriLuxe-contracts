use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Certificate data structure matching the smart contract
#[derive(Debug, Clone, Serialize, Deserialize, ToSchema)]
pub struct Certificate {
    pub owner: String,
    pub metadata_hash: String,
    pub is_valid: bool,
}

/// Request body for initializing the contract
#[derive(Debug, Deserialize, ToSchema)]
pub struct InitRequest {
    pub admin_address: String,
}

/// Request body for issuing a certificate
#[derive(Debug, Deserialize, ToSchema)]
pub struct IssueCertificateRequest {
    pub cert_id: String,
    pub metadata_hash: String,
    pub owner_address: String,
}

/// Request body for verifying a certificate
#[derive(Debug, Deserialize, ToSchema)]
pub struct VerifyCertificateRequest {
    pub metadata_hash: String,
}

/// Request body for transferring a certificate
#[derive(Debug, Deserialize, ToSchema)]
pub struct TransferCertificateRequest {
    pub new_owner_address: String,
    pub current_owner_secret_key: String,
}

/// Response for successful operations
#[derive(Debug, Serialize, ToSchema)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub message: String,
}

/// Specific response types for OpenAPI documentation
#[derive(Debug, Serialize, ToSchema)]
pub struct HealthResponse {
    pub success: bool,
    pub data: Option<String>,
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct CertificateResponse {
    pub success: bool,
    pub data: Option<Certificate>,
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct TransactionApiResponse {
    pub success: bool,
    pub data: Option<TransactionResponse>,
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct VerifyApiResponse {
    pub success: bool,
    pub data: Option<VerifyResponse>,
    pub message: String,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct ExistsApiResponse {
    pub success: bool,
    pub data: Option<ExistsResponse>,
    pub message: String,
}

/// Response for verification operations
#[derive(Debug, Serialize, ToSchema)]
pub struct VerifyResponse {
    pub is_valid: bool,
    pub cert_id: String,
    pub metadata_hash: String,
}

/// Response for certificate existence check
#[derive(Debug, Serialize, ToSchema)]
pub struct ExistsResponse {
    pub exists: bool,
    pub cert_id: String,
}

/// Response for transaction operations
#[derive(Debug, Serialize, ToSchema)]
pub struct TransactionResponse {
    pub transaction_hash: String,
    pub status: String,
}

/// Error response structure
#[derive(Debug, Serialize, ToSchema)]
pub struct ErrorResponse {
    pub success: bool,
    pub error: String,
    pub code: u16,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T, message: String) -> Self {
        Self {
            success: true,
            data: Some(data),
            message,
        }
    }

    pub fn success_with_message(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: true,
            data: Some(()),
            message,
        }
    }

    pub fn error(message: String) -> ApiResponse<()> {
        ApiResponse {
            success: false,
            data: None,
            message,
        }
    }
}

impl ErrorResponse {
    pub fn new(error: String, code: u16) -> Self {
        Self {
            success: false,
            error,
            code,
        }
    }

    pub fn bad_request(error: String) -> Self {
        Self::new(error, 400)
    }

    pub fn not_found(error: String) -> Self {
        Self::new(error, 404)
    }

    pub fn internal_error(error: String) -> Self {
        Self::new(error, 500)
    }
}