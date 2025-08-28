//! 主網只讀開發示例
//! 
//! 展示如何在沒有測試網的情況下，使用主網進行安全的只讀開發

use qubic_rpc::{
    QubicRpcClient, QubicWallet, Network, RetryConfig, NetworkHealthChecker
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🔍 主網只讀開發模式");
    println!("==================");
    println!("⚠️  僅進行只讀操作，不廣播任何交易");

    // 使用保守重試策略避免速率限制
    let client = QubicRpcClient::with_retry_config(
        Network::Mainnet,
        RetryConfig::conservative()
    )?;

    // 1. 網路狀態監控開發
    println!("\n📊 開發網路狀態監控功能:");
    match client.get_status().await {
        Ok(status) => {
            println!("   ✅ 當前 tick: {}", status.last_processed_tick.tick_number);
            println!("   ✅ 當前 epoch: {}", status.last_processed_tick.epoch);
            println!("   ✅ 跳過的 tick 範圍: {}", status.skipped_ticks.len());
            
            // 分析網路活動
            let latest_tick = status.last_processed_tick.tick_number;
            println!("   📈 網路活躍度分析:");
            println!("      - 最新處理的 tick: {}", latest_tick);
            println!("      - 空 tick 統計: {:?}", status.empty_ticks_per_epoch);
        }
        Err(e) => {
            println!("   ❌ 狀態查詢失敗: {}", e);
        }
    }

    // 2. 錢包和地址生成開發
    println!("\n👛 開發錢包管理功能:");
    
    // 使用開發用種子（不是真實資金）
    let dev_seeds = [
        "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa", // 55個a
        "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb", // 55個b
        "ccccccccccccccccccccccccccccccccccccccccccccccccccccccc", // 55個c
    ];

    for (i, seed) in dev_seeds.iter().enumerate() {
        let wallet = QubicWallet::from_seed(seed)?;
        println!("   錢包 {}: ", i + 1);
        println!("      地址: {}", wallet.address());
        println!("      公鑰: {}", hex::encode(wallet.public_key()));
        
        // 查詢餘額（安全的只讀操作）
        match client.get_balance(&wallet.public_key()).await {
            Ok(balance) => {
                println!("      餘額: {} QU", balance);
            }
            Err(e) => {
                println!("      餘額查詢: {} (預期行為)", e);
            }
        }
    }

    // 3. 交易建構和簽名開發（不廣播）
    println!("\n💸 開發交易建構和簽名功能:");
    let wallet1 = QubicWallet::from_seed(&dev_seeds[0])?;
    let wallet2 = QubicWallet::from_seed(&dev_seeds[1])?;
    
    if let Ok(status) = client.get_status().await {
        let future_tick = status.last_processed_tick.tick_number + 1000;
        
        // 創建轉帳交易
        let transfer_tx = wallet1.create_transfer(
            &wallet2.public_key(),
            1000, // 1000 QU
            future_tick
        )?;
        
        println!("   ✅ 轉帳交易已創建:");
        println!("      從: {}", hex::encode(transfer_tx.transaction.source_public_key));
        println!("      到: {}", hex::encode(transfer_tx.transaction.destination_public_key));
        println!("      金額: {} QU", transfer_tx.transaction.amount);
        println!("      目標 tick: {}", transfer_tx.transaction.tick);
        
        // 驗證簽名
        let is_valid = wallet1.verify_transaction(&transfer_tx)?;
        println!("      簽名驗證: {}", if is_valid { "✅ 有效" } else { "❌ 無效" });
        
        // 智能合約交易示例
        let contract_tx = wallet1.create_smart_contract_transaction(
            &[0x42; 32], // 假想的合約地址
            500,         // 500 QU
            future_tick + 1,
            1,           // 輸入類型
            vec![0x01, 0x02, 0x03, 0x04] // 假想的輸入數據
        )?;
        
        println!("   ✅ 智能合約交易已創建:");
        println!("      合約: {}", hex::encode(contract_tx.transaction.destination_public_key));
        println!("      輸入類型: {}", contract_tx.transaction.input_type);
        println!("      輸入數據: {:?}", contract_tx.transaction.input_data);
        
        println!("   ⚠️  交易已創建但不會廣播（安全開發模式）");
    }

    // 4. 網路健康監控開發
    println!("\n🏥 開發網路健康監控:");
    let health_checker = NetworkHealthChecker::new(RetryConfig::conservative());
    
    for _ in 0..3 {
        let health = health_checker.check_health(&client).await?;
        println!("   狀態: {:?}", health.status);
        println!("   描述: {}", health.status_description());
        println!("   響應時間: {:?}", health.response_time);
        
        if !health.is_usable() {
            println!("   ⚠️  網路不可用，等待後重試...");
            tokio::time::sleep(Duration::from_secs(2)).await;
        } else {
            break;
        }
    }

    // 5. 錯誤處理和重試邏輯開發
    println!("\n🔄 開發錯誤處理和重試邏輯:");
    
    // 測試不同的錯誤情況
    let test_scenarios = [
        ("實體查詢 (預期 404)", || async {
            client.get_entity(&[0u8; 32]).await
        }),
        ("仲裁查詢 (預期 404)", || async {
            client.get_quorum().await
        }),
        ("智能合約查詢 (預期錯誤)", || async {
            client.query_smart_contract(999, 1, &[0x01, 0x02]).await
        }),
    ];

    for (description, test_fn) in test_scenarios.iter() {
        println!("   測試: {}", description);
        match test_fn().await {
            Ok(_) => println!("      ✅ 意外成功"),
            Err(e) => println!("      ❌ 預期錯誤: {}", e),
        }
    }

    println!("\n🎯 開發總結:");
    println!("===========");
    println!("✅ 網路狀態監控 - 完全可開發");
    println!("✅ 錢包管理 - 完全可開發");
    println!("✅ 交易建構 - 完全可開發");
    println!("✅ 簽名驗證 - 完全可開發");
    println!("✅ 錯誤處理 - 完全可開發");
    println!("✅ 重試邏輯 - 完全可開發");
    println!("⚠️  交易廣播 - 需要測試網或小額主網測試");
    println!("⚠️  智能合約互動 - 需要已部署的合約");

    println!("\n💡 無測試網開發建議:");
    println!("==================");
    println!("• 使用主網進行只讀操作開發");
    println!("• 本地測試所有簽名和驗證邏輯");
    println!("• 使用模擬數據進行端到端測試");
    println!("• 實作全面的錯誤處理和重試機制");
    println!("• 準備測試網恢復後的驗證腳本");

    Ok(())
}
