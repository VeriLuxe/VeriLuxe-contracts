use anyhow::{anyhow, Result};
use std::env;

/// Application configuration
#[derive(Debug, Clone)]
pub struct Config {
    pub soroban_network_passphrase: String,
    pub soroban_rpc_url: String,
    pub fashion_auth_contract_id: String,
    pub admin_secret_key: String,
    pub api_host: String,
    pub api_port: u16,
}

impl Config {
    /// Load configuration from environment variables
    pub fn from_env() -> Result<Self> {
        // Try to load .env file from multiple locations
        println!("Attempting to load .env file...");
        if let Err(e) = dotenv::dotenv() {
            println!("Failed to load .env from current dir: {:?}", e);
            // Try loading from the parent directory (project root)
            if let Err(e2) = dotenv::from_filename("../../.env") {
                println!("Failed to load .env from ../../.env: {:?}", e2);
                // Try loading from the current directory
                if let Err(e3) = dotenv::from_filename(".env") {
                    println!("Failed to load .env from .env: {:?}", e3);
                } else {
                    println!("Successfully loaded .env from .env");
                }
            } else {
                println!("Successfully loaded .env from ../../.env");
            }
        } else {
            println!("Successfully loaded .env from dotenv()");
        }

        let soroban_network_passphrase = env::var("SOROBAN_NETWORK_PASSPHRASE")
            .unwrap_or_else(|_| "Test SDF Network ; September 2015".to_string());

        let soroban_rpc_url = env::var("SOROBAN_RPC_URL")
            .unwrap_or_else(|_| "https://soroban-testnet.stellar.org:443".to_string());

        let fashion_auth_contract_id = env::var("FASHION_AUTH_CONTRACT_ID")
            .map_err(|_| anyhow!("FASHION_AUTH_CONTRACT_ID environment variable is required"))?;

        let admin_secret_key = env::var("ADMIN_SECRET_KEY")
            .map_err(|_| anyhow!("ADMIN_SECRET_KEY environment variable is required"))?;

        let api_host = env::var("API_HOST")
            .unwrap_or_else(|_| "127.0.0.1".to_string());

        let api_port = env::var("API_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse::<u16>()
            .map_err(|_| anyhow!("Invalid API_PORT format"))?;

        Ok(Self {
            soroban_network_passphrase,
            soroban_rpc_url,
            fashion_auth_contract_id,
            admin_secret_key,
            api_host,
            api_port,
        })
    }

    /// Get the full API address
    pub fn api_address(&self) -> String {
        format!("{}:{}", self.api_host, self.api_port)
    }
}