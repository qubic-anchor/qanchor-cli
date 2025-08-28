//! Error types for Qubic RPC client

use thiserror::Error;

/// Qubic RPC client error types
#[derive(Error, Debug)]
pub enum QubicRpcError {
    /// HTTP request error
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// Base64 encoding/decoding error
    #[error("Base64 error: {0}")]
    Base64(#[from] base64::DecodeError),

    /// Invalid network configuration
    #[error("Invalid network configuration: {0}")]
    InvalidNetwork(String),

    /// RPC server error
    #[error("RPC server error: {0}")]
    ServerError(String),

    /// Invalid response format
    #[error("Invalid response format: {0}")]
    InvalidResponse(String),

    /// Transaction error
    #[error("Transaction error: {0}")]
    Transaction(String),

    /// Smart contract error
    #[error("Smart contract error: {0}")]
    SmartContract(String),

    /// Cryptographic error
    #[error("Cryptographic error: {0}")]
    Crypto(String),

    /// Timeout error
    #[error("Request timeout")]
    Timeout,

    /// Generic error
    #[error("Qubic RPC error: {0}")]
    Other(String),
}

/// Result type alias for Qubic RPC operations
pub type Result<T> = std::result::Result<T, QubicRpcError>;

impl QubicRpcError {
    /// Create a new server error
    pub fn server_error(message: impl Into<String>) -> Self {
        Self::ServerError(message.into())
    }

    /// Create a new invalid response error
    pub fn invalid_response(message: impl Into<String>) -> Self {
        Self::InvalidResponse(message.into())
    }

    /// Create a new transaction error
    pub fn transaction_error(message: impl Into<String>) -> Self {
        Self::Transaction(message.into())
    }

    /// Create a new smart contract error
    pub fn smart_contract_error(message: impl Into<String>) -> Self {
        Self::SmartContract(message.into())
    }

    /// Create a new crypto error
    pub fn crypto_error(message: impl Into<String>) -> Self {
        Self::Crypto(message.into())
    }

    /// Create a new generic error
    pub fn other(message: impl Into<String>) -> Self {
        Self::Other(message.into())
    }
}
