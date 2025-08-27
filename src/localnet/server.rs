use anyhow::Result;
use colored::*;
use std::sync::{Arc, Mutex};
use tokio::net::TcpListener;

use super::{api, QubicState};

pub struct LocalNet {
    port: u16,
    state: Arc<Mutex<QubicState>>,
    reset_on_start: bool,
}

impl LocalNet {
    pub fn new(port: u16, reset: bool) -> Self {
        Self {
            port,
            state: Arc::new(Mutex::new(QubicState::new())),
            reset_on_start: reset,
        }
    }
    
    pub async fn start(&mut self) -> Result<()> {
        if self.reset_on_start {
            self.state.lock().unwrap().reset()?;
        }
        
        println!();
        println!("{}", "🌐 QAnchor Local Qubic Network".bold().cyan());
        println!("   {} http://127.0.0.1:{}", "Address:".dimmed(), self.port.to_string().cyan());
        println!("   {} {}", "Network:".dimmed(), "local".green());
        println!("   {} {}", "Status:".dimmed(), "starting...".yellow());
        println!();
        
        let app = api::create_router(self.state.clone());
        let listener = TcpListener::bind(format!("127.0.0.1:{}", self.port)).await?;
        
        println!("{} Local Qubic network started successfully!", "✅".green());
        println!();
        println!("{}", "Available endpoints:".bold());
        println!("  {} GET  /health", "•".cyan());
        println!("  {} POST /contracts", "•".cyan());
        println!("  {} GET  /contracts/:id", "•".cyan());  
        println!("  {} POST /contracts/:id/call", "•".cyan());
        println!("  {} GET  /blocks", "•".cyan());
        println!();
        println!("{}", "Example usage:".dimmed());
        println!("  curl http://127.0.0.1:{}/health", self.port);
        println!();
        println!("{} Press Ctrl+C to stop the server", "💡".blue());
        println!();
        
        // 啟動服務器
        axum::serve(listener, app).await?;
        
        Ok(())
    }
}
