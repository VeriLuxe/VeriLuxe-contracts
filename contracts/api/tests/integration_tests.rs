use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::Response,
};
use serde_json::{json, Value};
use tokio_test;
use tower::ServiceExt;
use veriluxe_api::{
    config::Config,
    handlers::AppState,
    routes::create_router,
    soroban_client::SorobanClient,
};

async fn create_test_app() -> Result<axum::Router, Box<dyn std::error::Error>> {
    // Use test configuration
    let config = Config {
        soroban_network_passphrase: "Test SDF Network ; September 2015".to_string(),
        soroban_rpc_url: "https://soroban-testnet.stellar.org:443".to_string(),
        fashion_auth_contract_id: "test_contract_id".to_string(),
        admin_secret_key: "test_admin_secret_key".to_string(),
        api_host: "127.0.0.1".to_string(),
        api_port: 3000,
    };

    // Create mock Soroban client (this would need proper mocking in a real test)
    let soroban_client = SorobanClient::new(
        config.soroban_rpc_url,
        config.soroban_network_passphrase,
        config.fashion_auth_contract_id,
        config.admin_secret_key,
    )?;

    let app_state = AppState { soroban_client };
    Ok(create_router(app_state))
}

#[tokio::test]
async fn test_health_check() {
    let app = create_test_app().await.expect("Failed to create test app");

    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body_json["success"], true);
    assert_eq!(body_json["data"], "healthy");
}

#[tokio::test]
async fn test_init_contract_missing_admin() {
    let app = create_test_app().await.expect("Failed to create test app");

    let request_body = json!({});

    let request = Request::builder()
        .method("POST")
        .uri("/init")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // This should return a 422 or 400 status code for missing required field
    assert!(response.status().is_client_error());
}

#[tokio::test]
async fn test_issue_certificate_validation() {
    let app = create_test_app().await.expect("Failed to create test app");

    // Test with empty cert_id
    let request_body = json!({
        "cert_id": "",
        "metadata_hash": "QmHash123",
        "owner_address": "GXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/certificates")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body_json["success"], false);
    assert!(body_json["error"]
        .as_str()
        .unwrap()
        .contains("Certificate ID cannot be empty"));
}

#[tokio::test]
async fn test_get_certificate_empty_id() {
    let app = create_test_app().await.expect("Failed to create test app");

    let request = Request::builder()
        .uri("/certificates/")
        .body(Body::empty())
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    // This should return 404 as the route doesn't exist
    assert_eq!(response.status(), StatusCode::NOT_FOUND);
}

#[tokio::test]
async fn test_verify_certificate_validation() {
    let app = create_test_app().await.expect("Failed to create test app");

    // Test with empty metadata_hash
    let request_body = json!({
        "metadata_hash": ""
    });

    let request = Request::builder()
        .method("POST")
        .uri("/certificates/CERT001/verify")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body_json["success"], false);
    assert!(body_json["error"]
        .as_str()
        .unwrap()
        .contains("Metadata hash cannot be empty"));
}

#[tokio::test]
async fn test_transfer_certificate_validation() {
    let app = create_test_app().await.expect("Failed to create test app");

    // Test with empty new_owner_address
    let request_body = json!({
        "new_owner_address": "",
        "current_owner_secret_key": "test_secret_key"
    });

    let request = Request::builder()
        .method("POST")
        .uri("/certificates/CERT001/transfer")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_vec(&request_body).unwrap()))
        .unwrap();

    let response = app.oneshot(request).await.unwrap();

    assert_eq!(response.status(), StatusCode::BAD_REQUEST);

    let body = axum::body::to_bytes(response.into_body(), usize::MAX)
        .await
        .unwrap();
    let body_json: Value = serde_json::from_slice(&body).unwrap();

    assert_eq!(body_json["success"], false);
    assert!(body_json["error"]
        .as_str()
        .unwrap()
        .contains("New owner address cannot be empty"));
}