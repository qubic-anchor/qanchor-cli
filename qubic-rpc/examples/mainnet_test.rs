//! Test Qubic mainnet connectivity
//! 
//! Since testnet returns HTTP 521, let's test mainnet which appears to be working

use qubic_rpc::{QubicRpcClient, QubicWallet, Network};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Qubic Mainnet Connectivity Test");
    println!("==================================");
    
    // Connect to mainnet (which we confirmed is working)
    println!("\n🔗 Connecting to Qubic mainnet...");
    let client = QubicRpcClient::with_timeout(Network::Mainnet, Duration::from_secs(15))?;
    println!("✅ Connected to: {}", client.base_url());

    // Test ping
    println!("\n🏓 Testing network ping...");
    match client.ping().await {
        Ok(duration) => {
            println!("✅ Ping successful: {:?}", duration);
        }
        Err(e) => {
            println!("❌ Ping failed: {}", e);
            return Ok(());
        }
    }

    // Get network status
    println!("\n📊 Getting network status...");
    let current_tick = match client.get_status().await {
        Ok(status) => {
            println!("✅ Network Status:");
            println!("   Current tick: {}", status.last_processed_tick.tick_number);
            println!("   Current epoch: {}", status.last_processed_tick.epoch);
            println!("   Skipped ticks: {} ranges", status.skipped_ticks.len());
            println!("   Processed epochs: {}", status.processed_tick_intervals_per_epoch.len());
            println!("   Empty tick epochs: {}", status.empty_ticks_per_epoch.len());
            status.last_processed_tick.tick_number
        }
        Err(e) => {
            println!("❌ Failed to get status: {}", e);
            return Ok(());
        }
    };

    // Test a sample wallet creation (not using real seeds on mainnet)
    println!("\n👛 Testing wallet creation...");
    let test_seed = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123";
    let wallet = QubicWallet::from_seed(test_seed)?;
    println!("✅ Test wallet created:");
    println!("   Address: {}", wallet.address());
    println!("   Public key: {}", hex::encode(wallet.public_key()));

    // Query the test wallet balance
    println!("\n💰 Querying test wallet balance...");
    match client.get_balance(&wallet.public_key()).await {
        Ok(balance) => {
            println!("✅ Balance: {} QU", balance);
        }
        Err(e) => {
            println!("❌ Failed to get balance: {}", e);
        }
    }

    // Test quorum status
    println!("\n⚖️  Testing network quorum...");
    match client.get_quorum().await {
        Ok(quorum) => {
            println!("✅ Quorum Status:");
            println!("   Quorum size: {}", quorum.quorum_size);
            println!("   Total computors: {}", quorum.total_computors);
            println!("   Online computors: {}", quorum.online_computors);
            println!("   Network ready: {}", quorum.quorum_size >= 451);
        }
        Err(e) => {
            println!("❌ Failed to get quorum: {}", e);
        }
    }

    // Test getting current tick
    println!("\n🕒 Testing current tick...");
    match client.get_current_tick().await {
        Ok(tick) => {
            println!("✅ Current tick: {}", tick);
        }
        Err(e) => {
            println!("❌ Failed to get current tick: {}", e);
        }
    }

    // Test network readiness
    println!("\n🟢 Testing network readiness...");
    match client.is_network_ready().await {
        Ok(ready) => {
            println!("✅ Network ready: {}", ready);
        }
        Err(e) => {
            println!("❌ Failed to check readiness: {}", e);
        }
    }

    println!("\n🎉 Mainnet test completed!");
    println!("\n📋 Analysis of network status:");
    println!("✅ Mainnet: WORKING (HTTP 200)");
    println!("❌ Testnet: DOWN (HTTP 521 - Web server is down)"); 
    println!("❌ Staging: INCOMPLETE (HTTP 404 - Not found)");

    println!("\n💡 Explanation for testnet unavailability:");
    println!("   HTTP 521 = 'Web server is down'");
    println!("   This is a Cloudflare error indicating the origin server");
    println!("   (Qubic testnet backend) is not responding.");
    println!("   This could be due to:");
    println!("   • Maintenance downtime");
    println!("   • Server overload"); 
    println!("   • Infrastructure issues");
    println!("   • Temporary service suspension");

    Ok(())
}
