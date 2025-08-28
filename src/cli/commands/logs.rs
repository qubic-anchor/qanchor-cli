use anyhow::Result;
use colored::*;
use qubic_rpc::{QubicRpcClient, Network};
use std::time::Duration;
use tokio::time::{sleep, interval};
use chrono::{DateTime, Utc};

/// Execute logs command
pub async fn execute(
    contract_id: Option<&str>,
    follow: bool,
    tail: Option<u32>,
    since: Option<&str>,
    filter: Option<&str>,
    network: Option<&str>,
) -> Result<()> {
    let target_network = network.unwrap_or("testnet");
    
    println!("{}", "📋 Fetching contract logs...".bold());
    println!("  {} Target network: {}", "🌐".blue(), target_network.cyan());
    
    if let Some(id) = contract_id {
        println!("  {} Contract ID: {}", "📄".blue(), id.cyan());
    } else {
        println!("  {} Scope: {}", "📄".blue(), "All contracts".cyan());
    }
    
    // Connect to network
    let client = connect_to_network(target_network).await?;
    
    if follow {
        // Real-time log streaming
        stream_logs(&client, contract_id, filter).await
    } else {
        // Historical log query
        fetch_historical_logs(&client, contract_id, tail, since, filter).await
    }
}

/// Stream real-time logs from contracts
async fn stream_logs(
    client: &QubicRpcClient,
    contract_id: Option<&str>,
    filter: Option<&str>,
) -> Result<()> {
    println!("{}", "🔄 Starting real-time log streaming...".green().bold());
    println!("  {} Press Ctrl+C to stop", "💡".yellow());
    println!();
    
    let mut last_tick = get_current_tick(client).await?;
    let mut log_count = 0;
    let mut interval = interval(Duration::from_secs(2));
    
    loop {
        interval.tick().await;
        
        // Get current network status
        match client.get_status().await {
            Ok(status) => {
                let current_tick = status.last_processed_tick.tick_number;
                
                if current_tick > last_tick {
                    // Simulate fetching logs for new ticks
                    let new_logs = simulate_fetch_logs_for_range(
                        last_tick + 1,
                        current_tick,
                        contract_id,
                        filter,
                    ).await;
                    
                    for log in new_logs {
                        log_count += 1;
                        print_log_entry(&log, log_count);
                    }
                    
                    last_tick = current_tick;
                } else {
                    // No new ticks, show heartbeat
                    print!(".");
                    std::io::Write::flush(&mut std::io::stdout()).unwrap();
                }
            }
            Err(e) => {
                println!("  {} Network error: {}", "⚠️".yellow(), e.to_string().red());
                sleep(Duration::from_secs(5)).await;
            }
        }
    }
}

/// Fetch historical logs
async fn fetch_historical_logs(
    client: &QubicRpcClient,
    contract_id: Option<&str>,
    tail: Option<u32>,
    since: Option<&str>,
    filter: Option<&str>,
) -> Result<()> {
    println!("{}", "📜 Fetching historical logs...".green().bold());
    
    // Get current network status for context
    let status = client.get_status().await?;
    let current_tick = status.last_processed_tick.tick_number;
    
    // Determine the range to query
    let (start_tick, end_tick) = determine_log_range(current_tick, tail, since)?;
    
    println!("  {} Querying ticks {} to {}", "🔍".blue(), 
        start_tick.to_string().cyan(), 
        end_tick.to_string().cyan()
    );
    println!();
    
    // Simulate fetching historical logs
    let logs = simulate_fetch_logs_for_range(start_tick, end_tick, contract_id, filter).await;
    
    if logs.is_empty() {
        println!("  {} No logs found in the specified range", "📭".yellow());
    } else {
        println!("  {} Found {} log entries:", "📋".green(), logs.len().to_string().cyan());
        println!();
        
        for (i, log) in logs.iter().enumerate() {
            print_log_entry(log, i + 1);
        }
    }
    
    Ok(())
}

/// Determine the tick range for log queries
fn determine_log_range(
    current_tick: u64,
    tail: Option<u32>,
    since: Option<&str>,
) -> Result<(u64, u64)> {
    let end_tick = current_tick;
    
    let start_tick = if let Some(since_str) = since {
        // Parse time-based filter
        parse_since_time(since_str, current_tick)?
    } else if let Some(tail_count) = tail {
        // Get last N entries (approximate)
        if current_tick > tail_count as u64 * 100 {
            current_tick - (tail_count as u64 * 100)
        } else {
            1
        }
    } else {
        // Default: last 1000 ticks
        if current_tick > 1000 {
            current_tick - 1000
        } else {
            1
        }
    };
    
    Ok((start_tick, end_tick))
}

/// Parse time-based since filter
fn parse_since_time(since: &str, current_tick: u64) -> Result<u64> {
    // Simple time parsing - could be enhanced
    if since.ends_with('h') {
        let hours: u64 = since.trim_end_matches('h').parse()?;
        // Approximate: 1 hour = 3600 seconds, ~360 ticks (assuming 10s per tick)
        let ticks_back = hours * 360;
        Ok(if current_tick > ticks_back { current_tick - ticks_back } else { 1 })
    } else if since.ends_with('m') {
        let minutes: u64 = since.trim_end_matches('m').parse()?;
        // Approximate: 1 minute = 60 seconds, ~6 ticks
        let ticks_back = minutes * 6;
        Ok(if current_tick > ticks_back { current_tick - ticks_back } else { 1 })
    } else if since.ends_with('s') {
        let seconds: u64 = since.trim_end_matches('s').parse()?;
        // Approximate: 10 seconds per tick
        let ticks_back = seconds / 10;
        Ok(if current_tick > ticks_back { current_tick - ticks_back } else { 1 })
    } else {
        // Try to parse as tick number
        let tick: u64 = since.parse()?;
        Ok(tick)
    }
}

/// Get current tick from network
async fn get_current_tick(client: &QubicRpcClient) -> Result<u64> {
    let status = client.get_status().await?;
    Ok(status.last_processed_tick.tick_number)
}

/// Simulate fetching logs for a tick range
async fn simulate_fetch_logs_for_range(
    start_tick: u64,
    end_tick: u64,
    contract_id: Option<&str>,
    filter: Option<&str>,
) -> Vec<LogEntry> {
    let mut logs = Vec::new();
    
    // Simulate some log entries
    let log_frequency = 10; // One log every ~10 ticks
    
    for tick in (start_tick..=end_tick).step_by(log_frequency) {
        // Always generate at least one log per iteration for demo
        logs.push(LogEntry {
            tick,
            timestamp: tick_to_timestamp(tick),
            contract_id: contract_id.unwrap_or("QC7a8b9d2e3f4g5h6i7j8k9l0m1n2o3p4q5r6s7t8u9v0w1x2y3z4a5b6c7d8e9f0").to_string(),
            log_type: LogType::Info,
            message: "Contract execution completed successfully".to_string(),
            data: Some("result: 42, gas_used: 1250".to_string()),
        });
        
        // Simulate different types of logs
        if tick % 30 == 0 {
            logs.push(LogEntry {
                tick,
                timestamp: tick_to_timestamp(tick),
                contract_id: contract_id.unwrap_or("QC7a8b9d2e3f4g5h6i7j8k9l0m1n2o3p4q5r6s7t8u9v0w1x2y3z4a5b6c7d8e9f0").to_string(),
                log_type: LogType::Info,
                message: "Contract execution completed successfully".to_string(),
                data: Some("result: 42, gas_used: 1250".to_string()),
            });
        }
        
        if tick % 50 == 0 {
            logs.push(LogEntry {
                tick,
                timestamp: tick_to_timestamp(tick),
                contract_id: contract_id.unwrap_or("QD8e9f0a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6a7b8c9d0e").to_string(),
                log_type: LogType::Warning,
                message: "High gas usage detected".to_string(),
                data: Some("gas_limit: 5000, gas_used: 4750".to_string()),
            });
        }
        
        if tick % 100 == 0 {
            logs.push(LogEntry {
                tick,
                timestamp: tick_to_timestamp(tick),
                contract_id: contract_id.unwrap_or("QE9f0a1b2c3d4e5f6g7h8i9j0k1l2m3n4o5p6q7r8s9t0u1v2w3x4y5z6a7b8c9d0e1f").to_string(),
                log_type: LogType::Error,
                message: "Transaction failed: insufficient funds".to_string(),
                data: Some("required: 1000, available: 750".to_string()),
            });
        }
    }
    
    // Apply filtering if specified
    if let Some(filter_str) = filter {
        logs.retain(|log| {
            log.message.to_lowercase().contains(&filter_str.to_lowercase()) ||
            log.data.as_ref().map_or(false, |d| d.to_lowercase().contains(&filter_str.to_lowercase()))
        });
    }
    
    logs
}

/// Convert tick number to approximate timestamp
fn tick_to_timestamp(tick: u64) -> DateTime<Utc> {
    // Qubic genesis approximation + tick * 10 seconds
    let genesis_timestamp = 1704067200; // Approximate Qubic genesis time
    let tick_duration = 10; // 10 seconds per tick (approximate)
    
    let timestamp = genesis_timestamp + (tick * tick_duration);
    DateTime::from_timestamp(timestamp as i64, 0).unwrap_or_else(|| Utc::now())
}

/// Print a single log entry
fn print_log_entry(log: &LogEntry, index: usize) {
    let type_icon = match log.log_type {
        LogType::Info => "ℹ️",
        LogType::Warning => "⚠️",
        LogType::Error => "❌",
        LogType::Debug => "🐛",
    };
    
    let type_color = match log.log_type {
        LogType::Info => "INFO".green(),
        LogType::Warning => "WARN".yellow(),
        LogType::Error => "ERROR".red(),
        LogType::Debug => "DEBUG".blue(),
    };
    
    println!(
        "{} {} [{}] {} {} {}",
        format!("#{:03}", index).dimmed(),
        type_icon,
        log.timestamp.format("%H:%M:%S").to_string().dimmed(),
        format!("tick:{}", log.tick).cyan(),
        type_color,
        log.message
    );
    
    println!(
        "    {} Contract: {}",
        "📄".blue(),
        format!("{}...", &log.contract_id[..20]).dimmed()
    );
    
    if let Some(data) = &log.data {
        println!("    {} Data: {}", "📊".blue(), data.dimmed());
    }
    
    println!();
}

/// Connect to the specified network
async fn connect_to_network(network: &str) -> Result<QubicRpcClient> {
    let network_enum = match network.to_lowercase().as_str() {
        "mainnet" | "main" => Network::Mainnet,
        "testnet" | "test" => Network::Testnet,
        "staging" | "stage" => Network::Staging,
        "local" => {
            println!("  {} Local network not supported for logs, using testnet", "⚠️".yellow());
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

/// Log entry structure
#[derive(Debug, Clone)]
struct LogEntry {
    tick: u64,
    timestamp: DateTime<Utc>,
    contract_id: String,
    log_type: LogType,
    message: String,
    data: Option<String>,
}

/// Log entry types
#[derive(Debug, Clone)]
enum LogType {
    Info,
    Warning,
    Error,
    Debug,
}
