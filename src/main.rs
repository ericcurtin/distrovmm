mod cli;
mod oci;
mod rootfs;
mod vm;

use anyhow::Result;
use clap::Parser;

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .init();

    let cli = cli::Cli::parse();
    cli.execute().await
}
