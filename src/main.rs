//! PicoClaw CLI entry point
//!
//! This is the main executable for PicoClaw, providing command-line interface
//! and initialization of the system.

use clap::Parser;
use std::path::PathBuf;
use tracing::info;

#[derive(Parser, Debug)]
#[command(name = "picoclaw")]
#[command(about = "Ultra-lightweight personal AI Assistant for embedded systems", long_about = None)]
#[command(version)]
#[command(author)]
struct Args {
    /// Path to configuration file
    #[arg(short, long, value_name = "FILE")]
    config: Option<PathBuf>,

    /// Log level (debug, info, warn, error)
    #[arg(short, long, default_value = "info")]
    log_level: String,

    /// Enable verbose output
    #[arg(short, long)]
    verbose: bool,

    /// Show version and exit
    #[arg(long)]
    version: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    if args.version {
        println!("picoclaw {}", env!("CARGO_PKG_VERSION"));
        return Ok(());
    }

    // Initialize logging
    picoclaw::logging::setup::init_logging(&args.log_level)?;

    info!("Starting PicoClaw v{}", env!("CARGO_PKG_VERSION"));
    info!("Configuration file: {:?}", args.config);

    // TODO: Load configuration
    // TODO: Initialize async runtime
    // TODO: Start agent loop
    // TODO: Connect to channels

    info!("PicoClaw started successfully");

    Ok(())
}
