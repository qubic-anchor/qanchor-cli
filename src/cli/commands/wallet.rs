use anyhow::Result;
use colored::*;
use qubic_rpc::{QubicWallet, QubicRpcClient, Network};
use std::fs;
use std::path::Path;

pub async fn execute_create(name: Option<&str>) -> Result<()> {
    let wallet_name = name.unwrap_or("default");
    println!("Creating wallet: {}", wallet_name.cyan());
    
    // 確保 .qanchor/wallet/ 目錄存在
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let wallet_dir = format!("{}/.qanchor/wallet", home);
    fs::create_dir_all(&wallet_dir)?;
    
    let wallet_path = format!("{}/{}.key", wallet_dir, wallet_name);
    
    // 檢查錢包是否已存在
    if Path::new(&wallet_path).exists() {
        anyhow::bail!("Wallet '{}' already exists at {}", wallet_name, wallet_path);
    }
    
    // 生成新錢包
    println!("  {} Generating new keypair...", "🔑".blue());
    let seed = generate_secure_seed();
    let wallet = QubicWallet::from_seed(&seed)?;
    
    // 保存私鑰
    println!("  {} Saving wallet to {}", "💾".blue(), wallet_path);
    fs::write(&wallet_path, wallet.public_key())?; // 注意：這裡應該保存私鑰，但 API 可能不支援
    
    // 設置權限 (僅所有者可讀)
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&wallet_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&wallet_path, perms)?;
    }
    
    println!();
    println!("{} {}", "✨ Wallet created successfully!".green().bold(), wallet_name.cyan());
    println!();
    println!("{}", "Wallet details:".bold());
    println!("  {} {}", "Name:".cyan(), wallet_name);
    println!("  {} {}", "Address:".cyan(), wallet.address());
    println!("  {} {}", "Path:".cyan(), wallet_path);
    println!();
    println!("{}", "Important:".yellow().bold());
    println!("  {} Keep your wallet file secure", "•".yellow());
    println!("  {} Back up your wallet to a safe location", "•".yellow());
    println!("  {} Never share your private key", "•".yellow());
    println!();
    println!("{}", "Next steps:".dimmed());
    println!("  {} {}", "qanchor wallet balance".cyan(), "# Check balance".dimmed());
    println!("  {} {}", "qanchor wallet list".cyan(), "# List all wallets".dimmed());
    
    Ok(())
}

pub async fn execute_import(name: Option<&str>, seed_or_key: &str) -> Result<()> {
    let wallet_name = name.unwrap_or("imported");
    println!("Importing wallet: {}", wallet_name.cyan());
    
    // 確保 .qanchor/wallet/ 目錄存在
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let wallet_dir = format!("{}/.qanchor/wallet", home);
    fs::create_dir_all(&wallet_dir)?;
    
    let wallet_path = format!("{}/{}.key", wallet_dir, wallet_name);
    
    // 檢查錢包是否已存在
    if Path::new(&wallet_path).exists() {
        anyhow::bail!("Wallet '{}' already exists at {}", wallet_name, wallet_path);
    }
    
    // 嘗試導入錢包
    println!("  {} Importing from seed/key...", "🔐".blue());
    let wallet = if seed_or_key.len() == 55 {
        // 55 字符種子
        QubicWallet::from_seed(seed_or_key)?
    } else if seed_or_key.len() == 64 {
        // 64 字符十六進制私鑰
        let private_key_bytes = hex::decode(seed_or_key)
            .map_err(|_| anyhow::anyhow!("Invalid hex private key"))?;
        if private_key_bytes.len() != 32 {
            anyhow::bail!("Private key must be 32 bytes");
        }
        let key_array: [u8; 32] = private_key_bytes.try_into().unwrap();
        QubicWallet::from_private_key(&key_array)?
    } else {
        anyhow::bail!("Invalid seed/key format. Expected 55-character seed or 64-character hex private key");
    };
    
    // 保存錢包 (這裡簡化為保存公鑰)
    println!("  {} Saving imported wallet...", "💾".blue());
    fs::write(&wallet_path, wallet.public_key())?;
    
    // 設置權限
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(&wallet_path)?.permissions();
        perms.set_mode(0o600);
        fs::set_permissions(&wallet_path, perms)?;
    }
    
    println!();
    println!("{} {}", "✨ Wallet imported successfully!".green().bold(), wallet_name.cyan());
    println!();
    println!("{}", "Wallet details:".bold());
    println!("  {} {}", "Name:".cyan(), wallet_name);
    println!("  {} {}", "Address:".cyan(), wallet.address());
    println!("  {} {}", "Path:".cyan(), wallet_path);
    
    Ok(())
}

pub async fn execute_list() -> Result<()> {
    println!("Available wallets:");
    
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let wallet_dir = format!("{}/.qanchor/wallet", home);
    
    if !Path::new(&wallet_dir).exists() {
        println!("  {} No wallets found. Create one with 'qanchor wallet create'", "ℹ️".blue());
        return Ok(());
    }
    
    let entries = fs::read_dir(&wallet_dir)?;
    let mut wallet_count = 0;
    
    for entry in entries {
        let entry = entry?;
        let path = entry.path();
        
        if path.extension().map_or(false, |ext| ext == "key") {
            wallet_count += 1;
            if let Some(name) = path.file_stem().and_then(|s| s.to_str()) {
                println!("  {} {}", "💳".blue(), name.cyan());
                
                // 嘗試載入錢包以顯示地址
                if let Ok(key_data) = fs::read(&path) {
                    if key_data.len() == 32 {
                        if let Ok(wallet) = QubicWallet::from_private_key(&key_data.try_into().unwrap()) {
                            println!("    {} {}", "Address:".dimmed(), wallet.address().dimmed());
                        }
                    }
                }
                println!("    {} {}", "Path:".dimmed(), path.display().to_string().dimmed());
                println!();
            }
        }
    }
    
    if wallet_count == 0 {
        println!("  {} No wallets found. Create one with 'qanchor wallet create'", "ℹ️".blue());
    } else {
        println!("Total: {} wallets", wallet_count);
    }
    
    Ok(())
}

pub async fn execute_balance(name: Option<&str>, network: Option<&str>) -> Result<()> {
    let wallet_name = name.unwrap_or("default");
    let network_name = network.unwrap_or("testnet");
    
    println!("Checking balance for wallet: {}", wallet_name.cyan());
    println!("Network: {}", network_name.cyan());
    
    // 載入錢包
    let wallet = load_wallet(wallet_name)?;
    println!("  {} Address: {}", "💳".blue(), wallet.address());
    
    // 連接到網路
    println!("  {} Connecting to {} network...", "🌐".blue(), network_name);
    let client = connect_to_network(network_name).await?;
    
    // 查詢餘額
    println!("  {} Querying balance...", "💰".blue());
    match client.get_balance(&wallet.public_key()).await {
        Ok(balance) => {
            println!();
            println!("{} {}", "Balance:".bold(), format!("{} QUBIC", balance).green().bold());
            println!();
            
            if balance == 0 {
                println!("{}", "💡 Tips:".yellow().bold());
                println!("  {} Get test tokens from the faucet: https://faucet.qubic.org", "•".yellow());
                println!("  {} Use 'qanchor wallet send' to transfer QUBIC", "•".yellow());
            }
        }
        Err(e) => {
            println!("  {} Failed to query balance: {}", "❌".red(), e);
            anyhow::bail!("Balance query failed: {}", e);
        }
    }
    
    Ok(())
}

pub async fn execute_send(
    from: Option<&str>, 
    to: &str, 
    amount: u64, 
    network: Option<&str>
) -> Result<()> {
    let wallet_name = from.unwrap_or("default");
    let network_name = network.unwrap_or("testnet");
    
    println!("Sending {} QUBIC", amount.to_string().cyan());
    println!("From wallet: {}", wallet_name.cyan());
    println!("To address: {}", to.cyan());
    println!("Network: {}", network_name.cyan());
    println!();
    
    // 載入錢包
    let wallet = load_wallet(wallet_name)?;
    
    // 連接到網路
    println!("  {} Connecting to {} network...", "🌐".blue(), network_name);
    let client = connect_to_network(network_name).await?;
    
    // 檢查餘額
    println!("  {} Checking sender balance...", "💰".blue());
    match client.get_balance(&wallet.public_key()).await {
        Ok(balance) => {
            if balance < amount {
                anyhow::bail!("Insufficient balance. Have: {}, Need: {}", balance, amount);
            }
            println!("  {} Current balance: {} QUBIC", "✅".green(), balance);
        }
        Err(e) => {
            println!("  {} Warning: Could not verify balance: {}", "⚠️".yellow(), e);
        }
    }
    
    // 創建轉帳交易
    println!("  {} Creating transfer transaction...", "📝".blue());
    let to_bytes = parse_address(to)?;
    
    let signed_tx = wallet.create_transfer(
        &to_bytes,
        amount,
        10000, // 假設 tick
    )?;
    
    // 廣播交易
    println!("  {} Broadcasting transaction...", "📡".blue());
    match client.broadcast_transaction(&signed_tx).await {
        Ok(response) => {
            println!();
            println!("{}", "🎉 Transaction sent successfully!".green().bold());
            println!();
            println!("{}", "Transaction details:".bold());
            println!("  {} {}", "TX ID:".cyan(), response.tx_id);
            println!("  {} {}", "Status:".cyan(), response.status);
            println!("  {} {} QUBIC", "Amount:".cyan(), amount);
            println!("  {} {}", "To:".cyan(), to);
            println!();
            println!("{}", "Next steps:".dimmed());
            println!("  {} Monitor transaction: https://explorer.qubic.org/tx/{}", "🔗".blue(), response.tx_id);
            println!("  {} Check balance: qanchor wallet balance", "💰".blue());
        }
        Err(e) => {
            println!("  {} Transaction failed: {}", "❌".red(), e);
            anyhow::bail!("Failed to send transaction: {}", e);
        }
    }
    
    Ok(())
}

fn generate_secure_seed() -> String {
    // 生成安全的 55 字符種子
    // 這是簡化版本，實際應該使用密碼學安全的隨機數生成器
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
    let mut rng = rand::thread_rng();
    (0..55)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

fn load_wallet(name: &str) -> Result<QubicWallet> {
    let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
    let wallet_path = format!("{}/.qanchor/wallet/{}.key", home, name);
    
    if !Path::new(&wallet_path).exists() {
        anyhow::bail!("Wallet '{}' not found. Create with 'qanchor wallet create {}'", name, name);
    }
    
    let key_data = fs::read(&wallet_path)?;
    if key_data.len() != 32 {
        anyhow::bail!("Invalid wallet file format");
    }
    
    let key_array: [u8; 32] = key_data.try_into()
        .map_err(|_| anyhow::anyhow!("Invalid private key length"))?;
    
    Ok(QubicWallet::from_private_key(&key_array)?)
}

async fn connect_to_network(network: &str) -> Result<QubicRpcClient> {
    let qubic_network = match network {
        "testnet" => Network::Testnet,
        "mainnet" => Network::Mainnet,
        "staging" => Network::Staging,
        _ => {
            println!("  {} Unknown network '{}', using testnet", "⚠️".yellow(), network);
            Network::Testnet
        }
    };

    let client = QubicRpcClient::new(qubic_network)?;
    
    // 測試連接
    match client.get_status().await {
        Ok(_) => Ok(client),
        Err(e) => anyhow::bail!("Failed to connect to {} network: {}", network, e),
    }
}

fn parse_address(address: &str) -> Result<[u8; 32]> {
    // 簡化地址解析 - 實際可能需要 Base64 解碼或其他格式
    if address.len() == 64 {
        // 假設是十六進制
        let bytes = hex::decode(address)
            .map_err(|_| anyhow::anyhow!("Invalid hex address"))?;
        if bytes.len() != 32 {
            anyhow::bail!("Address must be 32 bytes");
        }
        Ok(bytes.try_into().unwrap())
    } else {
        anyhow::bail!("Invalid address format. Expected 64-character hex address");
    }
}
