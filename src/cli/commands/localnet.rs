use anyhow::Result;
use colored::*;
use crate::localnet::LocalNet;

pub async fn execute(port: u16, reset: bool) -> Result<()> {
    println!("{}", "🌐 Starting QAnchor Local Network...".bold());
    println!();
    println!("Configuration:");
    println!("  {} {}", "Port:".dimmed(), port.to_string().cyan());
    println!("  {} {}", "Reset:".dimmed(), if reset { "Yes".green() } else { "No".yellow() }.to_string());
    println!("  {} {}", "Mode:".dimmed(), "Development".cyan());
    println!();
    
    let mut localnet = LocalNet::new(port, reset);
    
    // 啟動本地網路
    match localnet.start().await {
        Ok(_) => {
            println!("{}", "Local network stopped.".dimmed());
            Ok(())
        }
        Err(e) => {
            println!("{} Failed to start local network: {}", "❌".red(), e);
            Err(e)
        }
    }
}
