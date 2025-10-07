//! files-watch - Filesystem sync daemon for Files.com

mod cli;
mod commands;
mod config;
mod conflict;
mod progress;
mod state;
mod syncer;
mod watcher;

use anyhow::Result;
use clap::Parser;
use tracing_subscriber::EnvFilter;

use cli::{Cli, Commands};

#[tokio::main]
async fn main() -> Result<()> {
    // Parse CLI arguments
    let cli = Cli::parse();

    // Setup logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(
            EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(log_level)),
        )
        .init();

    // Dispatch commands
    match cli.command {
        Commands::Init {
            local_path,
            remote,
            direction,
            ignore,
        } => {
            commands::handle_init(local_path, remote, direction, ignore).await?;
        }
        Commands::Start { daemon, path } => {
            commands::handle_start(daemon, path).await?;
        }
        Commands::Status { path } => {
            commands::handle_status(path).await?;
        }
        Commands::Pause { path: _ } => {
            println!("Pause command not yet implemented (Phase 3)");
        }
        Commands::Resume { path: _ } => {
            println!("Resume command not yet implemented (Phase 3)");
        }
        Commands::Sync {
            path,
            direction,
            full,
        } => {
            commands::handle_sync(path, direction, full).await?;
        }
        Commands::List => {
            commands::handle_list().await?;
        }
        Commands::Remove { path: _ } => {
            println!("Remove command not yet implemented");
        }
        Commands::History { path: _, limit: _ } => {
            println!("History command not yet implemented");
        }
    }

    Ok(())
}
