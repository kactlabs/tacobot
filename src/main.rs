//! TacoBot CLI entry point
//!
//! This is the main executable for TacoBot, providing command-line interface
//! and initialization of the system.

use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "tacobot")]
#[command(about = "Ultra-lightweight personal AI Assistant for embedded systems", long_about = None)]
#[command(version)]
#[command(author)]
struct Args {
    /// Path to configuration file
    #[arg(short, long, value_name = "FILE", global = true)]
    config: Option<PathBuf>,

    /// Log level (debug, info, warn, error)
    #[arg(short, long, default_value = "info", global = true)]
    log_level: String,

    /// Enable verbose output
    #[arg(short, long, global = true)]
    verbose: bool,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Chat with the agent
    Agent {
        /// Message to send to the agent
        #[arg(short, long)]
        message: Option<String>,
    },
    /// Start the gateway for channel integrations
    Gateway,
    /// Show system status
    Status,
    /// Manage scheduled cron jobs
    Cron {
        #[command(subcommand)]
        action: CronAction,
    },
    /// Initialize configuration and workspace
    Onboard,
}

#[derive(Subcommand, Debug)]
enum CronAction {
    /// List all scheduled jobs
    List,
    /// Add a new scheduled job
    Add {
        /// Cron expression
        #[arg(short, long)]
        expression: String,
        /// Job description
        #[arg(short, long)]
        description: String,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    // Initialize logging
    picoclaw::logging::setup::init_logging(&args.log_level)?;

    info!("Starting TacoBot v{}", env!("CARGO_PKG_VERSION"));
    if let Some(config_path) = &args.config {
        info!("Configuration file: {:?}", config_path);
    }

    match args.command {
        Some(Commands::Agent { message }) => {
            handle_agent(message).await?;
        }
        Some(Commands::Gateway) => {
            handle_gateway().await?;
        }
        Some(Commands::Status) => {
            handle_status().await?;
        }
        Some(Commands::Cron { action }) => {
            handle_cron(action).await?;
        }
        Some(Commands::Onboard) => {
            handle_onboard().await?;
        }
        None => {
            // Default: show help
            println!("TacoBot v{}", env!("CARGO_PKG_VERSION"));
            println!("Ultra-lightweight personal AI Assistant for embedded systems");
            println!("\nUsage: tacobot [OPTIONS] <COMMAND>");
            println!("\nCommands:");
            println!("  agent    Chat with the agent");
            println!("  gateway  Start the gateway for channel integrations");
            println!("  status   Show system status");
            println!("  cron     Manage scheduled cron jobs");
            println!("  onboard  Initialize configuration and workspace");
            println!("\nOptions:");
            println!("  -c, --config <FILE>          Path to configuration file");
            println!("  -l, --log-level <LOG_LEVEL>  Log level (debug, info, warn, error)");
            println!("  -v, --verbose                Enable verbose output");
            println!("  -h, --help                   Print help");
            println!("  -V, --version                Print version");
        }
    }

    info!("TacoBot completed successfully");

    Ok(())
}

async fn handle_agent(message: Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting agent");

    if let Some(msg) = message {
        info!("Processing message: {}", msg);
        println!("Agent received: {}", msg);
        // TODO: Process message through agent loop
        // TODO: Call LLM provider
        // TODO: Return response
    } else {
        info!("Starting interactive agent mode");
        println!("Interactive agent mode (not yet implemented)");
        // TODO: Start interactive REPL
    }

    Ok(())
}

async fn handle_gateway() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting gateway");
    println!("Gateway mode (not yet implemented)");
    // TODO: Initialize channel connections
    // TODO: Start listening for messages
    Ok(())
}

async fn handle_status() -> Result<(), Box<dyn std::error::Error>> {
    info!("Showing status");
    println!("TacoBot v{}", env!("CARGO_PKG_VERSION"));
    println!("Status: OK");
    // TODO: Show actual status information
    Ok(())
}

async fn handle_cron(action: CronAction) -> Result<(), Box<dyn std::error::Error>> {
    match action {
        CronAction::List => {
            info!("Listing cron jobs");
            println!("Cron jobs (not yet implemented)");
            // TODO: List scheduled jobs
        }
        CronAction::Add {
            expression,
            description,
        } => {
            info!("Adding cron job: {} - {}", expression, description);
            println!("Added cron job: {} - {}", expression, description);
            // TODO: Add scheduled job
        }
    }
    Ok(())
}

async fn handle_onboard() -> Result<(), Box<dyn std::error::Error>> {
    info!("Starting onboard process");
    
    let home = std::env::var("HOME")?;
    let workspace_dir = format!("{}/.tacobot/workspace", home);
    let config_path = format!("{}/.tacobot/config.yaml", home);
    
    // Create workspace directory
    std::fs::create_dir_all(&workspace_dir)?;
    println!("✓ Created workspace directory: {}", workspace_dir);
    
    // Create subdirectories
    let subdirs = vec!["sessions", "memory", "state", "cron", "skills"];
    for subdir in subdirs {
        std::fs::create_dir_all(format!("{}/{}", workspace_dir, subdir))?;
    }
    println!("✓ Created workspace subdirectories");
    
    // Create default config if it doesn't exist
    if !std::path::Path::new(&config_path).exists() {
        let default_config = r#"agent:
  max_context_size: 8192
  timeout_ms: 5000
  memory_limit_mb: 10

channels:
  telegram:
    enabled: false
    token: ""
  discord:
    enabled: false
    token: ""

llm:
  default_provider: "openrouter"
  providers:
    openrouter:
      api_key: ""
      model: "meta-llama/llama-2-70b-chat"

logging:
  level: "info"
  format: "json"
"#;
        std::fs::write(&config_path, default_config)?;
        println!("✓ Created default config: {}", config_path);
    } else {
        println!("✓ Config already exists: {}", config_path);
    }
    
    // Create workspace files
    let workspace_files = vec![
        ("AGENTS.md", "# Agent Configuration\n\nConfigure agent behavior here.\n"),
        ("IDENTITY.md", "# Agent Identity\n\nDefine your agent's identity and personality.\n"),
        ("SOUL.md", "# Agent Soul\n\nDefine your agent's core values and principles.\n"),
        ("TOOLS.md", "# Available Tools\n\nList of tools available to the agent.\n"),
        ("USER.md", "# User Preferences\n\nDefine user preferences and settings.\n"),
        ("HEARTBEAT.md", "# Periodic Tasks\n\nDefine tasks to run periodically.\n"),
        ("MEMORY.md", "# Long-term Memory\n\nAgent's long-term memory storage.\n"),
    ];
    
    for (filename, content) in workspace_files {
        let filepath = format!("{}/{}", workspace_dir, filename);
        if !std::path::Path::new(&filepath).exists() {
            std::fs::write(&filepath, content)?;
        }
    }
    println!("✓ Created workspace files");
    
    println!("\n✅ Onboarding complete!");
    println!("\nNext steps:");
    println!("1. Edit config: {}", config_path);
    println!("2. Set your API keys (OPENROUTER_API_KEY, etc.)");
    println!("3. Run: tacobot agent -m \"Hello\"");
    
    info!("Onboarding completed successfully");
    Ok(())
}
