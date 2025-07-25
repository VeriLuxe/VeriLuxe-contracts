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

/// Opens the Swagger UI URL in the default browser
fn open_browser(url: &str) -> Result<()> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("cmd")
            .args(["/c", "start", url])
            .spawn()?;
    }
    
    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(url)
            .spawn()?;
    }
    
    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(url)
            .spawn()?;
    }
    
    Ok(())
}

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
    let server_url = format!("http://{}", config.api_address());
    let swagger_url = format!("{}/swagger-ui", server_url);
    
    info!("API server listening on {}", config.api_address());
    info!("Swagger UI available at: {}", swagger_url);
    
    // Auto-open Swagger UI in browser
    if let Err(e) = open_browser(&swagger_url) {
        tracing::warn!("Failed to open browser: {}", e);
    }

    // Start server
    axum::serve(listener, app).await?;

    Ok(())
}