//! Core sync logic for uploading files

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use files_sdk::FilesClient;
use files_sdk::files::FileHandler;
use files_sdk::progress::ProgressCallback;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tracing::{debug, info, warn};

use crate::config::WatchConfig;
use crate::state::SyncState;

/// File syncer
pub struct Syncer {
    client: FilesClient,
    config: WatchConfig,
    state: SyncState,
}

impl Syncer {
    /// Create a new syncer
    pub fn new(client: FilesClient, config: WatchConfig) -> Result<Self> {
        let state = SyncState::load(&config.local_path)?;

        Ok(Self {
            client,
            config,
            state,
        })
    }

    /// Sync a single file to Files.com
    pub async fn sync_file(
        &mut self,
        file_path: &Path,
        progress: Option<Arc<dyn ProgressCallback>>,
    ) -> Result<()> {
        // Get relative path
        let relative_path = file_path
            .strip_prefix(&self.config.local_path)
            .context("File is not in watched directory")?
            .to_string_lossy()
            .to_string();

        // Check if file should be ignored
        if self.should_ignore(&relative_path) {
            debug!("Ignoring file: {}", relative_path);
            return Ok(());
        }

        // Get file metadata
        let metadata = fs::metadata(file_path)
            .await
            .with_context(|| format!("Failed to read metadata for: {}", file_path.display()))?;

        if !metadata.is_file() {
            debug!("Skipping non-file: {}", file_path.display());
            return Ok(());
        }

        let size = metadata.len();
        let modified = metadata
            .modified()
            .ok()
            .and_then(|t| {
                DateTime::from_timestamp(
                    t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64,
                    0,
                )
            })
            .unwrap_or_else(Utc::now);

        // Check if file needs syncing
        if !self.state.needs_sync(&relative_path, size, modified) {
            debug!("File already synced: {}", relative_path);
            return Ok(());
        }

        // Build remote path
        let remote_path = self.build_remote_path(&relative_path);

        info!("Syncing: {} -> {}", relative_path, remote_path);

        // Open file for streaming
        let file = fs::File::open(file_path)
            .await
            .with_context(|| format!("Failed to open file: {}", file_path.display()))?;

        // Upload using streaming API
        let handler = FileHandler::new(self.client.clone());
        handler
            .upload_stream(&remote_path, file, Some(size as i64), progress)
            .await
            .with_context(|| format!("Failed to upload: {}", remote_path))?;

        // Record sync in state
        self.state.record_sync(
            relative_path.clone(),
            size,
            modified,
            None, // TODO: Add hash calculation
            "up".to_string(),
        );

        self.state.save()?;

        info!("Synced: {}", relative_path);

        Ok(())
    }

    /// Sync all files in the watched directory
    pub async fn sync_all(
        &mut self,
        progress: Option<Arc<dyn ProgressCallback>>,
    ) -> Result<Vec<PathBuf>> {
        let mut synced_files = Vec::new();

        // Walk directory tree
        let mut entries = fs::read_dir(&self.config.local_path)
            .await
            .with_context(|| {
                format!(
                    "Failed to read directory: {}",
                    self.config.local_path.display()
                )
            })?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .context("Failed to read directory entry")?
        {
            let path = entry.path();

            if path.is_dir() {
                // Recursively sync subdirectories
                if let Ok(sub_files) = self.sync_directory(&path, progress.clone()).await {
                    synced_files.extend(sub_files);
                }
            } else if path.is_file() {
                match self.sync_file(&path, progress.clone()).await {
                    Ok(()) => {
                        synced_files.push(path);
                    }
                    Err(e) => {
                        warn!("Failed to sync {}: {}", path.display(), e);
                    }
                }
            }
        }

        Ok(synced_files)
    }

    /// Recursively sync a directory
    fn sync_directory<'a>(
        &'a mut self,
        dir_path: &'a Path,
        progress: Option<Arc<dyn ProgressCallback>>,
    ) -> std::pin::Pin<Box<dyn std::future::Future<Output = Result<Vec<PathBuf>>> + 'a>> {
        Box::pin(async move {
            let mut synced_files = Vec::new();

            let mut entries = fs::read_dir(dir_path)
                .await
                .with_context(|| format!("Failed to read directory: {}", dir_path.display()))?;

            while let Some(entry) = entries
                .next_entry()
                .await
                .context("Failed to read directory entry")?
            {
                let path = entry.path();

                if path.is_dir() {
                    if let Ok(sub_files) = self.sync_directory(&path, progress.clone()).await {
                        synced_files.extend(sub_files);
                    }
                } else if path.is_file() {
                    match self.sync_file(&path, progress.clone()).await {
                        Ok(()) => {
                            synced_files.push(path);
                        }
                        Err(e) => {
                            warn!("Failed to sync {}: {}", path.display(), e);
                        }
                    }
                }
            }

            Ok(synced_files)
        })
    }

    /// Handle a file deletion
    pub async fn handle_delete(&mut self, file_path: &Path) -> Result<()> {
        let relative_path = file_path
            .strip_prefix(&self.config.local_path)
            .context("File is not in watched directory")?
            .to_string_lossy()
            .to_string();

        // Remove from state
        self.state.remove_file(&relative_path);
        self.state.save()?;

        info!("Removed from state: {}", relative_path);

        // TODO: Optionally delete from Files.com
        // For Phase 1, we just track local deletions

        Ok(())
    }

    /// Check if a file should be ignored based on patterns
    fn should_ignore(&self, relative_path: &str) -> bool {
        for pattern in &self.config.ignore_patterns {
            if let Ok(glob_pattern) = glob::Pattern::new(pattern) {
                if glob_pattern.matches(relative_path) {
                    return true;
                }
            }
        }
        false
    }

    /// Build the full remote path
    fn build_remote_path(&self, relative_path: &str) -> String {
        let remote_base = self.config.remote_path.trim_end_matches('/');
        format!("{}/{}", remote_base, relative_path)
    }

    /// Get current state
    #[allow(dead_code)]
    pub fn state(&self) -> &SyncState {
        &self.state
    }
}
