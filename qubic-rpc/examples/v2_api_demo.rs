//! Qubic RPC 2.0 API demonstration
//! 
//! This example demonstrates the new v2 API features including:
//! - Advanced transaction filtering
//! - Pagination support
//! - Range queries
//! - Enhanced tick data retrieval

use qubic_rpc::{QubicRpcClient, Network};
use qubic_rpc::types::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Qubic RPC 2.0 API Demo");
    println!("=======================");
    
    // Create client for mainnet
    let client = QubicRpcClient::new(Network::Mainnet)?;
    
    // Example 1: Get recent transactions for a specific identity
    println!("\n📋 Example 1: Get recent transactions");
    let identity = "AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAFXIB"; // Burn address
    
    match client.get_identity_transactions(identity, Some(5), Some(0)).await {
        Ok(response) => {
            println!("✅ Found {} transactions (total: {})", 
                response.transactions.len(), response.total_count);
            
            for (i, tx) in response.transactions.iter().enumerate() {
                println!("  {}. Tick: {}, Amount: {}, Type: {}", 
                    i + 1, tx.tick_number, tx.amount, tx.input_type);
            }
        }
        Err(e) => println!("❌ Failed to get transactions: {}", e),
    }
    
    // Example 2: Get transactions with amount filter (>1M QUBIC)
    println!("\n💰 Example 2: Get high-value transactions (>1M QUBIC)");
    match client.get_transactions_with_amount_filter(
        identity, 
        Some("1000000"), // Min 1M QUBIC
        None, // No max limit
        Some(3)
    ).await {
        Ok(response) => {
            println!("✅ Found {} high-value transactions", response.transactions.len());
            
            for tx in &response.transactions {
                println!("  Tick: {}, Amount: {} QUBIC", 
                    tx.tick_number, tx.amount);
            }
        }
        Err(e) => println!("❌ Failed to get high-value transactions: {}", e),
    }
    
    // Example 3: Get transactions in specific tick range
    println!("\n⏰ Example 3: Get transactions in tick range");
    let current_tick = match client.get_current_tick().await {
        Ok(tick) => tick,
        Err(_) => 25000000, // Fallback
    };
    
    let start_tick = current_tick.saturating_sub(1000); // Last 1000 ticks
    
    match client.get_transactions_in_tick_range(
        identity,
        Some(start_tick),
        Some(current_tick),
        Some(3)
    ).await {
        Ok(response) => {
            println!("✅ Found {} transactions in tick range {}-{}", 
                response.transactions.len(), start_tick, current_tick);
            
            for tx in &response.transactions {
                println!("  Tick: {}, Amount: {}", tx.tick_number, tx.amount);
            }
        }
        Err(e) => println!("❌ Failed to get transactions in range: {}", e),
    }
    
    // Example 4: Get recent tick data
    println!("\n🔍 Example 4: Get recent tick data");
    match client.get_recent_ticks(Some(5)).await {
        Ok(response) => {
            println!("✅ Retrieved {} recent ticks", response.tick_data.len());
            
            for tick in &response.tick_data {
                println!("  Tick: {}, Epoch: {}, Transactions: {}", 
                    tick.tick_number, tick.epoch, tick.transaction_count);
            }
        }
        Err(e) => println!("❌ Failed to get tick data: {}", e),
    }
    
    // Example 5: Advanced filtering with custom request
    println!("\n🔧 Example 5: Advanced custom filtering");
    let custom_request = TransactionsForIdentityRequest {
        identity: identity.to_string(),
        filters: Some(QueryFilters {
            input_type: Some("0".to_string()), // Only transfers
            transaction_type: None,
            execution_status: None,
        }),
        ranges: Some(QueryRanges {
            amount: Some(RangeFilter {
                gt: None,
                gte: Some("500000".to_string()), // At least 500K QUBIC
                lt: None,
                lte: Some("5000000".to_string()), // At most 5M QUBIC
            }),
            tick_number: None,
            timestamp: None,
        }),
        pagination: Some(Pagination {
            size: Some(10),
            offset: Some(0),
        }),
    };
    
    match client.get_transactions_for_identity_v2(&custom_request).await {
        Ok(response) => {
            println!("✅ Found {} filtered transactions", response.transactions.len());
            
            for tx in &response.transactions {
                println!("  Tick: {}, Amount: {}, Input Type: {}", 
                    tx.tick_number, tx.amount, tx.input_type);
            }
        }
        Err(e) => println!("❌ Failed to get filtered transactions: {}", e),
    }
    
    println!("\n🎉 Qubic RPC 2.0 API demo completed!");
    Ok(())
}
