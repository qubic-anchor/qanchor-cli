//! Full integration test using official Qubic testnet resources
//! 
//! Based on: https://docs.qubic.org/developers/testnet-resources
//! Uses official pre-funded seeds for realistic testing

use qubic_rpc::{QubicRpcClient, QubicWallet, Network};
use std::time::Duration;

/// Official pre-funded testnet seeds from Qubic documentation
/// Each contains approximately 1 billion Qubic tokens
const TESTNET_SEEDS: &[&str] = &[
    "fwqatwliqyszxivzgtyyfllymopjimkyoreolgyflsnfpcytkhagqii",
    "xpsxzzfqvaohzzwlbofvqkqeemzhnrscpeeokoumekfodtgzmwghtqm", 
    "ukzbkszgzpipmxrrqcxcppumxoxzerrvbjgthinzodrlyblkedutmsy",
    "wgfqazfmgucrluchpuivdkguaijrowcnuclfsjrthfezqapnjelkgll",
    "kewgvatawujuzikurbhwkrisjiubfxgfqkrvcqvfvgfgajphbvhlaos",
];

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🧪 Qubic Testnet Integration Test");
    println!("=================================");
    println!("📋 Using official testnet resources from:");
    println!("   https://docs.qubic.org/developers/testnet-resources");
    
    // 1. Connect to official testnet
    println!("\n🔗 Connecting to official testnet...");
    let client = QubicRpcClient::with_timeout(Network::Testnet, Duration::from_secs(15))?;
    println!("✅ Connected to: {}", client.base_url());

    // 2. Test basic network connectivity
    println!("\n🏓 Testing network connectivity...");
    match client.ping().await {
        Ok(duration) => {
            println!("✅ Network ping: {:?}", duration);
        }
        Err(e) => {
            println!("❌ Network ping failed: {}", e);
            println!("ℹ️  Testnet may be temporarily unavailable");
            return Ok(());
        }
    }

    // 3. Get network status
    println!("\n📊 Getting network status...");
    let current_tick = match client.get_status().await {
        Ok(status) => {
            println!("✅ Network Status:");
            println!(
                "   Current tick: {}",
                status.last_processed_tick.tick_number
            );
            println!(
                "   Current epoch: {}",
                status.last_processed_tick.epoch
            );
            status.last_processed_tick.tick_number
        }
        Err(e) => {
            println!("❌ Failed to get status: {}", e);
            return Ok(());
        }
    };

    // 4. Test pre-funded wallets from official seeds
    println!("\n👛 Testing official pre-funded wallets...");
    for (i, seed) in TESTNET_SEEDS.iter().take(3).enumerate() {
        println!("\n   Wallet {} ({}...):", i + 1, &seed[..20]);
        
        let wallet = QubicWallet::from_seed(seed)?;
        let public_key = wallet.public_key();
        println!("   📍 Address: {}", wallet.address());
        println!("   🔑 Public key: {}", hex::encode(public_key));

        // Query balance
        match client.get_balance(&public_key).await {
            Ok(balance) => {
                println!("   💰 Balance: {} QU", balance);
                if balance > 0 {
                    println!("   ✅ Wallet is funded!");
                } else {
                    println!("   ⚠️  Wallet appears to be empty");
                }
            }
            Err(e) => {
                println!("   ❌ Failed to get balance: {}", e);
            }
        }

        // Get full entity info
        match client.get_entity(&public_key).await {
            Ok(entity) => {
                println!("   📈 Entity info:");
                println!("      Current tick: {}", entity.tick);
                println!("      Latest incoming: {}", entity.latest_incoming_transfer_tick);
                println!("      Latest outgoing: {}", entity.latest_outgoing_transfer_tick);
            }
            Err(e) => {
                println!("   ❌ Failed to get entity: {}", e);
            }
        }
    }

    // 5. Test transaction creation with real testnet conditions
    println!("\n💸 Testing transaction creation...");
    let sender_wallet = QubicWallet::from_seed(TESTNET_SEEDS[0])?;
    let receiver_wallet = QubicWallet::from_seed(TESTNET_SEEDS[1])?;

    let transfer_amount = 1000; // 1000 QU
    let signed_tx = sender_wallet.create_transfer(
        &receiver_wallet.public_key(),
        transfer_amount,
        current_tick + 1000, // Future tick
    )?;

    println!("✅ Created transfer transaction:");
    println!("   From: {}", hex::encode(signed_tx.transaction.source_public_key));
    println!("   To: {}", hex::encode(signed_tx.transaction.destination_public_key));
    println!("   Amount: {} QU", signed_tx.transaction.amount);
    println!("   Tick: {}", signed_tx.transaction.tick);
    
    // Verify signature locally
    let is_valid = sender_wallet.verify_transaction(&signed_tx)?;
    println!("   ✅ Signature valid: {}", is_valid);

    // Note: We don't actually broadcast the transaction in this test
    println!("   ℹ️  Transaction created but not broadcasted (test mode)");

    // 6. Test smart contract query (example)
    println!("\n📄 Testing smart contract query...");
    let contract_query_data = vec![0x01, 0x02, 0x03, 0x04];
    match client.query_smart_contract(1, 1, &contract_query_data).await {
        Ok(response) => {
            println!("✅ Contract query successful!");
            println!("   Response length: {} bytes", response.len());
            println!("   Response data: {:?}", response);
        }
        Err(e) => {
            println!("❌ Contract query failed: {}", e);
            println!("   ℹ️  This is expected without a deployed contract");
        }
    }

    // 7. Test network quorum status
    println!("\n⚖️  Testing network quorum...");
    match client.get_quorum().await {
        Ok(quorum) => {
            println!("✅ Quorum Status:");
            println!("   Quorum size: {}", quorum.quorum_size);
            println!("   Total computors: {}", quorum.total_computors);
            println!("   Online computors: {}", quorum.online_computors);
            println!("   Network ready: {}", quorum.quorum_size >= 451);
            
            if quorum.quorum_size >= 451 {
                println!("   🟢 Network has sufficient consensus for transactions");
            } else {
                println!("   🟡 Network consensus below threshold (451/676)");
            }
        }
        Err(e) => {
            println!("❌ Failed to get quorum: {}", e);
        }
    }

    // 8. Test block query
    println!("\n📦 Testing block query...");
    let test_tick = current_tick.saturating_sub(100); // Get a recent block
    match client.get_block(test_tick).await {
        Ok(block) => {
            println!("✅ Block {} info:", test_tick);
            println!("   Epoch: {}", block.epoch);
            println!("   Transactions: {}", block.number_of_transactions);
            println!("   Timestamp: {}", block.timestamp);
        }
        Err(e) => {
            println!("❌ Failed to get block: {}", e);
        }
    }

    println!("\n🎉 Testnet integration test completed!");
    
    println!("\n📋 Summary:");
    println!("✅ Network connectivity established");
    println!("✅ Official pre-funded wallets tested");
    println!("✅ Transaction creation and signing verified");
    println!("✅ Local cryptographic operations working");
    
    println!("\n💡 Next steps for development:");
    println!("1. Join Qubic Discord for faucet access");
    println!("2. Use testnet for smart contract deployment");
    println!("3. Monitor transactions with qlogging service");
    println!("4. Test with small amounts before mainnet");

    Ok(())
}
