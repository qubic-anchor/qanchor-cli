//! QIDL 自動生成器
//! 
//! 從 qanchor-lang 程式自動生成 QIDL 介面定義

use anyhow::{Result, Context};
use std::path::Path;
use std::fs;
use crate::qidl::{QidlProgram, QidlBuilder, Instruction, Account};
use chrono::Utc;

/// QIDL 生成器
pub struct QidlGenerator {
    /// 源碼路徑
    source_path: String,
    /// 輸出路徑
    output_path: String,
    /// 程式名稱
    program_name: String,
}

impl QidlGenerator {
    /// 創建新的 QIDL 生成器
    pub fn new(source_path: &str, output_path: &str, program_name: &str) -> Self {
        Self {
            source_path: source_path.to_string(),
            output_path: output_path.to_string(),
            program_name: program_name.to_string(),
        }
    }
    
    /// 生成 QIDL 檔案
    pub fn generate(&self) -> Result<QidlProgram> {
        println!("🔄 正在從源碼生成 QIDL...");
        
        // 讀取源碼檔案
        let source_content = fs::read_to_string(&self.source_path)
            .with_context(|| format!("無法讀取源碼檔案: {}", self.source_path))?;
        
        // 解析源碼生成 QIDL
        let qidl = self.parse_source_code(&source_content)?;
        
        // 寫入 QIDL 檔案
        self.write_qidl_file(&qidl)?;
        
        println!("✅ QIDL 生成完成: {}", self.output_path);
        Ok(qidl)
    }
    
    /// 解析源碼生成 QIDL
    fn parse_source_code(&self, content: &str) -> Result<QidlProgram> {
        let mut builder = QidlBuilder::new(&self.program_name, "0.1.0")
            .description(&format!("{} 智能合約", self.program_name))
            .author("QAnchor Developer");
        
        // 簡化版解析 - 在實際實現中需要使用 syn crate 進行 AST 解析
        // 這裡只是演示框架
        
        // 查找 #[program] 巨集
        if content.contains("#[program]") {
            println!("  📦 發現 #[program] 模組");
            
            // 解析指令
            let instructions = self.extract_instructions(content)?;
            for instruction in instructions {
                builder = builder.instruction(instruction);
            }
            
            // 解析帳戶結構
            let accounts = self.extract_accounts(content)?;
            for account in accounts {
                builder = builder.account_type(account);
            }
        } else {
            println!("  ⚠️  未找到 #[program] 巨集，生成基本 QIDL");
        }
        
        let mut qidl = builder.build();
        
        // 添加元資料
        qidl.metadata.compiler_version = Some(env!("CARGO_PKG_VERSION").to_string());
        qidl.metadata.generated_at = Some(Utc::now().to_rfc3339());
        qidl.metadata.source_hash = Some(self.calculate_source_hash(content));
        
        Ok(qidl)
    }
    
    /// 提取指令定義
    fn extract_instructions(&self, _content: &str) -> Result<Vec<Instruction>> {
        // 簡化實現 - 實際需要 AST 解析
        println!("  🔍 解析指令定義...");
        
        let mut instructions = Vec::new();
        
        // 示例：添加一個基本指令
        instructions.push(Instruction {
            name: "initialize".to_string(),
            description: "初始化程式".to_string(),
            args: vec![],
            accounts: vec![],
            returns: None,
            discriminator: Some([0x17, 0x5a, 0x77, 0x2a, 0xd1, 0x1f, 0x68, 0x3b]), // 示例
            example: Some("await program.methods.initialize().rpc()".to_string()),
        });
        
        Ok(instructions)
    }
    
    /// 提取帳戶結構定義
    fn extract_accounts(&self, _content: &str) -> Result<Vec<Account>> {
        // 簡化實現 - 實際需要 AST 解析
        println!("  🔍 解析帳戶結構...");
        
        let mut accounts = Vec::new();
        
        // 示例：添加一個基本帳戶類型
        accounts.push(Account {
            name: format!("{}State", self.program_name),
            description: "程式狀態帳戶".to_string(),
            fields: vec![],
        });
        
        Ok(accounts)
    }
    
    /// 計算源碼雜湊值
    fn calculate_source_hash(&self, content: &str) -> String {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        content.hash(&mut hasher);
        format!("{:x}", hasher.finish())
    }
    
    /// 寫入 QIDL 檔案
    fn write_qidl_file(&self, qidl: &QidlProgram) -> Result<()> {
        let json = serde_json::to_string_pretty(qidl)
            .context("無法序列化 QIDL")?;
        
        // 確保輸出目錄存在
        if let Some(parent) = Path::new(&self.output_path).parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("無法創建輸出目錄: {}", parent.display()))?;
        }
        
        fs::write(&self.output_path, json)
            .with_context(|| format!("無法寫入 QIDL 檔案: {}", self.output_path))?;
        
        Ok(())
    }
}

/// QIDL 驗證器
pub struct QidlValidator;

impl QidlValidator {
    /// 驗證 QIDL 檔案
    pub fn validate_file(path: &Path) -> Result<()> {
        println!("🔍 驗證 QIDL 檔案: {}", path.display());
        
        let content = fs::read_to_string(path)
            .with_context(|| format!("無法讀取 QIDL 檔案: {}", path.display()))?;
        
        let qidl: QidlProgram = serde_json::from_str(&content)
            .with_context(|| "QIDL JSON 格式錯誤")?;
        
        Self::validate_qidl(&qidl)?;
        
        println!("✅ QIDL 驗證通過");
        Ok(())
    }
    
    /// 驗證 QIDL 結構
    pub fn validate_qidl(qidl: &QidlProgram) -> Result<()> {
        // 檢查基本結構
        if qidl.program.name.is_empty() {
            anyhow::bail!("程式名稱不能為空");
        }
        
        if qidl.version.is_empty() {
            anyhow::bail!("版本不能為空");
        }
        
        // 檢查指令唯一性
        let mut instruction_names = std::collections::HashSet::new();
        for instruction in &qidl.instructions {
            if !instruction_names.insert(&instruction.name) {
                anyhow::bail!("重複的指令名稱: {}", instruction.name);
            }
        }
        
        // 檢查帳戶類型唯一性
        let mut account_names = std::collections::HashSet::new();
        for account in &qidl.accounts {
            if !account_names.insert(&account.name) {
                anyhow::bail!("重複的帳戶類型名稱: {}", account.name);
            }
        }
        
        // 檢查類型引用
        let type_names: std::collections::HashSet<_> = qidl.types.iter()
            .map(|t| &t.name)
            .collect();
        
        let all_type_names: std::collections::HashSet<_> = account_names
            .union(&type_names)
            .collect();
        
        // 檢查指令中的類型引用
        for instruction in &qidl.instructions {
            for account_ref in &instruction.accounts {
                if !all_type_names.contains(&&account_ref.account_type) && 
                   !Self::is_builtin_type(&account_ref.account_type) {
                    anyhow::bail!(
                        "指令 '{}' 中未知的帳戶類型: '{}'", 
                        instruction.name, 
                        account_ref.account_type
                    );
                }
            }
        }
        
        println!("  ✅ 基本結構驗證通過");
        println!("  ✅ 指令唯一性驗證通過 ({} 個指令)", qidl.instructions.len());
        println!("  ✅ 帳戶類型唯一性驗證通過 ({} 個類型)", qidl.accounts.len());
        println!("  ✅ 類型引用驗證通過");
        
        Ok(())
    }
    
    /// 檢查是否為內建類型
    fn is_builtin_type(type_name: &str) -> bool {
        matches!(type_name, 
            "PublicKey" | "u8" | "u16" | "u32" | "u64" | "i8" | "i16" | "i32" | "i64" |
            "bool" | "string" | "bytes" | "Signer" | "SystemProgram"
        )
    }
}

/// QIDL 差異比較器
pub struct QidlDiffer;

impl QidlDiffer {
    /// 比較兩個 QIDL 檔案的差異
    pub fn diff_files(old_path: &Path, new_path: &Path) -> Result<()> {
        println!("🔄 比較 QIDL 差異...");
        
        let old_content = fs::read_to_string(old_path)
            .with_context(|| format!("無法讀取舊 QIDL: {}", old_path.display()))?;
        let new_content = fs::read_to_string(new_path)
            .with_context(|| format!("無法讀取新 QIDL: {}", new_path.display()))?;
        
        let old_qidl: QidlProgram = serde_json::from_str(&old_content)?;
        let new_qidl: QidlProgram = serde_json::from_str(&new_content)?;
        
        Self::diff_qidl(&old_qidl, &new_qidl);
        
        Ok(())
    }
    
    /// 比較兩個 QIDL 的差異
    pub fn diff_qidl(old: &QidlProgram, new: &QidlProgram) {
        println!("📊 QIDL 變更摘要:");
        
        // 比較指令
        let old_instructions: std::collections::HashSet<_> = 
            old.instructions.iter().map(|i| &i.name).collect();
        let new_instructions: std::collections::HashSet<_> = 
            new.instructions.iter().map(|i| &i.name).collect();
        
        for added in new_instructions.difference(&old_instructions) {
            println!("  ➕ 新增指令: {}", added);
        }
        
        for removed in old_instructions.difference(&new_instructions) {
            println!("  ➖ 移除指令: {}", removed);
        }
        
        // 比較帳戶類型
        let old_accounts: std::collections::HashSet<_> = 
            old.accounts.iter().map(|a| &a.name).collect();
        let new_accounts: std::collections::HashSet<_> = 
            new.accounts.iter().map(|a| &a.name).collect();
        
        for added in new_accounts.difference(&old_accounts) {
            println!("  ➕ 新增帳戶類型: {}", added);
        }
        
        for removed in old_accounts.difference(&new_accounts) {
            println!("  ➖ 移除帳戶類型: {}", removed);
        }
        
        // 版本變更
        if old.version != new.version {
            println!("  📈 版本變更: {} → {}", old.version, new.version);
        }
        
        if old_instructions == new_instructions && old_accounts == new_accounts {
            println!("  ✅ 無重大變更");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_qidl_generator() {
        let temp_dir = tempdir().unwrap();
        let source_path = temp_dir.path().join("lib.rs");
        let output_path = temp_dir.path().join("test.qidl");
        
        // 創建測試源碼
        fs::write(&source_path, r#"
            use qanchor_lang::prelude::*;
            
            #[program]
            pub mod test_program {
                use super::*;
                
                pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
                    Ok(())
                }
            }
            
            #[derive(Accounts)]
            pub struct Initialize<'info> {
                #[account(init, payer = user, space = 8 + 32)]
                pub test_account: Account<'info, TestAccount>,
                #[account(mut)]
                pub user: Signer<'info>,
            }
            
            #[account]
            pub struct TestAccount {
                pub data: [u8; 32],
            }
        "#).unwrap();
        
        let generator = QidlGenerator::new(
            source_path.to_str().unwrap(),
            output_path.to_str().unwrap(),
            "test_program"
        );
        
        let qidl = generator.generate().unwrap();
        
        assert_eq!(qidl.program.name, "test_program");
        assert!(!qidl.instructions.is_empty());
        assert!(output_path.exists());
    }
    
    #[test]
    fn test_qidl_validator() {
        let qidl = QidlBuilder::new("test", "1.0.0")
            .description("Test program")
            .build();
        
        QidlValidator::validate_qidl(&qidl).unwrap();
    }
}
