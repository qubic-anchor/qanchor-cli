//! # Qubic RPC Client
//! 
//! A Rust client library for interacting with the Qubic blockchain RPC API.
//! 
//! Based on the official [Qubic RPC documentation](https://docs.qubic.org/api/rpc).
//! 
//! ## Features
//! 
//! - Full RPC API coverage for Qubic blockchain
//! - Smart contract interaction support
//! - Transaction broadcasting
//! - Base64 encoding/decoding for data
//! - Multiple network support (mainnet, testnet, staging)
//! 
//! ## Example
//! 
//! ```rust,no_run
//! use qubic_rpc::{QubicRpcClient, Network};
//! 
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     let client = QubicRpcClient::new(Network::Mainnet)?;
//!     let status = client.get_status().await?;
//!     println!(
//!         "Current tick: {}",
//!         status.last_processed_tick.tick_number
//!     );
//!     Ok(())
//! }
//! ```

pub mod client;
pub mod types;
pub mod crypto;
pub mod error;
pub mod wallet;
pub mod retry;

// Re-export main types
pub use client::QubicRpcClient;
pub use types::*;
pub use error::{QubicRpcError, Result};
pub use wallet::{QubicWallet, TransactionBuilder};
pub use retry::{RetryConfig, NetworkHealthChecker, NetworkHealth, HealthStatus, with_retry};

/// Qubic network endpoints
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Network {
    /// Production mainnet: https://rpc.qubic.org
    Mainnet,
    /// Test network: https://testnet-rpc.qubic.org  
    Testnet,
    /// Staging environment: https://rpc-staging.qubic.org
    Staging,
}

impl Network {
    /// Get the base URL for this network
    pub fn base_url(&self) -> &'static str {
        match self {
            Network::Mainnet => "https://rpc.qubic.org",
            Network::Testnet => "https://testnet-rpc.qubic.org", 
            Network::Staging => "https://rpc-staging.qubic.org",
        }
    }

    /// Get the API version for this network
    pub fn api_version(&self) -> &'static str {
        match self {
            Network::Mainnet | Network::Testnet => "v1",
            Network::Staging => "v2",
        }
    }

    /// Get the full API base URL
    pub fn api_base_url(&self) -> String {
        format!("{}/{}", self.base_url(), self.api_version())
    }
}
