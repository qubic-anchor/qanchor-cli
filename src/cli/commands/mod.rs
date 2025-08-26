use clap::Subcommand;
use anyhow::Result;
use colored::*;

pub mod init;
pub mod build;
pub mod deploy;
pub mod test;

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new QAnchor project
    Init {
        /// Project name
        name: String,
        /// Template to use
        #[arg(short, long, default_value = "basic-oracle")]
        template: String,
        /// Target directory
        #[arg(short, long)]
        path: Option<String>,
    },
    /// Build the project
    Build {
        /// Build configuration
        #[arg(short, long, default_value = "debug")]
        config: String,
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Deploy to network
    Deploy {
        /// Target network
        #[arg(short, long, default_value = "local")]
        network: String,
        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
    },
    /// Run tests
    Test {
        /// Test pattern
        #[arg(short, long)]
        pattern: Option<String>,
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
}

impl Commands {
    pub async fn execute(&self) -> Result<()> {
        match self {
            Commands::Init { name, template, path } => {
                println!("{} {}", "📦 Initializing project:".bold(), name.cyan());
                init::execute(name, template, path.as_deref()).await
            }
            Commands::Build { config, verbose } => {
                println!("{} {}", "🔨 Building project with config:".bold(), config.cyan());
                build::execute(config, *verbose).await
            }
            Commands::Deploy { network, yes } => {
                println!("{} {}", "🚀 Deploying to network:".bold(), network.cyan());
                deploy::execute(network, *yes).await
            }
            Commands::Test { pattern, verbose } => {
                println!("{}", "🧪 Running tests...".bold());
                test::execute(pattern.as_deref(), *verbose).await
            }
        }
    }
}

