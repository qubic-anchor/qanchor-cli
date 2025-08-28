//! Network connectivity test for all Qubic networks
//! 
//! Tests connectivity to mainnet, testnet, and staging networks

use qubic_rpc::{QubicRpcClient, Network};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🌐 Qubic Network Connectivity Test");
    println!("==================================");

    let networks = [
        ("Mainnet", Network::Mainnet),
        ("Testnet", Network::Testnet),
        ("Staging", Network::Staging),
    ];

    for (name, network) in networks.iter() {
        println!("\n🔗 Testing {} network...", name);
        println!("   URL: {}", network.base_url());
        
        // Create client with shorter timeout for testing
        let client = match QubicRpcClient::with_timeout(*network, Duration::from_secs(10)) {
            Ok(client) => client,
            Err(e) => {
                println!("   ❌ Failed to create client: {}", e);
                continue;
            }
        };

        // Test basic connectivity
        print!("   🏓 Ping test... ");
        match client.ping().await {
            Ok(duration) => {
                println!("✅ {}ms", duration.as_millis());
            }
            Err(e) => {
                println!("❌ {}", e);
                continue;
            }
        }

        // Test status endpoint
        print!("   📊 Status test... ");
        match client.get_status().await {
            Ok(status) => {
                println!("✅");
                println!(
                    "      Current tick: {}",
                    status.last_processed_tick.tick_number
                );
                println!(
                    "      Current epoch: {}",
                    status.last_processed_tick.epoch
                );
            }
            Err(e) => {
                println!("❌ {}", e);
                continue;
            }
        }

        // Test quorum endpoint
        print!("   ⚖️  Quorum test... ");
        match client.get_quorum().await {
            Ok(quorum) => {
                println!("✅");
                println!("      Quorum size: {}", quorum.quorum_size);
                println!("      Total computors: {}", quorum.total_computors);
                println!("      Online computors: {}", quorum.online_computors);
                println!("      Network ready: {}", quorum.quorum_size >= 451);
            }
            Err(e) => {
                println!("❌ {}", e);
            }
        }

        // Test a sample entity query (using zero address as example)
        print!("   👤 Entity test... ");
        let zero_address = [0u8; 32];
        match client.get_entity(&zero_address).await {
            Ok(entity) => {
                println!("✅");
                println!("      Balance: {}", entity.balance);
                println!("      Tick: {}", entity.tick);
            }
            Err(e) => {
                println!("❌ {}", e);
            }
        }

        // Test current tick (derived from status)
        print!("   🕒 Current tick test... ");
        match client.get_current_tick().await {
            Ok(tick) => {
                println!("✅ Tick: {}", tick);
            }
            Err(e) => {
                println!("❌ {}", e);
            }
        }

        // Test network readiness
        print!("   🟢 Network ready test... ");
        match client.is_network_ready().await {
            Ok(ready) => {
                println!("✅ Ready: {}", ready);
            }
            Err(e) => {
                println!("❌ {}", e);
            }
        }

        println!("   {} network test completed", name);
    }

    println!("\n📋 Summary:");
    println!("   Mainnet: Official production network");
    println!("   Testnet: Development and testing network");
    println!("   Staging: Internal testing network (V2 API)");
    
    println!("\n💡 Tips:");
    println!("   - Use testnet for development");
    println!("   - Use mainnet for production");
    println!("   - Network availability may vary");
    println!("   - Some endpoints might not be implemented yet");

    Ok(())
}
