use anyhow::{Result, Context};
use std::path::Path;
use super::QidlProgram;

pub struct QidlParser;

impl QidlParser {
    /// 從檔案解析 QIDL
    pub fn parse_file(path: &Path) -> Result<QidlProgram> {
        let content = std::fs::read_to_string(path)
            .with_context(|| format!("Failed to read QIDL file: {}", path.display()))?;
        
        Self::parse_string(&content)
    }
    
    /// 從字串解析 QIDL
    pub fn parse_string(content: &str) -> Result<QidlProgram> {
        serde_json::from_str(content)
            .with_context(|| {
                // 提供更詳細的錯誤訊息
                format!("Failed to parse QIDL JSON. Content preview: {}", 
                    &content[..content.len().min(200)])
            })
    }
    
    /// 驗證 QIDL 結構
    pub fn validate(program: &QidlProgram) -> Result<()> {
        // 檢查程式名稱
        if program.program.name.is_empty() {
            anyhow::bail!("Program name cannot be empty");
        }
        
        // 檢查指令 (允許空指令列表用於測試)
        if program.instructions.is_empty() {
            println!("⚠️ Warning: Program has no instructions defined");
        }
        
        // 檢查指令名稱唯一性
        let mut instruction_names = std::collections::HashSet::new();
        for instruction in &program.instructions {
            if !instruction_names.insert(&instruction.name) {
                anyhow::bail!("Duplicate instruction name: {}", instruction.name);
            }
        }
        
        // 檢查帳戶類型引用
        let account_names: std::collections::HashSet<_> = 
            program.accounts.iter().map(|a| &a.name).collect();
        
        for instruction in &program.instructions {
            for account_ref in &instruction.accounts {
                if !account_names.contains(&account_ref.account_type) && 
                   account_ref.account_type != "PublicKey" {
                    anyhow::bail!(
                        "Unknown account type '{}' in instruction '{}'", 
                        account_ref.account_type, 
                        instruction.name
                    );
                }
            }
        }
        
        Ok(())
    }
}
