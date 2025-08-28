//! Basic usage example for qubic-rpc client
//! 
//! Demonstrates how to:
//! - Create a wallet
//! - Build transactions  
//! - Query network status
//! - Interact with smart contracts

use qubic_rpc::{
    QubicRpcClient, QubicWallet, TransactionBuilder, Network,
    SmartContractQueryRequest,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔗 Qubic RPC Client Example");
    println!("===========================");

    // 1. Create RPC client (using testnet for safety)
    println!("\n📡 Creating RPC client...");
    let client = QubicRpcClient::new(Network::Testnet)?;
    println!("✅ Connected to: {}", client.base_url());

    // 2. Test network connectivity
    println!("\n🏓 Testing network connectivity...");
    match client.ping().await {
        Ok(duration) => {
            println!("✅ Network ping: {:?}", duration);
        }
        Err(e) => {
            println!("❌ Network ping failed: {}", e);
            println!("ℹ️  This is expected if testnet is not available");
        }
    }

    // 3. Try to get network status
    println!("\n📊 Getting network status...");
    match client.get_status().await {
        Ok(status) => {
            println!(
                "✅ Current tick: {}",
                status.last_processed_tick.tick_number
            );
            println!(
                "✅ Current epoch: {}",
                status.last_processed_tick.epoch
            );
        }
        Err(e) => {
            println!("❌ Failed to get status: {}", e);
            println!("ℹ️  This is expected if testnet is not available");
        }
    }

    // 4. Create a wallet
    println!("\n👛 Creating wallet...");
    let seed = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ123";
    let wallet = QubicWallet::from_seed(seed)?;
    println!("✅ Wallet address: {}", wallet.address());
    
    // Display public key as hex
    let public_key = wallet.public_key();
    let hex_key = hex::encode(public_key);
    println!("✅ Public key (hex): {}", hex_key);

    // 5. Create a transfer transaction (example)
    println!("\n💸 Creating transfer transaction...");
    let recipient = [1u8; 32]; // Example recipient
    let amount = 1000; // Amount in basic units
    let current_tick = 12345; // Example tick

    let signed_tx = wallet.create_transfer(&recipient, amount, current_tick)?;
    println!("✅ Created transfer transaction");
    println!("   From: {}", hex::encode(signed_tx.transaction.source_public_key));
    println!("   To: {}", hex::encode(signed_tx.transaction.destination_public_key));
    println!("   Amount: {}", signed_tx.transaction.amount);
    println!("   Tick: {}", signed_tx.transaction.tick);
    
    // Verify signature
    let is_valid = wallet.verify_transaction(&signed_tx)?;
    println!("✅ Signature valid: {}", is_valid);

    // 6. Create a smart contract transaction using builder
    println!("\n📄 Creating smart contract transaction...");
    let contract_address = [2u8; 32]; // Example contract
    let contract_input = vec![0x01, 0x02, 0x03, 0x04]; // Example input data
    
    let transaction = TransactionBuilder::new()
        .source(wallet.public_key())
        .destination(contract_address)
        .amount(500)
        .tick(current_tick + 1)
        .input_type(1) // Smart contract call
        .input_data(contract_input.clone())
        .build()?;

    let signed_contract_tx = wallet.sign_transaction(transaction)?;
    println!("✅ Created smart contract transaction");
    println!("   Contract: {}", hex::encode(signed_contract_tx.transaction.destination_public_key));
    println!("   Input type: {}", signed_contract_tx.transaction.input_type);
    println!("   Input data: {:?}", signed_contract_tx.transaction.input_data);

    // 7. Example smart contract query (would fail without real contract)
    println!("\n🔍 Example smart contract query...");
    match client.query_smart_contract(1, 1, &contract_input).await {
        Ok(response) => {
            println!("✅ Contract response: {:?}", response);
        }
        Err(e) => {
            println!("❌ Contract query failed: {}", e);
            println!("ℹ️  This is expected without a real deployed contract");
        }
    }

    // 8. Check if network is ready (has quorum)
    println!("\n⚖️  Checking network quorum...");
    match client.is_network_ready().await {
        Ok(ready) => {
            println!("✅ Network ready: {}", ready);
        }
        Err(e) => {
            println!("❌ Failed to check quorum: {}", e);
        }
    }

    println!("\n🎉 Example completed!");
    println!("\nℹ️  Note: Some operations may fail when testnet is unavailable.");
    println!("   This is normal and demonstrates error handling.");

    Ok(())
}

// Helper function to add hex dependency if needed
// Add this to Cargo.toml:
// [dev-dependencies]
// hex = "0.4"
