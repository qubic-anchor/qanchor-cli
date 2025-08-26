use anyhow::{Result, Context};
use colored::*;
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};
use std::time::Duration;
use crate::templates::Template;

pub async fn execute(name: &str, template: &str, target_path: Option<&str>) -> Result<()> {
    // 驗證專案名稱
    if !is_valid_project_name(name) {
        anyhow::bail!("Invalid project name '{}'. Project names must contain only letters, numbers, hyphens, and underscores.", name);
    }
    
    // 決定目標路徑
    let project_path = match target_path {
        Some(path) => Path::new(path).join(name),
        None => Path::new(name).to_path_buf(),
    };
    
    // 檢查目錄是否已存在
    if project_path.exists() {
        anyhow::bail!("Directory '{}' already exists", project_path.display());
    }
    
    // 進度條設定
    let pb = ProgressBar::new(4);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .unwrap()
        .progress_chars("█▉▊▋▌▍▎▏  "));
    
    // 步驟 1: 建立專案目錄
    pb.set_message("Creating project directory...");
    std::fs::create_dir_all(&project_path)
        .context("Failed to create project directory")?;
    pb.inc(1);
    tokio::time::sleep(Duration::from_millis(200)).await;
    
    // 步驟 2: 載入範本
    pb.set_message("Loading template...");
    let template_engine = Template::new(template)?;
    pb.inc(1);
    tokio::time::sleep(Duration::from_millis(200)).await;
    
    // 步驟 3: 生成檔案
    pb.set_message("Generating project files...");
    template_engine.generate(&project_path, name).await
        .context("Failed to generate project files")?;
    pb.inc(1);
    tokio::time::sleep(Duration::from_millis(200)).await;
    
    // 步驟 4: 完成
    pb.set_message("Finalizing project...");
    pb.inc(1);
    tokio::time::sleep(Duration::from_millis(200)).await;
    
    pb.finish_with_message("Project created successfully!");
    
    // 成功訊息
    println!();
    println!("{} {}", "✨ Successfully created".green(), name.bold());
    println!();
    println!("{}", "Next steps:".bold());
    println!("  {} {}", "cd".cyan(), name);
    println!("  {} {}", "qanchor".cyan(), "build".green());
    println!("  {} {}", "qanchor".cyan(), "deploy --network local".green());
    println!();
    println!("{}", "Happy coding with QAnchor! 🚀".dimmed());
    
    Ok(())
}

fn is_valid_project_name(name: &str) -> bool {
    !name.is_empty() 
        && name.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_')
        && !name.starts_with('-')
        && !name.ends_with('-')
}

