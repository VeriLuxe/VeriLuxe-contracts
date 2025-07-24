mod config;
mod handlers;
mod models;
mod routes;
mod soroban_client;

use anyhow::Result;
use config::Config;
use handlers::AppState;
use routes::create_router;
use soroban_client::SorobanClient;
use tokio::net::TcpListener;
use tracing::{info, level_filters::LevelFilter};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::INFO.into())
                .from_env_lossy(),
        )
        .finish();

    tracing::subscriber::set_global_default(subscriber)
        .expect("Failed to set subscriber");

    // Load configuration
    let config = Config::from_env()?;
    info!("Loaded configuration successfully");

    // Initialize Soroban client
    let soroban_client = SorobanClient::new(
        config.soroban_rpc_url.clone(),
        config.soroban_network_passphrase.clone(),
        config.fashion_auth_contract_id.clone(),
        config.admin_secret_key.clone(),
    )?;
    info!("Initialized Soroban client");

    // Create application state
    let app_state = AppState { soroban_client };

    // Create router
    let app = create_router(app_state);

    // Create listener
    let listener = TcpListener::bind(&config.api_address()).await?;
    info!("API server listening on {}", config.api_address());

    // Start server
    axum::serve(listener, app).await?;

    Ok(())
}