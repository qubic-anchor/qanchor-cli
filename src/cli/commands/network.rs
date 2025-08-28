use anyhow::Result;
use colored::*;
use qubic_rpc::{QubicRpcClient, Network};
use std::time::{Duration, Instant};
use tokio::time::sleep;

/// Execute network status command
pub async fn execute_status(network: Option<&str>) -> Result<()> {
    let target_network = network.unwrap_or("testnet");
    
    println!("{}", "🌐 Checking network status...".bold());
    println!("  {} Target network: {}", "📡".blue(), target_network.cyan());
    
    // Connect to network
    let client = connect_to_network(target_network).await?;
    
    // Get network status
    match client.get_status().await {
        Ok(status) => {
            println!("{}", "✅ Network Status: HEALTHY".green().bold());
            println!("  {} Network: {}", "🌐".blue(), target_network.cyan());
            println!("  {} Last Processed Tick: {}", "⏰".blue(), status.last_processed_tick.tick_number.to_string().cyan());
            println!("  {} Current Epoch: {}", "📅".blue(), status.last_processed_tick.epoch.to_string().cyan());
            println!("  {} Skipped Ticks: {}", "⚠️".yellow(), status.skipped_ticks.len().to_string().cyan());
            println!("  {} Processed Epochs: {}", "📈".blue(), status.processed_tick_intervals_per_epoch.len().to_string().cyan());
            
            // Show network URL
            println!("  {} RPC Endpoint: {}", "🔗".blue(), client.network().base_url().cyan());
        }
        Err(e) => {
            println!("{}", "❌ Network Status: UNHEALTHY".red().bold());
            println!("  {} Error: {}", "⚠️".yellow(), e.to_string().red());
            anyhow::bail!("Network status check failed: {}", e);
        }
    }
    
    Ok(())
}

/// Execute network ping command
pub async fn execute_ping(network: Option<&str>, count: Option<u32>) -> Result<()> {
    let target_network = network.unwrap_or("testnet");
    let ping_count = count.unwrap_or(5);
    
    println!("{}", "🏓 Pinging network nodes...".bold());
    println!("  {} Target network: {}", "📡".blue(), target_network.cyan());
    println!("  {} Ping count: {}", "🔢".blue(), ping_count.to_string().cyan());
    println!();
    
    // Connect to network
    let client = connect_to_network(target_network).await?;
    
    let mut successful_pings = 0;
    let mut total_time = Duration::new(0, 0);
    let mut min_time = Duration::from_secs(u64::MAX);
    let mut max_time = Duration::new(0, 0);
    
    for i in 1..=ping_count {
        let start_time = Instant::now();
        
        match client.get_status().await {
            Ok(status) => {
                let duration = start_time.elapsed();
                successful_pings += 1;
                total_time += duration;
                
                if duration < min_time {
                    min_time = duration;
                }
                if duration > max_time {
                    max_time = duration;
                }
                
                println!(
                    "  {} Ping #{}: {} tick={} time={}ms",
                    "✅".green(),
                    i.to_string().cyan(),
                    "SUCCESS".green(),
                    status.last_processed_tick.tick_number.to_string().yellow(),
                    duration.as_millis().to_string().cyan()
                );
            }
            Err(e) => {
                let duration = start_time.elapsed();
                println!(
                    "  {} Ping #{}: {} time={}ms error={}",
                    "❌".red(),
                    i.to_string().cyan(),
                    "FAILED".red(),
                    duration.as_millis().to_string().cyan(),
                    e.to_string().red()
                );
            }
        }
        
        // Wait 1 second between pings (except for the last one)
        if i < ping_count {
            sleep(Duration::from_secs(1)).await;
        }
    }
    
    // Print statistics
    println!();
    println!("{}", "📊 Ping Statistics:".bold());
    println!("  {} Packets sent: {}", "📤".blue(), ping_count.to_string().cyan());
    println!("  {} Packets received: {}", "📥".green(), successful_pings.to_string().cyan());
    println!("  {} Packet loss: {}%", "📉".yellow(), 
        ((ping_count - successful_pings) as f64 / ping_count as f64 * 100.0).to_string().cyan());
    
    if successful_pings > 0 {
        let avg_time = total_time / successful_pings;
        println!("  {} Round-trip times:", "⏱️".blue());
        println!("    {} min = {}ms", "🔽".green(), min_time.as_millis().to_string().cyan());
        println!("    {} avg = {}ms", "📊".blue(), avg_time.as_millis().to_string().cyan());
        println!("    {} max = {}ms", "🔼".red(), max_time.as_millis().to_string().cyan());
    }
    
    if successful_pings == ping_count {
        println!("{}", "🎉 Network connectivity: EXCELLENT".green().bold());
    } else if successful_pings > ping_count / 2 {
        println!("{}", "⚠️ Network connectivity: UNSTABLE".yellow().bold());
    } else {
        println!("{}", "❌ Network connectivity: POOR".red().bold());
        anyhow::bail!("Network ping test failed with high packet loss");
    }
    
    Ok(())
}

/// Connect to the specified network
async fn connect_to_network(network: &str) -> Result<QubicRpcClient> {
    let network_enum = match network.to_lowercase().as_str() {
        "mainnet" | "main" => Network::Mainnet,
        "testnet" | "test" => Network::Testnet,
        "staging" | "stage" => Network::Staging,
        "local" => {
            println!("  {} Local network not supported for network commands, using testnet", "⚠️".yellow());
            Network::Testnet
        }
        _ => {
            println!("  {} Unknown network '{}', using testnet", "⚠️".yellow(), network);
            Network::Testnet
        }
    };
    
    let client = QubicRpcClient::new(network_enum)?;
    
    // Perform a quick connection test
    let _status = client.get_status().await
        .map_err(|e| anyhow::anyhow!("Failed to connect to network '{}': {}", network, e))?;
    
    Ok(client)
}
