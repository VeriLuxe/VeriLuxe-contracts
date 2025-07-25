use axum::{
    routing::{get, post},
    Router,
};
use tower_http::cors::CorsLayer;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

use crate::{
    handlers::{
        check_certificate_exists, get_certificate, health_check, init_contract, issue_certificate,
        revoke_certificate, transfer_certificate, verify_certificate, AppState, ApiDoc,
    },
};

/// Create the application router with all endpoints
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Health check
        .route("/health", get(health_check))
        
        // Contract initialization
        .route("/init", post(init_contract))
        
        // Certificate management
        .route("/certificates", post(issue_certificate))
        .route("/certificates/:id", get(get_certificate))
        .route("/certificates/:id/verify", post(verify_certificate))
        .route("/certificates/:id/transfer", post(transfer_certificate))
        .route("/certificates/:id/revoke", post(revoke_certificate))
        .route("/certificates/:id/exists", get(check_certificate_exists))
        
        // Swagger UI
        .merge(SwaggerUi::new("/swagger-ui")
            .url("/api-docs/openapi.json", ApiDoc::openapi()))
        
        // Add CORS middleware
        .layer(CorsLayer::permissive())
        
        // Add application state
        .with_state(state)
}