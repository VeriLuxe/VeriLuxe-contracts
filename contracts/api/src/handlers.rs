use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use tracing::{error, info};

use crate::{
    models::{
        ApiResponse, Certificate, ErrorResponse, ExistsResponse, InitRequest,
        IssueCertificateRequest, TransactionResponse, TransferCertificateRequest,
        VerifyCertificateRequest, VerifyResponse,
    },
    soroban_client::SorobanClient,
};

/// Application state containing the Soroban client
#[derive(Clone)]
pub struct AppState {
    pub soroban_client: SorobanClient,
}

/// Initialize the contract with admin
pub async fn init_contract(
    State(state): State<AppState>,
    Json(payload): Json<InitRequest>,
) -> Result<Json<ApiResponse<TransactionResponse>>, (StatusCode, Json<ErrorResponse>)> {
    info!("Initializing contract with admin: {}", payload.admin_address);

    match state.soroban_client.init(&payload.admin_address).await {
        Ok(tx_hash) => {
            let response = ApiResponse::success(
                TransactionResponse {
                    transaction_hash: tx_hash,
                    status: "submitted".to_string(),
                },
                "Contract initialized successfully".to_string(),
            );
            Ok(Json(response))
        }
        Err(e) => {
            error!("Failed to initialize contract: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(format!(
                    "Failed to initialize contract: {}",
                    e
                ))),
            ))
        }
    }
}

/// Issue a new certificate
pub async fn issue_certificate(
    State(state): State<AppState>,
    Json(payload): Json<IssueCertificateRequest>,
) -> Result<Json<ApiResponse<TransactionResponse>>, (StatusCode, Json<ErrorResponse>)> {
    info!("Issuing certificate: {}", payload.cert_id);

    // Validate input
    if payload.cert_id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(
                "Certificate ID cannot be empty".to_string(),
            )),
        ));
    }

    if payload.metadata_hash.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(
                "Metadata hash cannot be empty".to_string(),
            )),
        ));
    }

    if payload.owner_address.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(
                "Owner address cannot be empty".to_string(),
            )),
        ));
    }

    match state
        .soroban_client
        .issue_certificate(&payload.cert_id, &payload.metadata_hash, &payload.owner_address)
        .await
    {
        Ok(tx_hash) => {
            let response = ApiResponse::success(
                TransactionResponse {
                    transaction_hash: tx_hash,
                    status: "submitted".to_string(),
                },
                "Certificate issued successfully".to_string(),
            );
            Ok(Json(response))
        }
        Err(e) => {
            error!("Failed to issue certificate: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(format!(
                    "Failed to issue certificate: {}",
                    e
                ))),
            ))
        }
    }
}

/// Get certificate details by ID
pub async fn get_certificate(
    State(state): State<AppState>,
    Path(cert_id): Path<String>,
) -> Result<Json<ApiResponse<Certificate>>, (StatusCode, Json<ErrorResponse>)> {
    info!("Getting certificate details for: {}", cert_id);

    if cert_id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(
                "Certificate ID cannot be empty".to_string(),
            )),
        ));
    }

    match state.soroban_client.get_certificate_details(&cert_id).await {
        Ok(certificate) => {
            let response = ApiResponse::success(
                certificate,
                "Certificate details retrieved successfully".to_string(),
            );
            Ok(Json(response))
        }
        Err(e) => {
            error!("Failed to get certificate details: {}", e);
            if e.to_string().contains("not found") {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse::not_found(format!(
                        "Certificate {} not found",
                        cert_id
                    ))),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::internal_error(format!(
                        "Failed to get certificate details: {}",
                        e
                    ))),
                ))
            }
        }
    }
}

/// Verify a certificate by ID and metadata hash
pub async fn verify_certificate(
    State(state): State<AppState>,
    Path(cert_id): Path<String>,
    Json(payload): Json<VerifyCertificateRequest>,
) -> Result<Json<ApiResponse<VerifyResponse>>, (StatusCode, Json<ErrorResponse>)> {
    info!("Verifying certificate: {}", cert_id);

    if cert_id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(
                "Certificate ID cannot be empty".to_string(),
            )),
        ));
    }

    if payload.metadata_hash.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(
                "Metadata hash cannot be empty".to_string(),
            )),
        ));
    }

    match state
        .soroban_client
        .verify_certificate(&cert_id, &payload.metadata_hash)
        .await
    {
        Ok(is_valid) => {
            let response = ApiResponse::success(
                VerifyResponse {
                    is_valid,
                    cert_id: cert_id.clone(),
                    metadata_hash: payload.metadata_hash.clone(),
                },
                if is_valid {
                    "Certificate verification successful".to_string()
                } else {
                    "Certificate verification failed".to_string()
                },
            );
            Ok(Json(response))
        }
        Err(e) => {
            error!("Failed to verify certificate: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(format!(
                    "Failed to verify certificate: {}",
                    e
                ))),
            ))
        }
    }
}

/// Transfer certificate ownership
pub async fn transfer_certificate(
    State(state): State<AppState>,
    Path(cert_id): Path<String>,
    Json(payload): Json<TransferCertificateRequest>,
) -> Result<Json<ApiResponse<TransactionResponse>>, (StatusCode, Json<ErrorResponse>)> {
    info!("Transferring certificate: {}", cert_id);

    if cert_id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(
                "Certificate ID cannot be empty".to_string(),
            )),
        ));
    }

    if payload.new_owner_address.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(
                "New owner address cannot be empty".to_string(),
            )),
        ));
    }

    if payload.current_owner_secret_key.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(
                "Current owner secret key cannot be empty".to_string(),
            )),
        ));
    }

    match state
        .soroban_client
        .transfer_certificate(
            &cert_id,
            &payload.new_owner_address,
            &payload.current_owner_secret_key,
        )
        .await
    {
        Ok(tx_hash) => {
            let response = ApiResponse::success(
                TransactionResponse {
                    transaction_hash: tx_hash,
                    status: "submitted".to_string(),
                },
                "Certificate transferred successfully".to_string(),
            );
            Ok(Json(response))
        }
        Err(e) => {
            error!("Failed to transfer certificate: {}", e);
            if e.to_string().contains("not found") {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse::not_found(format!(
                        "Certificate {} not found",
                        cert_id
                    ))),
                ))
            } else if e.to_string().contains("invalid certificate") {
                Err((
                    StatusCode::BAD_REQUEST,
                    Json(ErrorResponse::bad_request(
                        "Cannot transfer invalid certificate".to_string(),
                    )),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::internal_error(format!(
                        "Failed to transfer certificate: {}",
                        e
                    ))),
                ))
            }
        }
    }
}

/// Revoke a certificate
pub async fn revoke_certificate(
    State(state): State<AppState>,
    Path(cert_id): Path<String>,
) -> Result<Json<ApiResponse<TransactionResponse>>, (StatusCode, Json<ErrorResponse>)> {
    info!("Revoking certificate: {}", cert_id);

    if cert_id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(
                "Certificate ID cannot be empty".to_string(),
            )),
        ));
    }

    match state.soroban_client.revoke_certificate(&cert_id).await {
        Ok(tx_hash) => {
            let response = ApiResponse::success(
                TransactionResponse {
                    transaction_hash: tx_hash,
                    status: "submitted".to_string(),
                },
                "Certificate revoked successfully".to_string(),
            );
            Ok(Json(response))
        }
        Err(e) => {
            error!("Failed to revoke certificate: {}", e);
            if e.to_string().contains("not found") {
                Err((
                    StatusCode::NOT_FOUND,
                    Json(ErrorResponse::not_found(format!(
                        "Certificate {} not found",
                        cert_id
                    ))),
                ))
            } else {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::internal_error(format!(
                        "Failed to revoke certificate: {}",
                        e
                    ))),
                ))
            }
        }
    }
}

/// Check if certificate exists
pub async fn check_certificate_exists(
    State(state): State<AppState>,
    Path(cert_id): Path<String>,
) -> Result<Json<ApiResponse<ExistsResponse>>, (StatusCode, Json<ErrorResponse>)> {
    info!("Checking if certificate exists: {}", cert_id);

    if cert_id.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse::bad_request(
                "Certificate ID cannot be empty".to_string(),
            )),
        ));
    }

    match state.soroban_client.certificate_exists(&cert_id).await {
        Ok(exists) => {
            let response = ApiResponse::success(
                ExistsResponse {
                    exists,
                    cert_id: cert_id.clone(),
                },
                if exists {
                    "Certificate exists".to_string()
                } else {
                    "Certificate does not exist".to_string()
                },
            );
            Ok(Json(response))
        }
        Err(e) => {
            error!("Failed to check certificate existence: {}", e);
            Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::internal_error(format!(
                    "Failed to check certificate existence: {}",
                    e
                ))),
            ))
        }
    }
}

/// Health check endpoint
pub async fn health_check() -> Json<ApiResponse<String>> {
    Json(ApiResponse::success(
        "healthy".to_string(),
        "API is running".to_string(),
    ))
}