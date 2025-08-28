# Qubic RPC Client

A comprehensive Rust client library for interacting with the Qubic blockchain RPC API.

Based on the [official Qubic RPC documentation](https://docs.qubic.org/api/rpc) and [testnet resources](https://docs.qubic.org/developers/testnet-resources).

## Features

- 🌐 **Multi-Network Support**: Mainnet, Testnet, and Staging networks
- 🔒 **Complete Crypto Support**: K12 hashing, Ed25519 signing, and wallet management
- 📄 **Smart Contract Integration**: Query and interact with Qubic smart contracts
- 💸 **Transaction Management**: Create, sign, and broadcast transactions
- 🔄 **Automatic Retry Logic**: Configurable retry strategies for network resilience
- 🏥 **Health Monitoring**: Network health checking and status monitoring
- ⚡ **Async/Await**: Full async support with Tokio
- 🛡️ **Error Handling**: Comprehensive error types and handling

## Quick Start

Add this to your `Cargo.toml`:

```toml
[dependencies]
qubic-rpc = "0.1.0"
tokio = { version = "1.0", features = ["full"] }
```

### Basic Usage

```rust
use qubic_rpc::{QubicRpcClient, QubicWallet, Network};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create RPC client
    let client = QubicRpcClient::new(Network::Mainnet)?;
    
    // Get network status
    let status = client.get_status().await?;
    println!("Current tick: {}", status.last_processed_tick.tick_number);
    
    // Create wallet from seed
    let wallet = QubicWallet::from_seed("your-55-character-seed")?;
    
    // Create and sign transaction
    let signed_tx = wallet.create_transfer(
        &recipient_pubkey,
        1000, // amount in QU
        status.last_processed_tick.tick_number + 1000
    )?;
    
    // Broadcast transaction
    let response = client.broadcast_transaction(&signed_tx).await?;
    println!("Transaction ID: {}", response.tx_id);
    
    Ok(())
}
```

## Network Status and Availability

### Current Network Status

| Network | Status | Description |
|---------|--------|-------------|
| **Mainnet** | ✅ Available | Production network - fully operational |
| **Testnet** | ❌ Down | Currently returning HTTP 521 (Web server down) |
| **Staging** | ⚠️ Limited | V2 API with limited endpoint support |

### Expected Behavior When Nodes Are Unavailable

When network nodes are unavailable or experiencing issues, the client exhibits the following behavior:

#### Automatic Retry Strategy

```rust
use qubic_rpc::{QubicRpcClient, Network, RetryConfig};

// Configure retry behavior
let retry_config = RetryConfig {
    max_attempts: 3,
    initial_delay: Duration::from_millis(500),
    max_delay: Duration::from_secs(10),
    backoff_multiplier: 2.0,
};

let client = QubicRpcClient::with_retry_config(Network::Testnet, retry_config)?;
```

#### Error Types and Handling

- **HTTP 521 (Web server down)**: Automatically retried with exponential backoff
- **HTTP 502/503 (Bad gateway/Service unavailable)**: Retried as temporary issues
- **HTTP 404 (Not found)**: Not retried, indicates missing endpoint
- **Timeout errors**: Retried with increased delays
- **Connection errors**: Retried as network issues

#### Health Monitoring

```rust
use qubic_rpc::{NetworkHealthChecker, HealthStatus};

let health_checker = NetworkHealthChecker::new(RetryConfig::default());
let health = health_checker.check_health(&client).await?;

match health.status {
    HealthStatus::Healthy => println!("Network is operational"),
    HealthStatus::Degraded => println!("Network is slow but usable"),
    HealthStatus::Slow => println!("Network is very slow"),
    HealthStatus::Unhealthy => println!("Network is not available"),
}
```

### API Endpoint Availability

Some endpoints may not be implemented on all networks:

- **`/v1/status`**: ✅ Available on all networks
- **`/v1/entity/{id}`**: ❌ Returns 404 on most networks
- **`/v1/quorum`**: ❌ Returns 404 on most networks  
- **`/v1/querySmartContract`**: ⚠️ Available but may fail without deployed contracts
- **`/v1/broadcast-transaction`**: ⚠️ Available but requires valid transactions

The client handles these gracefully with descriptive error messages.

## Examples

### Network Health Monitoring

```rust
use qubic_rpc::{QubicRpcClient, Network, NetworkHealthChecker, RetryConfig};

let client = QubicRpcClient::new(Network::Mainnet)?;
let health_checker = NetworkHealthChecker::new(RetryConfig::conservative());

let health = health_checker.check_health(&client).await?;
println!("Network status: {}", health.status_description());
println!("Response time: {:?}", health.response_time);

if health.is_usable() {
    // Proceed with operations
    let status = client.get_status().await?;
    println!("Network is ready, current tick: {}", status.last_processed_tick.tick_number);
} else {
    println!("Network is not available, trying alternative...");
}
```

### Transaction Creation with Error Handling

```rust
use qubic_rpc::{QubicWallet, TransactionBuilder, QubicRpcClient, Network};

// Create wallet
let wallet = QubicWallet::from_seed("your-seed-here")?;

// Get current network state
let client = QubicRpcClient::new(Network::Mainnet)?;
let status = client.get_status().await?;

// Build transaction with proper tick
let transaction = TransactionBuilder::new()
    .source(wallet.public_key())
    .destination(recipient)
    .amount(1000)
    .tick(status.last_processed_tick.tick_number + 1000)
    .build()?;

// Sign and verify
let signed_tx = wallet.sign_transaction(transaction)?;
assert!(wallet.verify_transaction(&signed_tx)?);

// Broadcast with retry logic
match client.broadcast_transaction(&signed_tx).await {
    Ok(response) => println!("Transaction broadcasted: {}", response.tx_id),
    Err(e) => println!("Broadcast failed: {}", e),
}
```

### Smart Contract Query with Fallback

```rust
use qubic_rpc::{QubicRpcClient, Network, QubicRpcError};

async fn query_contract_with_fallback(
    contract_index: u32,
    input_data: &[u8]
) -> Result<Vec<u8>, QubicRpcError> {
    // Try mainnet first
    let client = QubicRpcClient::new(Network::Mainnet)?;
    
    match client.query_smart_contract(contract_index, 1, input_data).await {
        Ok(result) => return Ok(result),
        Err(e) => println!("Mainnet query failed: {}", e),
    }
    
    // Fallback to testnet (if available)
    let client = QubicRpcClient::new(Network::Testnet)?;
    
    match client.query_smart_contract(contract_index, 1, input_data).await {
        Ok(result) => Ok(result),
        Err(e) => {
            println!("Testnet query also failed: {}", e);
            Err(e)
        }
    }
}
```

## Development and Testing

### Running Examples

```bash
# Basic usage example
cargo run --example basic_usage

# Network connectivity test  
cargo run --example network_test

# Health monitoring demo
cargo run --example network_health_check

# Mainnet integration test
cargo run --example mainnet_test

# Testnet integration (may fail if testnet is down)
cargo run --example testnet_integration
```

### Testing with Unavailable Networks

The library is designed to handle network unavailability gracefully:

```bash
# Run tests (some integration tests may be ignored)
cargo test

# Run integration tests (requires network access)
cargo test --ignored

# Test retry logic specifically
cargo test retry_
```

## Best Practices

### Production Usage

1. **Use Conservative Retry**: `RetryConfig::conservative()` for production
2. **Monitor Health**: Check network health before critical operations
3. **Handle Failures**: Implement proper error handling and fallbacks
4. **Log Retry Attempts**: Enable logging to monitor retry behavior

```rust
let client = QubicRpcClient::with_retry_config(
    Network::Mainnet,
    RetryConfig::conservative()
)?;
```

### Development and Testing

1. **Use Testnet**: When available, use testnet for development
2. **Small Amounts**: Test with small amounts even on testnet
3. **Monitor Logs**: Use `qlogging` service for transaction monitoring
4. **Clean Up**: Remove test deployments after testing

### Error Handling Strategy

```rust
match client.get_status().await {
    Ok(status) => {
        // Normal operation
        println!("Network operational, tick: {}", status.last_processed_tick.tick_number);
    }
    Err(QubicRpcError::ServerError(msg)) if msg.contains("521") => {
        // Server down - implement backoff or alternative
        println!("Server temporarily down, trying again later...");
    }
    Err(QubicRpcError::Http(_)) => {
        // Network connectivity issue
        println!("Network connectivity problem");
    }
    Err(e) => {
        // Other errors
        println!("Unexpected error: {}", e);
    }
}
```

## Contributing

Contributions are welcome! Please ensure:

1. All tests pass: `cargo test`
2. Examples work: `cargo run --example basic_usage`
3. Documentation is updated
4. Error handling follows established patterns

## License

This project is licensed under the MIT License - see the LICENSE file for details.

## Acknowledgments

- Built on the [official Qubic RPC API](https://docs.qubic.org/api/rpc)
- Follows [Qubic testnet best practices](https://docs.qubic.org/developers/testnet-resources)
- Implements resilient network patterns for blockchain applications
