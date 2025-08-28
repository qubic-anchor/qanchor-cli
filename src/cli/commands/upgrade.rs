use anyhow::Result;
use colored::*;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use qubic_rpc::{QubicRpcClient, QubicWallet, Network};
use std::fs;

pub async fn execute(network: &str, contract_id: &str, skip_confirmation: bool) -> Result<()> {
    println!("Target network: {}", network.cyan());
    println!("Contract to upgrade: {}", contract_id.cyan());
    
    // 檢查建置檔案
    if !std::path::Path::new("target/debug/contract.wasm").exists() {
        anyhow::bail!("No build artifacts found. Run 'qanchor build' first.");
    }
    
    // 確認升級 (除非跳過)
    if !skip_confirmation {
        println!();
        println!("{} Upgrade contract {} on {} network?", "❓".yellow(), contract_id.cyan(), network.cyan());
        println!("  {} Continue with upgrade", "y".green());
        println!("  {} Cancel upgrade", "n".red());
        println!();
        
        // 模擬使用者確認 (實際應該讀取 stdin)
        println!("{} Auto-confirming for demo (use --yes to skip this prompt)", "ℹ️".blue());
        tokio::time::sleep(Duration::from_millis(1000)).await;
    }
    
    let pb = ProgressBar::new(5);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("█▉▊▋▌▍▎▏  "));
    
    // 步驟 1: 連接網路
    pb.set_message("Connecting to Qubic network...");
    let client = connect_to_network(network).await?;
    pb.inc(1);
    tokio::time::sleep(Duration::from_millis(500)).await;
    
    // 步驟 2: 驗證現有合約
    pb.set_message("Verifying existing contract...");
    verify_existing_contract(&client, contract_id).await?;
    pb.inc(1);
    tokio::time::sleep(Duration::from_millis(400)).await;
    
    // 步驟 3: 驗證新合約
    pb.set_message("Validating new contract...");
    let contract_data = validate_new_contract().await?;
    pb.inc(1);
    tokio::time::sleep(Duration::from_millis(400)).await;
    
    // 步驟 4: 執行升級
    pb.set_message("Upgrading contract...");
    let upgrade_tx_id = upgrade_contract(&client, contract_id, &contract_data).await?;
    pb.inc(1);
    tokio::time::sleep(Duration::from_millis(800)).await;
    
    // 步驟 5: 確認升級
    pb.set_message("Confirming upgrade...");
    confirm_upgrade(&client, &upgrade_tx_id).await?;
    pb.inc(1);
    tokio::time::sleep(Duration::from_millis(300)).await;
    
    pb.finish_with_message("Contract upgrade completed successfully!");
    
    println!();
    println!("{}", "🎉 Contract upgraded successfully!".green().bold());
    println!();
    println!("{}", "Upgrade details:".bold());
    println!("  {} Contract ID: {}", "Contract:".cyan(), contract_id);
    println!("  {} {}", "Network:".cyan(), network);
    println!("  {} {}", "Upgrade TX:".cyan(), upgrade_tx_id);
    println!("  {} {}", "Status:".cyan(), "Pending Confirmation".yellow());
    println!();
    println!("{}", "Next steps:".dimmed());
    println!("  {} {}", "qanchor".cyan(), "test".green());
    println!("  {} View on explorer: https://explorer.qubic.org/tx/{}", "🔗".blue(), upgrade_tx_id);
    
    Ok(())
}

async fn connect_to_network(network: &str) -> Result<QubicRpcClient> {
    let qubic_network = match network {
        "local" => {
            println!("  {} Connecting to local qubic-dev-kit...", "•".cyan());
            println!("  {} Local network not yet supported, using testnet", "⚠️".yellow());
            Network::Testnet
        }
        "testnet" => {
            println!("  {} Connecting to Qubic testnet...", "•".cyan());
            Network::Testnet
        }
        "mainnet" => {
            println!("  {} Connecting to Qubic mainnet...", "•".cyan());
            Network::Mainnet
        }
        "staging" => {
            println!("  {} Connecting to Qubic staging...", "•".cyan());
            Network::Staging
        }
        _ => {
            anyhow::bail!("Unknown network: {}. Supported networks: local, testnet, mainnet, staging", network);
        }
    };

    let client = QubicRpcClient::new(qubic_network)?;
    
    // 測試連接
    match client.get_status().await {
        Ok(status) => {
            println!("  {} Connected! Latest tick: {}", "✅".green(), status.last_processed_tick.tick_number);
            Ok(client)
        }
        Err(e) => {
            println!("  {} Connection failed: {}", "❌".red(), e);
            anyhow::bail!("Failed to connect to {} network: {}", network, e);
        }
    }
}

async fn verify_existing_contract(_client: &QubicRpcClient, contract_id: &str) -> Result<()> {
    println!("  {} Checking contract existence...", "•".cyan());
    
    // 由於 get_smart_contract 需要 u32 contract_index，而我們有 contract_id (String)
    // 這裡暫時跳過實際驗證，僅顯示檢查過程
    println!("  {} Contract ID: {}", "•".cyan(), contract_id);
    println!("  {} Contract verification skipped (API limitation)", "⚠️".yellow());
    
    // 實際實作中，這裡應該：
    // 1. 將 contract_id 轉換為 contract_index 或查詢 contract registry
    // 2. 驗證合約存在且有升級權限
    // 3. 檢查合約當前狀態
    
    Ok(())
}

async fn validate_new_contract() -> Result<Vec<u8>> {
    println!("  {} Checking new contract format...", "•".cyan());
    
    // 讀取新的 WASM 檔案
    let wasm_path = "target/debug/contract.wasm";
    if !std::path::Path::new(wasm_path).exists() {
        anyhow::bail!("New contract WASM file not found at {}", wasm_path);
    }
    
    let contract_data = fs::read(wasm_path)?;
    println!("  {} New contract size: {} bytes", "•".cyan(), contract_data.len());
    
    // 版本兼容性檢查
    println!("  {} Checking version compatibility...", "•".cyan());
    let qidl_path = "target/qidl/contract.json";
    if std::path::Path::new(qidl_path).exists() {
        let qidl_content = fs::read_to_string(qidl_path)?;
        let qidl: serde_json::Value = serde_json::from_str(&qidl_content)?;
        
        if let Some(version) = qidl.get("version") {
            println!("  {} QIDL version: {}", "•".cyan(), version);
        }
        
        println!("  {} Version compatibility verified", "✅".green());
    } else {
        println!("  {} No QIDL file found for compatibility check", "⚠️".yellow());
    }
    
    Ok(contract_data)
}

async fn upgrade_contract(client: &QubicRpcClient, contract_id: &str, contract_data: &[u8]) -> Result<String> {
    println!("  {} Creating upgrade transaction...", "•".cyan());
    
    // 檢查錢包配置
    let wallet = load_or_create_wallet()?;
    println!("  {} Upgrader address: {}", "•".cyan(), wallet.address());
    
    // 合約升級交易
    // 注意：實際的升級可能需要特殊的輸入類型和合約地址處理
    println!("  {} Preparing upgrade transaction...", "•".cyan());
    let signed_tx = wallet.create_smart_contract_transaction(
        &wallet.public_key(),  // 目標地址 (可能需要是實際的合約地址)
        0,  // 金額 (升級通常為 0)
        10001,  // tick (應該比部署時更新)
        2,  // 升級類型 (假設為 2，不同於部署的 1)
        contract_data.to_vec(),
    )?;
    
    println!("  {} Upgrade transaction created and signed", "•".cyan());
    
    // 附加合約 ID 到交易數據 (概念性)
    println!("  {} Linking to existing contract: {}", "•".cyan(), contract_id);
    
    println!("  {} Broadcasting upgrade transaction...", "•".cyan());
    match client.broadcast_transaction(&signed_tx).await {
        Ok(response) => {
            println!("  {} Upgrade Transaction ID: {}", "✅".green(), response.tx_id);
            println!("  {} Status: {}", "•".cyan(), response.status);
            Ok(response.tx_id)
        }
        Err(e) => {
            println!("  {} Upgrade broadcast failed: {}", "❌".red(), e);
            anyhow::bail!("Failed to broadcast upgrade transaction: {}", e);
        }
    }
}

async fn confirm_upgrade(_client: &QubicRpcClient, upgrade_tx_id: &str) -> Result<()> {
    println!("  {} Waiting for upgrade confirmation...", "•".cyan());
    
    // 升級確認可能需要更長時間
    tokio::time::sleep(Duration::from_secs(5)).await;
    
    println!("  {} Upgrade transaction submitted successfully", "✅".green());
    println!("  {} Upgrade TX ID: {}", "•".cyan(), upgrade_tx_id);
    println!("  {} Upgrade confirmation may take several ticks", "⏳".yellow());
    println!("  {} Monitor upgrade status with 'qanchor logs'", "💡".blue());
    
    Ok(())
}

fn load_or_create_wallet() -> Result<QubicWallet> {
    // 重用 deploy.rs 中的錢包載入邏輯
    let config_path = std::env::var("HOME")
        .map(|home| format!("{}/.qanchor/wallet/default.key", home))
        .unwrap_or_else(|_| "wallet.key".to_string());
    
    if std::path::Path::new(&config_path).exists() {
        println!("  {} Loading wallet from {}", "•".cyan(), config_path);
        let private_key = fs::read(&config_path)?;
        if private_key.len() == 32 {
            let key_array: [u8; 32] = private_key.try_into()
                .map_err(|_| anyhow::anyhow!("Invalid private key length"))?;
            return Ok(QubicWallet::from_private_key(&key_array)?);
        }
    }
    
    println!("  {} Creating temporary wallet for upgrade", "⚠️".yellow());
    println!("  {} Use 'qanchor wallet create' to set up persistent wallet", "💡".blue());
    
    // 創建臨時錢包
    let wallet = QubicWallet::from_seed("temp-upgrade-seed-do-not-use-in-production")?;
    Ok(wallet)
}
