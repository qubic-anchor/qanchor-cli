use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contract {
    pub id: String,
    pub name: String,
    pub code: Vec<u8>,
    pub deployed_at: u64,
    pub status: ContractStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContractStatus {
    Active,
    Paused,
    Disabled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub contract_id: String,
    pub method: String,
    pub args: serde_json::Value,
    pub result: Option<serde_json::Value>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    pub height: u64,
    pub hash: String,
    pub timestamp: u64,
    pub transactions: Vec<String>, // Transaction IDs
}

#[derive(Debug)]
pub struct QubicState {
    pub contracts: HashMap<String, Contract>,
    pub transactions: HashMap<String, Transaction>,
    pub blocks: Vec<Block>,
    pub current_height: u64,
}

impl QubicState {
    pub fn new() -> Self {
        Self {
            contracts: HashMap::new(),
            transactions: HashMap::new(),
            blocks: Vec::new(),
            current_height: 0,
        }
    }
    
    pub fn reset(&mut self) -> anyhow::Result<()> {
        self.contracts.clear();
        self.transactions.clear();
        self.blocks.clear();
        self.current_height = 0;
        
        // 建立創世區塊
        let genesis_block = Block {
            height: 0,
            hash: "0x000000000000000000000000000000000000000000000000000000000000000".to_string(),
            timestamp: chrono::Utc::now().timestamp() as u64,
            transactions: Vec::new(),
        };
        
        self.blocks.push(genesis_block);
        self.current_height = 1;
        
        println!("✅ Local Qubic network state reset");
        Ok(())
    }
    
    pub fn deploy_contract(&mut self, name: String, code: Vec<u8>) -> String {
        let contract_id = format!("QC{}", rand::random::<u32>());
        let contract = Contract {
            id: contract_id.clone(),
            name,
            code,
            deployed_at: chrono::Utc::now().timestamp() as u64,
            status: ContractStatus::Active,
        };
        
        self.contracts.insert(contract_id.clone(), contract);
        contract_id
    }
    
    pub fn call_contract(&mut self, contract_id: &str, method: &str, args: serde_json::Value) -> anyhow::Result<serde_json::Value> {
        if !self.contracts.contains_key(contract_id) {
            anyhow::bail!("Contract {} not found", contract_id);
        }
        
        let tx_id = Uuid::new_v4().to_string();
        
        // 模擬合約執行結果
        let result = match method {
            "initialize" => serde_json::json!({"status": "initialized"}),
            "update_price" => serde_json::json!({"status": "price_updated"}),
            "get_price" => serde_json::json!({
                "symbol": args.get("asset_id").unwrap_or(&serde_json::Value::String("BTC".to_string())),
                "price": 50000,
                "timestamp": chrono::Utc::now().timestamp()
            }),
            _ => serde_json::json!({"status": "method_not_found"}),
        };
        
        let transaction = Transaction {
            id: tx_id.clone(),
            contract_id: contract_id.to_string(),
            method: method.to_string(),
            args,
            result: Some(result.clone()),
            timestamp: chrono::Utc::now().timestamp() as u64,
        };
        
        self.transactions.insert(tx_id, transaction);
        Ok(result)
    }
    
    pub fn get_latest_block(&self) -> Option<&Block> {
        self.blocks.last()
    }
}
