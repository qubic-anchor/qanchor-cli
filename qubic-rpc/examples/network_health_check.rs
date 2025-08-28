//! Network health monitoring and retry demonstration
//! 
//! Shows how to use retry strategies and health checking for robust network operations

use qubic_rpc::{
    QubicRpcClient, Network, RetryConfig, NetworkHealthChecker, HealthStatus
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 Qubic Network Health Check & Retry Demo");
    println!("==========================================");

    // Test different retry configurations
    let networks = [
        ("Mainnet", Network::Mainnet),
        ("Testnet", Network::Testnet),
        ("Staging", Network::Staging),
    ];

    let retry_configs = [
        ("No Retry", RetryConfig::no_retry()),
        ("Conservative", RetryConfig::conservative()),
        ("Default", RetryConfig::default()),
        ("Aggressive", RetryConfig::aggressive()),
    ];

    for (network_name, network) in networks.iter() {
        println!("\n🌐 Testing {} Network", network_name);
        println!("{}  URL: {}", "   ", network.base_url());

        for (config_name, retry_config) in retry_configs.iter() {
            println!("\n   📊 Testing with {} retry config:", config_name);
            println!("      Max attempts: {}", retry_config.max_attempts);
            println!("      Initial delay: {:?}", retry_config.initial_delay);

            // Create client with specific retry config
            let client = QubicRpcClient::with_retry_config(*network, retry_config.clone())?;
            
            // Perform health check
            let health_checker = NetworkHealthChecker::new(retry_config.clone());
            let start_time = std::time::Instant::now();
            
            match health_checker.check_health(&client).await {
                Ok(health) => {
                    let elapsed = start_time.elapsed();
                    println!("      ✅ Health check completed in {:?}", elapsed);
                    println!("      Status: {:?}", health.status);
                    println!("      Description: {}", health.status_description());
                    println!("      Response time: {:?}", health.response_time);
                    
                    if let Some(error) = &health.error {
                        println!("      Error details: {}", error);
                    }

                    // Try to get network status if healthy enough
                    if health.is_usable() {
                        match client.get_status().await {
                            Ok(status) => {
                                println!("      📈 Current tick: {}", status.last_processed_tick.tick_number);
                                println!("      📈 Current epoch: {}", status.last_processed_tick.epoch);
                            }
                            Err(e) => {
                                println!("      ❌ Status query failed: {}", e);
                            }
                        }
                    } else {
                        println!("      ⚠️  Network not usable for operations");
                    }
                }
                Err(e) => {
                    let elapsed = start_time.elapsed();
                    println!("      ❌ Health check failed in {:?}: {}", elapsed, e);
                }
            }
        }
    }

    // Demonstrate specific retry scenarios
    println!("\n🔄 Retry Strategy Demonstrations");
    println!("================================");

    // Test with definitely working mainnet
    println!("\n✅ Testing retry with working endpoint (Mainnet):");
    let client = QubicRpcClient::with_retry_config(
        Network::Mainnet, 
        RetryConfig::conservative()
    )?;

    let start_time = std::time::Instant::now();
    match client.get_status().await {
        Ok(status) => {
            let elapsed = start_time.elapsed();
            println!("   ✅ Success in {:?}", elapsed);
            println!("   Current tick: {}", status.last_processed_tick.tick_number);
        }
        Err(e) => {
            let elapsed = start_time.elapsed();
            println!("   ❌ Failed in {:?}: {}", elapsed, e);
        }
    }

    // Test with likely failing testnet
    println!("\n❌ Testing retry with failing endpoint (Testnet):");
    let client = QubicRpcClient::with_retry_config(
        Network::Testnet, 
        RetryConfig::conservative()
    )?;

    let start_time = std::time::Instant::now();
    match client.get_status().await {
        Ok(status) => {
            let elapsed = start_time.elapsed();
            println!("   ✅ Unexpected success in {:?}", elapsed);
            println!("   Current tick: {}", status.last_processed_tick.tick_number);
        }
        Err(e) => {
            let elapsed = start_time.elapsed();
            println!("   ❌ Expected failure in {:?}: {}", elapsed, e);
            println!("   This demonstrates retry exhaustion after multiple attempts");
        }
    }

    println!("\n📋 Network Reliability Summary:");
    println!("===============================");
    println!("✅ Mainnet: Generally reliable for production use");
    println!("❌ Testnet: Currently down (HTTP 521) - use conservative retry");  
    println!("❓ Staging: Limited API surface (404 for many endpoints)");
    
    println!("\n💡 Retry Best Practices:");
    println!("========================");
    println!("• Use conservative retry for production");
    println!("• Use aggressive retry for development/testing");
    println!("• Monitor health before critical operations");
    println!("• Implement fallback strategies for failed networks");
    println!("• Log retry attempts for debugging");

    Ok(())
}
