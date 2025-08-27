use anyhow::{Result, Context};
use colored::*;
use std::path::Path;
use indicatif::{ProgressBar, ProgressStyle};

use crate::qidl::QidlParser;
use crate::generators::{SdkGenerator, TypeScriptGenerator, PythonGenerator};

pub async fn execute(lang: &str, output: &str, input: &str) -> Result<()> {
    // 驗證輸入檔案
    let input_path = Path::new(input);
    if !input_path.exists() {
        anyhow::bail!("QIDL file not found: {}", input);
    }
    
    // 設置進度條
    let pb = ProgressBar::new(4);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {pos}/{len} {msg}")
            .unwrap()
            .progress_chars("#>-")
    );
    
    // 步驟 1: 解析 QIDL
    pb.set_message("Parsing QIDL file...");
    let qidl_program = QidlParser::parse_file(input_path)
        .with_context(|| format!("Failed to parse QIDL file: {}", input_path.display()))?;
    
    // 驗證 QIDL 結構
    QidlParser::validate(&qidl_program)
        .context("QIDL validation failed")?;
    
    pb.inc(1);
    
    // 步驟 2: 初始化生成器
    pb.set_message("Initializing generator...");
    let generator: Box<dyn SdkGenerator> = match lang.to_lowercase().as_str() {
        "ts" | "typescript" => Box::new(TypeScriptGenerator::new()?),
        "py" | "python" => Box::new(PythonGenerator::new()?),
        _ => anyhow::bail!("Unsupported language: {}. Supported: ts, py", lang),
    };
    
    pb.inc(1);
    
    // 步驟 3: 準備輸出目錄
    pb.set_message("Preparing output directory...");
    let output_path = Path::new(output);
    generator.validate_output_dir(output_path)?;
    
    pb.inc(1);
    
    // 步驟 4: 生成 SDK
    pb.set_message("Generating SDK...");
    generator.generate(&qidl_program, output_path).await
        .context("Failed to generate SDK")?;
    
    pb.inc(1);
    pb.finish_with_message("SDK generation completed!");
    
    // 顯示結果
    println!();
    println!("{}", "✨ SDK Generated Successfully!".green().bold());
    println!();
    println!("📁 Output directory: {}", output_path.display().to_string().cyan());
    println!("🔧 Language: {}", generator.language().cyan());
    println!("📊 Program: {}", qidl_program.program.name.cyan());
    println!("📝 Instructions: {}", qidl_program.instructions.len().to_string().cyan());
    println!("🏗️ Types: {}", qidl_program.types.len().to_string().cyan());
    
    println!();
    println!("Next steps:");
    match generator.language() {
        "TypeScript" => {
            println!("  cd {}", output);
            println!("  npm install");
            println!("  npm run build");
        }
        "Python" => {
            println!("  cd {}", output);
            println!("  pip install -r requirements.txt");
            println!("  python -m pytest tests/");
        }
        _ => {}
    }
    
    Ok(())
}
