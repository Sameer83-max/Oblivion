use clap::{Parser, Subcommand};
use log::info;
use std::path::PathBuf;

mod core;
mod crypto;
mod certificates;
mod platform;
mod cli;
mod error;
mod utils;

use error::Result;

/// Secure Disk Erasure Tool - Cross-platform secure data sanitization
#[derive(Parser)]
#[command(name = "secure-disk-erasure")]
#[command(about = "A cross-platform secure disk erasure tool with certificate generation")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// List available storage devices
    List {
        /// Show detailed device information
        #[arg(short, long)]
        detailed: bool,
    },
    /// Securely erase a storage device
    Wipe {
        /// Target device path (e.g., /dev/sda, \\.\PhysicalDrive0)
        #[arg(short, long)]
        device: PathBuf,
        
        /// Erase mode: quick, full, or advanced
        #[arg(short, long, default_value = "full")]
        mode: String,
        
        /// Generate certificate after wipe
        #[arg(short, long)]
        certificate: bool,
        
        /// Output directory for certificates
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
    },
    /// Verify a wipe certificate
    Verify {
        /// Path to certificate file
        #[arg(short, long)]
        certificate: PathBuf,
        
        /// Public key for verification
        #[arg(short, long)]
        public_key: Option<PathBuf>,
    },
    /// Generate signing key pair
    GenerateKeys {
        /// Output directory for keys
        #[arg(short, long, default_value = ".")]
        output: PathBuf,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    env_logger::Builder::from_default_env()
        .filter_level(if cli.verbose {
            log::LevelFilter::Debug
        } else {
            log::LevelFilter::Info
        })
        .init();
    
    info!("Starting Secure Disk Erasure Tool");
    
    // Log system information
    utils::Utils::log_system_info();
    
    // Check privileges
    if let Err(e) = utils::Utils::check_privileges() {
        log::warn!("Privilege check failed: {}", e);
    }
    
    match cli.command {
        Commands::List { detailed } => {
            cli::list_devices(detailed).await?;
        }
        Commands::Wipe { device, mode, certificate, output } => {
            cli::wipe_device(device, mode, certificate, output).await?;
        }
        Commands::Verify { certificate, public_key } => {
            cli::verify_certificate(certificate, public_key).await?;
        }
        Commands::GenerateKeys { output } => {
            cli::generate_keys(output).await?;
        }
    }
    
    info!("Operation completed successfully");
    Ok(())
}
