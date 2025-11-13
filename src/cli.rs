use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "distrovmm")]
#[command(version)]
#[command(about = "Run full-system Linux virtual machines as easily as containers", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Run a distribution as a VM
    Run {
        /// Distribution name (e.g., fedora, ubuntu, fedora:43, ubuntu:24.04)
        image: String,
    },
}

impl Cli {
    pub async fn execute(self) -> Result<()> {
        match self.command {
            Commands::Run { image } => crate::vm::run_vm(&image).await,
        }
    }
}
