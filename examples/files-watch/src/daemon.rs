//! Daemon mode for continuous background monitoring

use anyhow::Result;
use colored::Colorize;
use files_sdk::FilesClient;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::time::interval;
use tracing::{error, info};

use crate::config::{Config, WatchConfig};
use crate::conflict::ConflictResolution;
use crate::progress::ProgressBarTracker;
use crate::syncer::Syncer;
use crate::watcher::{FileEvent, FileWatcher};

/// Daemon manager for running multiple watch configurations
pub struct Daemon {
    client: FilesClient,
    config: Config,
}

impl Daemon {
    /// Create a new daemon instance
    pub fn new(client: FilesClient, config: Config) -> Self {
        Self { client, config }
    }

    /// Run the daemon with all configured watch paths
    pub async fn run(&self) -> Result<()> {
        if self.config.watch.is_empty() {
            anyhow::bail!("No watch configurations found. Run 'files-watch init' first.");
        }

        println!("{}", "Starting files-watch daemon...".green().bold());
        println!();
        println!("  Watch configs: {}", self.config.watch.len());
        println!(
            "  Check interval: {}s",
            self.config.sync.check_interval_secs
        );
        println!();

        // Spawn a task for each watch configuration
        let mut tasks = Vec::new();

        for watch_config in &self.config.watch {
            let client = self.client.clone();
            let config = self.config.clone();
            let watch_config = watch_config.clone();

            let task = tokio::spawn(async move {
                if let Err(e) = Self::run_watch(client, config, watch_config).await {
                    error!("Watch task failed: {}", e);
                }
            });

            tasks.push(task);
        }

        // Wait for all tasks (they run indefinitely unless there's an error)
        for task in tasks {
            task.await?;
        }

        Ok(())
    }

    /// Run a single watch configuration
    async fn run_watch(
        client: FilesClient,
        config: Config,
        watch_config: WatchConfig,
    ) -> Result<()> {
        info!(
            "Starting watch: {} -> {}",
            watch_config.local_path.display(),
            watch_config.remote_path
        );

        println!(
            "{} Watching: {} -> {}",
            "→".blue(),
            watch_config.local_path.display(),
            watch_config.remote_path
        );

        // Create syncer
        let mut syncer = Syncer::new(client, watch_config.clone())?;

        // Perform initial sync based on direction
        Self::perform_sync(&mut syncer, &watch_config, &config).await?;

        // If direction is "up" or "both", watch for local changes
        let watch_local = watch_config.direction == "up" || watch_config.direction == "both";

        // If direction is "down" or "both", poll for remote changes
        let watch_remote = watch_config.direction == "down" || watch_config.direction == "both";

        // Spawn file watcher task if needed
        let watcher_task = if watch_local {
            let mut syncer_clone = syncer.clone();
            let watch_config_clone = watch_config.clone();

            Some(tokio::spawn(async move {
                if let Err(e) = Self::run_file_watcher(&mut syncer_clone, &watch_config_clone).await
                {
                    error!("File watcher error: {}", e);
                }
            }))
        } else {
            None
        };

        // Spawn remote polling task if needed
        let poller_task = if watch_remote {
            let mut syncer_clone = syncer.clone();
            let watch_config_clone = watch_config.clone();
            let config_clone = config.clone();

            Some(tokio::spawn(async move {
                if let Err(e) =
                    Self::run_remote_poller(&mut syncer_clone, &watch_config_clone, &config_clone)
                        .await
                {
                    error!("Remote poller error: {}", e);
                }
            }))
        } else {
            None
        };

        // Wait for tasks to complete (they run indefinitely)
        if let Some(task) = watcher_task {
            task.await?;
        }
        if let Some(task) = poller_task {
            task.await?;
        }

        Ok(())
    }

    /// Perform initial sync based on direction
    async fn perform_sync(
        syncer: &mut Syncer,
        watch_config: &WatchConfig,
        config: &Config,
    ) -> Result<()> {
        println!(
            "{} Performing initial sync for {}...",
            "⟳".cyan(),
            watch_config.local_path.display()
        );

        match watch_config.direction.as_str() {
            "up" => {
                let synced = syncer.sync_all(None).await?;
                println!(
                    "{} {} files uploaded from {}",
                    "✓".green(),
                    synced.len(),
                    watch_config.local_path.display()
                );
            }
            "down" => {
                let remote_files = syncer.scan_remote().await?;
                for (remote_path, size, mtime) in remote_files {
                    let local_path = watch_config.local_path.join(
                        remote_path
                            .strip_prefix(&watch_config.remote_path)
                            .unwrap_or(&remote_path)
                            .trim_start_matches('/'),
                    );
                    syncer
                        .download_file(&remote_path, &local_path, size, mtime, None)
                        .await?;
                }
                println!(
                    "{} Downloaded files to {}",
                    "✓".green(),
                    watch_config.local_path.display()
                );
            }
            "both" => {
                let conflict_resolution =
                    ConflictResolution::from_str(&config.conflict.resolution)?;
                syncer.sync_bidirectional(None, conflict_resolution).await?;
                println!(
                    "{} Bidirectional sync completed for {}",
                    "✓".green(),
                    watch_config.local_path.display()
                );
            }
            _ => anyhow::bail!("Invalid direction: {}", watch_config.direction),
        }

        Ok(())
    }

    /// Run file watcher for local changes
    async fn run_file_watcher(syncer: &mut Syncer, watch_config: &WatchConfig) -> Result<()> {
        let mut watcher = FileWatcher::new(&watch_config.local_path)?;

        while let Some(event) = watcher.next_event().await {
            match event {
                FileEvent::Created(path) | FileEvent::Modified(path) => {
                    let file_name = path
                        .file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("unknown");

                    info!("File changed: {}", path.display());

                    let metadata = tokio::fs::metadata(&path).await.ok();
                    let size = metadata.map(|m| m.len());
                    let progress = Arc::new(ProgressBarTracker::new(size));

                    match syncer.sync_file(&path, Some(progress.clone())).await {
                        Ok(()) => {
                            progress.finish();
                            println!(
                                "{} {} [{}]",
                                "✓".green(),
                                file_name,
                                watch_config.local_path.display()
                            );
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

                    info!("File deleted: {}", path.display());

                    if let Err(e) = syncer.handle_delete(&path).await {
                        error!("Failed to handle delete for {}: {}", path.display(), e);
                    } else {
                        println!(
                            "{} {} [{}]",
                            "✗".yellow(),
                            file_name,
                            watch_config.local_path.display()
                        );
                    }
                }
            }
        }

        Ok(())
    }

    /// Poll for remote changes
    async fn run_remote_poller(
        syncer: &mut Syncer,
        watch_config: &WatchConfig,
        config: &Config,
    ) -> Result<()> {
        let mut ticker = interval(Duration::from_secs(config.sync.check_interval_secs));

        loop {
            ticker.tick().await;

            info!("Checking for remote changes: {}", watch_config.remote_path);

            match syncer.scan_remote().await {
                Ok(remote_files) => {
                    for (remote_path, size, mtime) in remote_files {
                        let local_path = watch_config.local_path.join(
                            remote_path
                                .strip_prefix(&watch_config.remote_path)
                                .unwrap_or(&remote_path)
                                .trim_start_matches('/'),
                        );

                        // Check if file exists locally
                        if let Ok(metadata) = tokio::fs::metadata(&local_path).await {
                            let local_size = metadata.len();
                            let local_mtime = metadata.modified().ok().and_then(|t| {
                                chrono::DateTime::from_timestamp(
                                    t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64,
                                    0,
                                )
                            });

                            // Only download if remote is newer or different size
                            if let Some(local_mtime) = local_mtime {
                                if mtime <= local_mtime && size == local_size as i64 {
                                    continue;
                                }
                            }
                        }

                        // Download the file
                        match syncer
                            .download_file(&remote_path, &local_path, size, mtime, None)
                            .await
                        {
                            Ok(()) => {
                                let file_name = local_path
                                    .file_name()
                                    .and_then(|n| n.to_str())
                                    .unwrap_or("unknown");
                                println!(
                                    "{} {} [{}]",
                                    "↓".green(),
                                    file_name,
                                    watch_config.local_path.display()
                                );
                            }
                            Err(e) => {
                                error!("Failed to download {}: {}", remote_path, e);
                            }
                        }
                    }
                }
                Err(e) => {
                    error!("Failed to scan remote: {}", e);
                }
            }
        }
    }
}
