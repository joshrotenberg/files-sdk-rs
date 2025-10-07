//! Sync command implementation (one-time sync)

use anyhow::{Context, Result};
use colored::Colorize;
use files_sdk::FilesClient;
use std::env;
use std::path::PathBuf;

use crate::config::Config;
use crate::syncer::Syncer;

pub async fn handle_sync(path: PathBuf, direction: Option<String>, full: bool) -> Result<()> {
    let path = path.canonicalize()?;

    // Load config
    let config = Config::load()?;

    // Find watch config for this path
    let watch_config = config
        .find_watch(&path)
        .ok_or_else(|| anyhow::anyhow!("No watch configuration found for: {}", path.display()))?
        .clone();

    // Override direction if specified
    let mut watch_config = watch_config;
    if let Some(dir) = direction {
        if !["up", "down", "both"].contains(&dir.as_str()) {
            anyhow::bail!(
                "Invalid direction '{}'. Must be 'up', 'down', or 'both'",
                dir
            );
        }
        watch_config.direction = dir;
    }

    // Get API key
    let api_key =
        env::var("FILES_API_KEY").context("FILES_API_KEY environment variable not set")?;

    let client = FilesClient::builder().api_key(api_key).build()?;

    // Create syncer
    let mut syncer = Syncer::new(client, watch_config.clone())?;

    // Clear state if full sync
    if full {
        println!("{}", "Performing full sync (ignoring state)...".cyan());
        // TODO: Clear state
    } else {
        println!("{}", "Performing incremental sync...".cyan());
    }

    println!("  Local:     {}", watch_config.local_path.display());
    println!("  Remote:    {}", watch_config.remote_path);
    println!("  Direction: {}", watch_config.direction);
    println!();

    // Sync all files
    let synced = syncer.sync_all(None).await?;

    println!();
    println!("{} {} files synced", "✓".green(), synced.len());

    Ok(())
}
