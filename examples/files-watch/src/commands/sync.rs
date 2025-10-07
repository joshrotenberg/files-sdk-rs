//! Sync command implementation (one-time sync)

use anyhow::{Context, Result};
use colored::Colorize;
use files_sdk::FilesClient;
use std::env;
use std::path::PathBuf;

use crate::config::Config;
use crate::conflict::ConflictResolution;
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

    // Perform sync based on direction
    match watch_config.direction.as_str() {
        "up" => {
            // Upload only
            let synced = syncer.sync_all(None).await?;
            println!();
            println!("{} {} files uploaded", "✓".green(), synced.len());
        }
        "down" => {
            // Download only
            println!("{}", "Scanning remote directory...".cyan());
            let remote_files = syncer.scan_remote().await?;

            println!(
                "{}",
                format!("Found {} remote files", remote_files.len()).cyan()
            );

            let mut downloaded = 0;
            for (remote_path, size, mtime) in remote_files {
                // Get relative path
                let remote_base = watch_config.remote_path.trim_end_matches('/');
                if let Some(relative) = remote_path.strip_prefix(remote_base) {
                    let relative = relative.trim_start_matches('/');
                    let local_path = watch_config.local_path.join(relative);

                    // Download file
                    if let Err(e) = syncer
                        .download_file(&remote_path, &local_path, size, mtime, None)
                        .await
                    {
                        eprintln!("{} Failed to download {}: {}", "✗".red(), relative, e);
                    } else {
                        downloaded += 1;
                    }
                }
            }

            println!();
            println!("{} {} files downloaded", "✓".green(), downloaded);
        }
        "both" => {
            // Bidirectional sync
            let conflict_resolution = ConflictResolution::from_str(&config.conflict.resolution)
                .unwrap_or(ConflictResolution::Newest);

            println!(
                "{}",
                format!("Conflict resolution: {}", config.conflict.resolution).cyan()
            );

            syncer.sync_bidirectional(None, conflict_resolution).await?;

            println!();
            println!("{} Bidirectional sync complete", "✓".green());
        }
        _ => {
            anyhow::bail!("Invalid direction: {}", watch_config.direction);
        }
    }

    Ok(())
}
