//! Start command implementation

use anyhow::{Context, Result};
use colored::Colorize;
use files_sdk::FilesClient;
use std::env;
use std::path::PathBuf;
use std::sync::Arc;
use tracing::error;

use crate::config::Config;
use crate::daemon::Daemon;
use crate::progress::ProgressBarTracker;
use crate::syncer::Syncer;
use crate::watcher::{FileEvent, FileWatcher};

pub async fn handle_start(daemon: bool, path: Option<PathBuf>) -> Result<()> {
    // Load config
    let config = Config::load()?;

    if config.watch.is_empty() {
        anyhow::bail!("No watch configurations found. Run 'files-watch init' first.");
    }

    // Get API key
    let api_key =
        env::var("FILES_API_KEY").context("FILES_API_KEY environment variable not set")?;

    let client = FilesClient::builder().api_key(api_key).build()?;

    // If daemon mode, run daemon with all watch configs
    if daemon {
        let daemon = Daemon::new(client, config);
        return daemon.run().await;
    }

    // Otherwise, run single watch in foreground
    // Determine which watch configs to start
    let watches = if let Some(ref p) = path {
        let p = p.canonicalize()?;
        config
            .watch
            .iter()
            .filter(|w| w.local_path == p)
            .cloned()
            .collect::<Vec<_>>()
    } else {
        config.watch.clone()
    };

    if watches.is_empty() {
        if let Some(p) = path {
            anyhow::bail!("No watch configuration found for: {}", p.display());
        } else {
            anyhow::bail!("No watch configurations found");
        }
    }

    println!("{}", "Starting files-watch...".green().bold());
    println!();

    // For Phase 1, we only support single watch config
    if watches.len() > 1 {
        println!(
            "{}",
            "Note: Multiple watch configs found, starting first one only (Phase 1 limitation)"
                .yellow()
        );
    }

    let watch_config = watches[0].clone();

    println!("  Watching: {}", watch_config.local_path.display());
    println!("  Remote:   {}", watch_config.remote_path);
    println!("  Direction: {}", watch_config.direction);
    println!();

    // Create syncer
    let mut syncer = Syncer::new(client, watch_config.clone())?;

    // Do initial sync
    println!("{}", "Performing initial sync...".cyan());
    let synced = syncer.sync_all(None).await?;
    println!("{} {} files synced", "✓".green(), synced.len());
    println!();

    // Start file watcher
    println!("{}", "Watching for changes (Ctrl+C to stop)...".cyan());
    let mut watcher = FileWatcher::new(&watch_config.local_path)?;

    // Event loop
    while let Some(event) = watcher.next_event().await {
        match event {
            FileEvent::Created(path) | FileEvent::Modified(path) => {
                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                println!("{} {}", "→".blue(), file_name);

                // Sync the file with progress bar
                let metadata = tokio::fs::metadata(&path).await.ok();
                let size = metadata.map(|m| m.len());
                let progress = Arc::new(ProgressBarTracker::new(size));

                match syncer.sync_file(&path, Some(progress.clone())).await {
                    Ok(()) => {
                        progress.finish();
                        println!("{} {}", "✓".green(), file_name);
                    }
                    Err(e) => {
                        error!("Failed to sync {}: {}", path.display(), e);
                        println!("{} {} - {}", "✗".red(), file_name, e);
                    }
                }
            }
            FileEvent::Deleted(path) => {
                let file_name = path
                    .file_name()
                    .and_then(|n| n.to_str())
                    .unwrap_or("unknown");

                println!("{} {}", "✗".red(), file_name);

                if let Err(e) = syncer.handle_delete(&path).await {
                    error!("Failed to handle delete for {}: {}", path.display(), e);
                }
            }
        }
    }

    Ok(())
}
