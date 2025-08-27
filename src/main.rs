mod cli;
mod config;
mod templates;
mod utils;
mod error;
// Phase 2 新增模組
mod qidl;
mod generators;
mod localnet;

use cli::commands::Commands;
use clap::Parser;
use colored::*;

#[derive(Parser)]
#[command(
    name = "qanchor",
    about = "The Anchor for Qubic - Modern development framework",
    version = "0.1.0",
    long_about = "QAnchor brings the smooth Solana Anchor development experience to Qubic blockchain.\n\nIf you know Anchor, you know QAnchor!"
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // ASCII Art banner
    println!("{}", "🚀 QAnchor".bold().cyan());
    println!("{}", "The Anchor for Qubic".dimmed());
    println!();
    
    let cli = Cli::parse();
    
    match cli.command.execute().await {
        Ok(_) => {
            println!();
            println!("{}", "✅ Command completed successfully!".green().bold());
            Ok(())
        }
        Err(e) => {
            eprintln!();
            eprintln!("{} {}", "❌ Error:".red().bold(), e);
            std::process::exit(1);
        }
    }
}