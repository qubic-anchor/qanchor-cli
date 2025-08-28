//! Retry strategies and network resilience utilities
//! 
//! Provides retry mechanisms for handling network instability and node unavailability

use crate::error::{QubicRpcError, Result};
use std::time::Duration;
use tokio::time::sleep;

/// Retry configuration for network operations
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: u32,
    /// Initial delay between retries
    pub initial_delay: Duration,
    /// Maximum delay between retries
    pub max_delay: Duration,
    /// Multiplier for exponential backoff
    pub backoff_multiplier: f32,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: 3,
            initial_delay: Duration::from_millis(500),
            max_delay: Duration::from_secs(10),
            backoff_multiplier: 2.0,
        }
    }
}

impl RetryConfig {
    /// Create a conservative retry config for production use
    pub fn conservative() -> Self {
        Self {
            max_attempts: 2,
            initial_delay: Duration::from_secs(1),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 1.5,
        }
    }

    /// Create an aggressive retry config for development
    pub fn aggressive() -> Self {
        Self {
            max_attempts: 5,
            initial_delay: Duration::from_millis(200),
            max_delay: Duration::from_secs(15),
            backoff_multiplier: 2.5,
        }
    }

    /// Create config with no retries
    pub fn no_retry() -> Self {
        Self {
            max_attempts: 1,
            initial_delay: Duration::from_millis(0),
            max_delay: Duration::from_millis(0),
            backoff_multiplier: 1.0,
        }
    }
}

/// Retry a future with exponential backoff
pub async fn with_retry<F, Fut, T>(
    operation: F,
    config: &RetryConfig,
) -> Result<T>
where
    F: Fn() -> Fut,
    Fut: std::future::Future<Output = Result<T>>,
{
    let mut attempt = 1;
    let mut delay = config.initial_delay;

    loop {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                // Check if we should retry based on error type
                if !should_retry(&e) || attempt >= config.max_attempts {
                    return Err(e);
                }

                // Log retry attempt
                log::warn!(
                    "Attempt {} failed: {}. Retrying in {:?}...", 
                    attempt, e, delay
                );

                // Wait before retry
                sleep(delay).await;

                // Update for next attempt
                attempt += 1;
                delay = std::cmp::min(
                    Duration::from_millis((delay.as_millis() as f32 * config.backoff_multiplier) as u64),
                    config.max_delay,
                );
            }
        }
    }
}

/// Determine if an error is retryable
fn should_retry(error: &QubicRpcError) -> bool {
    match error {
        // Network errors - usually retryable
        QubicRpcError::Http(_) => true,
        QubicRpcError::Timeout => true,
        
        // Server errors that might be temporary
        QubicRpcError::ServerError(msg) => {
            // HTTP 521 (web server down) is retryable
            msg.contains("521") ||
            // HTTP 502/503 (bad gateway/service unavailable) are retryable
            msg.contains("502") || msg.contains("503") ||
            // Connection errors are retryable
            msg.contains("connection") || msg.contains("timeout")
        },
        
        // Client errors - usually not retryable
        QubicRpcError::InvalidResponse(_) => false,
        QubicRpcError::Json(_) => false,
        QubicRpcError::Base64(_) => false,
        QubicRpcError::InvalidNetwork(_) => false,
        QubicRpcError::Transaction(_) => false,
        QubicRpcError::SmartContract(_) => false,
        QubicRpcError::Crypto(_) => false,
        QubicRpcError::Other(_) => false,
    }
}

/// Network health checker
pub struct NetworkHealthChecker {
    config: RetryConfig,
}

impl NetworkHealthChecker {
    pub fn new(config: RetryConfig) -> Self {
        Self { config }
    }

    /// Check if a network endpoint is healthy
    pub async fn check_health(&self, client: &crate::QubicRpcClient) -> Result<NetworkHealth> {
        let start_time = std::time::Instant::now();
        
        // Try to ping the network
        let ping_result = with_retry(
            || client.ping(),
            &self.config,
        ).await;

        let response_time = start_time.elapsed();

        match ping_result {
            Ok(ping_time) => {
                let health_status = if ping_time.as_millis() < 1000 {
                    HealthStatus::Healthy
                } else if ping_time.as_millis() < 5000 {
                    HealthStatus::Degraded
                } else {
                    HealthStatus::Slow
                };

                Ok(NetworkHealth {
                    status: health_status,
                    response_time,
                    last_check: chrono::Utc::now(),
                    error: None,
                })
            }
            Err(e) => {
                Ok(NetworkHealth {
                    status: HealthStatus::Unhealthy,
                    response_time,
                    last_check: chrono::Utc::now(),
                    error: Some(e.to_string()),
                })
            }
        }
    }
}

/// Network health status
#[derive(Debug, Clone, PartialEq)]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Slow,
    Unhealthy,
}

/// Network health information
#[derive(Debug, Clone)]
pub struct NetworkHealth {
    pub status: HealthStatus,
    pub response_time: Duration,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub error: Option<String>,
}

impl NetworkHealth {
    /// Check if the network is usable for operations
    pub fn is_usable(&self) -> bool {
        matches!(self.status, HealthStatus::Healthy | HealthStatus::Degraded | HealthStatus::Slow)
    }

    /// Get a human-readable status description
    pub fn status_description(&self) -> &'static str {
        match self.status {
            HealthStatus::Healthy => "Network is healthy and responsive",
            HealthStatus::Degraded => "Network is functional but with reduced performance",
            HealthStatus::Slow => "Network is slow but operational",
            HealthStatus::Unhealthy => "Network is not available or not responding",
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_retry_config_defaults() {
        let config = RetryConfig::default();
        assert_eq!(config.max_attempts, 3);
        assert_eq!(config.initial_delay, Duration::from_millis(500));
    }

    #[test]
    fn test_should_retry_logic() {
        // Retryable errors
        assert!(should_retry(&QubicRpcError::Timeout));
        assert!(should_retry(&QubicRpcError::server_error("521 Web Server Is Down")));
        assert!(should_retry(&QubicRpcError::server_error("502 Bad Gateway")));
        
        // Non-retryable errors
        assert!(!should_retry(&QubicRpcError::invalid_response("bad json")));
        assert!(!should_retry(&QubicRpcError::crypto_error("invalid key")));
    }

    #[tokio::test]
    async fn test_retry_with_success() {
        use std::sync::{Arc, Mutex};
        
        let config = RetryConfig::no_retry();
        let call_count = Arc::new(Mutex::new(0));
        let call_count_clone = call_count.clone();
        
        let result = with_retry(
            || {
                let count = call_count_clone.clone();
                async move {
                    let mut call_count = count.lock().unwrap();
                    *call_count += 1;
                    Ok::<i32, QubicRpcError>(42)
                }
            },
            &config,
        ).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(*call_count.lock().unwrap(), 1);
    }

    #[tokio::test]
    async fn test_retry_with_eventual_success() {
        use std::sync::{Arc, Mutex};
        
        let config = RetryConfig {
            max_attempts: 3,
            initial_delay: Duration::from_millis(1),
            max_delay: Duration::from_millis(10),
            backoff_multiplier: 2.0,
        };
        
        let call_count = Arc::new(Mutex::new(0));
        let call_count_clone = call_count.clone();
        
        let result = with_retry(
            || {
                let count = call_count_clone.clone();
                async move {
                    let mut call_count = count.lock().unwrap();
                    *call_count += 1;
                    let current_count = *call_count;
                    drop(call_count); // Release lock before potential error
                    
                    if current_count < 3 {
                        Err(QubicRpcError::Timeout)
                    } else {
                        Ok(42)
                    }
                }
            },
            &config,
        ).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), 42);
        assert_eq!(*call_count.lock().unwrap(), 3);
    }
}
