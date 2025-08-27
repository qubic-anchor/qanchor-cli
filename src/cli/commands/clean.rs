use anyhow::Result;
use colored::*;
use std::path::Path;
use walkdir::WalkDir;

pub async fn execute(cache_only: bool, verbose: bool) -> Result<()> {
    let mut cleaned_items = Vec::new();
    let mut total_size = 0u64;
    
    // 清理目標目錄
    let targets = if cache_only {
        vec![".qanchor-cache", "temp"]
    } else {
        vec!["target", ".qanchor-cache", "generated", "node_modules", "temp"]
    };
    
    for target in targets {
        let path = Path::new(target);
        if path.exists() {
            if verbose {
                println!("🧹 Cleaning: {}", path.display().to_string().cyan());
            }
            
            // 計算目錄大小
            if let Ok(size) = calculate_dir_size(path) {
                total_size += size;
            }
            
            // 移除目錄
            if path.is_dir() {
                std::fs::remove_dir_all(path)?;
                cleaned_items.push(target);
            }
        }
    }
    
    // 顯示結果
    if cleaned_items.is_empty() {
        println!("{}", "✨ Nothing to clean!".green());
    } else {
        println!("{}", "🧹 Cleaned:".bold());
        for item in cleaned_items {
            println!("  • {}", item);
        }
        
        if total_size > 0 {
            let size_mb = total_size as f64 / 1024.0 / 1024.0;
            println!();
            println!("💾 Freed: {:.2} MB", size_mb);
        }
    }
    
    Ok(())
}

fn calculate_dir_size(path: &Path) -> Result<u64> {
    let mut size = 0;
    
    for entry in WalkDir::new(path) {
        let entry = entry?;
        if entry.file_type().is_file() {
            size += entry.metadata()?.len();
        }
    }
    
    Ok(size)
}
