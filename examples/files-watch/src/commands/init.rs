//! Init command implementation

use anyhow::Result;
use colored::Colorize;
use std::path::PathBuf;

use crate::config::{Config, WatchConfig};

pub async fn handle_init(
    local_path: PathBuf,
    remote: String,
    direction: String,
    ignore: Vec<String>,
) -> Result<()> {
    // Validate direction
    if !["up", "down", "both"].contains(&direction.as_str()) {
        anyhow::bail!(
            "Invalid direction '{}'. Must be 'up', 'down', or 'both'",
            direction
        );
    }

    // Validate local path exists
    if !local_path.exists() {
        anyhow::bail!("Local path does not exist: {}", local_path.display());
    }

    if !local_path.is_dir() {
        anyhow::bail!("Local path is not a directory: {}", local_path.display());
    }

    // Canonicalize the path
    let local_path = local_path.canonicalize()?;

    // Load config
    let mut config = Config::load()?;

    // Create watch config
    let watch_config = WatchConfig {
        local_path: local_path.clone(),
        remote_path: remote.clone(),
        direction: direction.clone(),
        ignore_patterns: ignore.clone(),
    };

    // Add to config
    config.add_watch(watch_config)?;

    println!("{}", "✓ Initialized sync configuration".green().bold());
    println!("  Local:     {}", local_path.display());
    println!("  Remote:    {}", remote);
    println!("  Direction: {}", direction);

    if !ignore.is_empty() {
        println!("  Ignore:    {}", ignore.join(", "));
    }

    println!("\n{}", "Run 'files-watch start' to begin syncing".yellow());

    Ok(())
}
