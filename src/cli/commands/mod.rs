use clap::Subcommand;
use anyhow::Result;
use colored::*;

pub mod init;
pub mod build;
pub mod deploy;
pub mod test;
// Phase 2 新增指令
pub mod generate;
pub mod localnet;
pub mod clean;
// Phase 3 新增指令
pub mod upgrade;
pub mod wallet;
pub mod network;
pub mod logs;

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
        /// Enable network testing with specified network
        #[arg(short, long)]
        network: Option<String>,
    },
    /// Generate SDK for different languages
    Generate {
        /// Target language (ts, py)
        #[arg(short, long)]
        lang: String,
        /// Output directory
        #[arg(short, long, default_value = "./generated")]
        output: String,
        /// QIDL input file
        #[arg(short, long, default_value = "src/oracle.qidl")]
        input: String,
    },
    /// Start local Qubic test network
    Localnet {
        /// Port to bind
        #[arg(short, long, default_value = "8899")]
        port: u16,
        /// Reset state
        #[arg(short, long)]
        reset: bool,
    },
    /// Clean build artifacts and cache
    Clean {
        /// Clean only cache
        #[arg(long)]
        cache_only: bool,
        /// Verbose output
        #[arg(short, long)]
        verbose: bool,
    },
    /// Upgrade an existing contract
    Upgrade {
        /// Contract ID to upgrade
        #[arg(short, long)]
        contract_id: String,
        /// Target network
        #[arg(short, long, default_value = "local")]
        network: String,
        /// Skip confirmation
        #[arg(short, long)]
        yes: bool,
    },
    /// Wallet management commands
    Wallet {
        #[command(subcommand)]
        wallet_command: WalletCommands,
    },
    /// Network management commands
    Network {
        #[command(subcommand)]
        network_command: NetworkCommands,
    },
    /// View contract logs
    Logs {
        /// Contract ID to filter logs (optional)
        #[arg(short, long)]
        contract: Option<String>,
        /// Follow real-time logs
        #[arg(short, long)]
        follow: bool,
        /// Number of recent log entries to show
        #[arg(short, long)]
        tail: Option<u32>,
        /// Show logs since specified time (e.g., '1h', '30m', '1234567890')
        #[arg(short, long)]
        since: Option<String>,
        /// Filter logs by keyword
        #[arg(long)]
        filter: Option<String>,
        /// Target network
        #[arg(short, long)]
        network: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum NetworkCommands {
    /// Check network status
    Status {
        /// Target network
        #[arg(short, long)]
        network: Option<String>,
    },
    /// Ping network nodes
    Ping {
        /// Target network
        #[arg(short, long)]
        network: Option<String>,
        /// Number of ping attempts
        #[arg(short, long)]
        count: Option<u32>,
    },
}

#[derive(Subcommand)]
pub enum WalletCommands {
    /// Create a new wallet
    Create {
        /// Wallet name
        #[arg(short, long)]
        name: Option<String>,
    },
    /// Import a wallet from seed or private key
    Import {
        /// Wallet name
        #[arg(short, long)]
        name: Option<String>,
        /// Seed phrase or private key
        seed_or_key: String,
    },
    /// List all wallets
    List,
    /// Check wallet balance
    Balance {
        /// Wallet name
        #[arg(short, long)]
        name: Option<String>,
        /// Target network
        #[arg(short, long)]
        network: Option<String>,
    },
    /// Send QUBIC tokens
    Send {
        /// Source wallet name
        #[arg(short, long)]
        from: Option<String>,
        /// Destination address
        #[arg(short, long)]
        to: String,
        /// Amount to send
        #[arg(short, long)]
        amount: u64,
        /// Target network
        #[arg(short, long)]
        network: Option<String>,
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
            Commands::Test { pattern, verbose, network } => {
                println!("{}", "🧪 Running tests...".bold());
                test::execute_with_network(pattern.as_deref(), *verbose, network.as_deref()).await
            }
            Commands::Generate { lang, output, input } => {
                println!("{} {} SDK to {}", "🔧 Generating".bold(), lang.cyan(), output.cyan());
                generate::execute(lang, output, input).await
            }
            Commands::Localnet { port, reset } => {
                println!("{} {}", "🌐 Starting local network on port".bold(), port.to_string().cyan());
                localnet::execute(*port, *reset).await
            }
            Commands::Clean { cache_only, verbose } => {
                println!("{}", "🧹 Cleaning project...".bold());
                clean::execute(*cache_only, *verbose).await
            }
            Commands::Upgrade { contract_id, network, yes } => {
                println!("{} {}", "⬆️ Upgrading contract:".bold(), contract_id.cyan());
                upgrade::execute(network, contract_id, *yes).await
            }
            Commands::Wallet { wallet_command } => {
                match wallet_command {
                    WalletCommands::Create { name } => {
                        println!("{}", "💳 Creating wallet...".bold());
                        wallet::execute_create(name.as_deref()).await
                    }
                    WalletCommands::Import { name, seed_or_key } => {
                        println!("{}", "📥 Importing wallet...".bold());
                        wallet::execute_import(name.as_deref(), seed_or_key).await
                    }
                    WalletCommands::List => {
                        println!("{}", "📋 Listing wallets...".bold());
                        wallet::execute_list().await
                    }
                    WalletCommands::Balance { name, network } => {
                        println!("{}", "💰 Checking wallet balance...".bold());
                        wallet::execute_balance(name.as_deref(), network.as_deref()).await
                    }
                    WalletCommands::Send { from, to, amount, network } => {
                        println!("{}", "💸 Sending QUBIC...".bold());
                        wallet::execute_send(from.as_deref(), to, *amount, network.as_deref()).await
                    }
                }
            }
            Commands::Network { network_command } => {
                match network_command {
                    NetworkCommands::Status { network } => {
                        println!("{}", "🌐 Checking network status...".bold());
                        network::execute_status(network.as_deref()).await
                    }
                    NetworkCommands::Ping { network, count } => {
                        println!("{}", "🏓 Pinging network...".bold());
                        network::execute_ping(network.as_deref(), *count).await
                    }
                }
            }
            Commands::Logs { contract, follow, tail, since, filter, network } => {
                println!("{}", "📋 Contract logs viewer...".bold());
                logs::execute(
                    contract.as_deref(),
                    *follow,
                    *tail,
                    since.as_deref(),
                    filter.as_deref(),
                    network.as_deref(),
                ).await
            }
        }
    }
}

