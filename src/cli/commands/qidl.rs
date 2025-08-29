//! QIDL 命令處理模組

use anyhow::Result;
use clap::Subcommand;
use std::path::PathBuf;
use crate::qidl::{QidlGenerator, QidlValidator, QidlDiffer};

#[derive(Subcommand, Clone)]
pub enum QidlCommands {
    /// 從源碼生成 QIDL 檔案
    Generate {
        /// 源碼檔案路徑
        #[arg(short, long, default_value = "src/lib.rs")]
        source: PathBuf,
        
        /// 輸出 QIDL 檔案路徑
        #[arg(short, long, default_value = "target/qidl/contract.json")]
        output: PathBuf,
        
        /// 程式名稱
        #[arg(short, long)]
        name: Option<String>,
        
        /// 覆蓋現有檔案
        #[arg(long)]
        force: bool,
    },
    
    /// 驗證 QIDL 檔案
    Validate {
        /// QIDL 檔案路徑
        #[arg(default_value = "target/qidl/contract.json")]
        file: PathBuf,
    },
    
    /// 比較兩個 QIDL 檔案的差異
    Diff {
        /// 舊 QIDL 檔案
        old: PathBuf,
        
        /// 新 QIDL 檔案
        new: PathBuf,
    },
    
    /// 格式化 QIDL 檔案
    Format {
        /// QIDL 檔案路徑
        #[arg(default_value = "target/qidl/contract.json")]
        file: PathBuf,
        
        /// 就地格式化
        #[arg(short, long)]
        in_place: bool,
    },
}

pub fn execute(command: QidlCommands) -> Result<()> {
    match command {
        QidlCommands::Generate { source, output, name, force } => {
            execute_generate(source, output, name, force)
        }
        QidlCommands::Validate { file } => {
            execute_validate(file)
        }
        QidlCommands::Diff { old, new } => {
            execute_diff(old, new)
        }
        QidlCommands::Format { file, in_place } => {
            execute_format(file, in_place)
        }
    }
}

fn execute_generate(source: PathBuf, output: PathBuf, name: Option<String>, force: bool) -> Result<()> {
    use colored::*;
    
    println!("{}", "🔄 生成 QIDL...".cyan());
    
    // 檢查源碼檔案是否存在
    if !source.exists() {
        anyhow::bail!("源碼檔案不存在: {}", source.display());
    }
    
    // 檢查輸出檔案是否已存在
    if output.exists() && !force {
        anyhow::bail!(
            "輸出檔案已存在: {}\n使用 --force 覆蓋現有檔案",
            output.display()
        );
    }
    
    // 自動推斷程式名稱
    let program_name = name.unwrap_or_else(|| {
        source.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("my_program")
            .to_string()
    });
    
    println!("  📂 源碼檔案: {}", source.display());
    println!("  📄 輸出檔案: {}", output.display());
    println!("  📝 程式名稱: {}", program_name);
    
    // 生成 QIDL
    let generator = QidlGenerator::new(
        source.to_str().unwrap(),
        output.to_str().unwrap(),
        &program_name,
    );
    
    let qidl = generator.generate()?;
    
    println!("{}", "✅ QIDL 生成成功!".green());
    println!("  📊 指令數量: {}", qidl.instructions.len());
    println!("  🏗️  帳戶類型: {}", qidl.accounts.len());
    println!("  🔧 自定義類型: {}", qidl.types.len());
    println!("  📋 錯誤定義: {}", qidl.errors.len());
    
    Ok(())
}

fn execute_validate(file: PathBuf) -> Result<()> {
    use colored::*;
    
    println!("{}", "🔍 驗證 QIDL...".cyan());
    
    if !file.exists() {
        anyhow::bail!("QIDL 檔案不存在: {}", file.display());
    }
    
    println!("  📄 檔案: {}", file.display());
    
    QidlValidator::validate_file(&file)?;
    
    println!("{}", "✅ QIDL 驗證通過!".green());
    
    Ok(())
}

fn execute_diff(old: PathBuf, new: PathBuf) -> Result<()> {
    use colored::*;
    
    println!("{}", "📊 比較 QIDL 差異...".cyan());
    
    if !old.exists() {
        anyhow::bail!("舊 QIDL 檔案不存在: {}", old.display());
    }
    
    if !new.exists() {
        anyhow::bail!("新 QIDL 檔案不存在: {}", new.display());
    }
    
    println!("  📄 舊檔案: {}", old.display());
    println!("  📄 新檔案: {}", new.display());
    
    QidlDiffer::diff_files(&old, &new)?;
    
    println!("{}", "✅ 差異比較完成!".green());
    
    Ok(())
}

fn execute_format(file: PathBuf, in_place: bool) -> Result<()> {
    use colored::*;
    use std::fs;
    
    println!("{}", "🎨 格式化 QIDL...".cyan());
    
    if !file.exists() {
        anyhow::bail!("QIDL 檔案不存在: {}", file.display());
    }
    
    // 讀取並解析 QIDL
    let content = fs::read_to_string(&file)?;
    let qidl: crate::qidl::QidlProgram = serde_json::from_str(&content)?;
    
    // 重新格式化
    let formatted = serde_json::to_string_pretty(&qidl)?;
    
    if in_place {
        // 就地格式化
        fs::write(&file, formatted)?;
        println!("  ✅ 檔案已格式化: {}", file.display());
    } else {
        // 輸出到標準輸出
        println!("\n{}", "格式化結果:".bold());
        println!("{}", formatted);
    }
    
    println!("{}", "✅ QIDL 格式化完成!".green());
    
    Ok(())
}
