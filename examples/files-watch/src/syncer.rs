//! Core sync logic for uploading and downloading files

use anyhow::{Context, Result};
use async_recursion::async_recursion;
use chrono::{DateTime, Utc};
use files_sdk::files::{FileHandler, FolderHandler};
use files_sdk::progress::ProgressCallback;
use files_sdk::FilesClient;
use sha2::{Digest, Sha256};
use std::path::{Path, PathBuf};
use std::sync::Arc;
use tokio::fs;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tracing::{debug, info, warn};

use crate::config::WatchConfig;
use crate::conflict::{ConflictResolution, ConflictWinner, FileConflict};
use crate::ignore::IgnoreMatcher;
use crate::state::SyncState;

/// File syncer
#[derive(Clone)]
pub struct Syncer {
    client: FilesClient,
    config: WatchConfig,
    state: SyncState,
    ignore_matcher: IgnoreMatcher,
}

impl Syncer {
    /// Create a new syncer
    pub fn new(client: FilesClient, config: WatchConfig) -> Result<Self> {
        let state = SyncState::load(&config.local_path)?;

        // Load ignore patterns from .filesignore and config
        let mut ignore_matcher = IgnoreMatcher::from_file(&config.local_path)?;

        // Add patterns from config
        for pattern in &config.ignore_patterns {
            ignore_matcher.add_pattern(pattern.clone());
        }

        Ok(Self {
            client,
            config,
            state,
            ignore_matcher,
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

        // Calculate file hash for incremental sync
        let hash = Self::calculate_file_hash(file_path).await?;

        // Check if file needs syncing (with hash comparison)
        if !self
            .state
            .needs_sync(&relative_path, size, modified, Some(&hash))
        {
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

        // Record sync in state (with hash)
        self.state.record_sync(
            relative_path.clone(),
            size,
            modified,
            Some(hash),
            "up".to_string(),
        );

        self.state.save()?;

        info!("Synced: {}", relative_path);

        Ok(())
    }

    /// Download a single file from Files.com
    pub async fn download_file(
        &mut self,
        remote_path: &str,
        local_path: &Path,
        size: i64,
        mtime: DateTime<Utc>,
        progress: Option<Arc<dyn ProgressCallback>>,
    ) -> Result<()> {
        info!("Downloading: {} -> {}", remote_path, local_path.display());

        // Create parent directories if needed
        if let Some(parent) = local_path.parent() {
            fs::create_dir_all(parent)
                .await
                .with_context(|| format!("Failed to create directory: {}", parent.display()))?;
        }

        // Download using streaming API
        let handler = FileHandler::new(self.client.clone());
        let mut file = fs::File::create(local_path)
            .await
            .with_context(|| format!("Failed to create local file: {}", local_path.display()))?;

        handler
            .download_stream(remote_path, &mut file, progress)
            .await
            .with_context(|| format!("Failed to download: {}", remote_path))?;

        file.flush()
            .await
            .context("Failed to flush downloaded file")?;

        // Update file modification time to match remote
        let system_time = std::time::SystemTime::from(mtime);
        let metadata = file.metadata().await?;
        let permissions = metadata.permissions();
        drop(file); // Close file before setting times

        filetime::set_file_mtime(
            local_path,
            filetime::FileTime::from_system_time(system_time),
        )
        .ok(); // Ignore errors - not critical

        // Restore permissions
        fs::set_permissions(local_path, permissions).await.ok();

        // Get relative path for state tracking
        let relative_path = local_path
            .strip_prefix(&self.config.local_path)
            .unwrap_or(local_path)
            .to_string_lossy()
            .to_string();

        // Record sync in state
        self.state.record_sync(
            relative_path.clone(),
            size as u64,
            mtime,
            None,
            "down".to_string(),
        );

        self.state.save()?;

        info!("Downloaded: {}", relative_path);

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
    #[async_recursion]
    async fn sync_directory(
        &mut self,
        dir_path: &Path,
        progress: Option<Arc<dyn ProgressCallback>>,
    ) -> Result<Vec<PathBuf>> {
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
        self.ignore_matcher.is_ignored(relative_path)
    }

    /// Calculate SHA256 hash of a file
    async fn calculate_file_hash(file_path: &Path) -> Result<String> {
        let mut file = fs::File::open(file_path).await?;
        let mut hasher = Sha256::new();
        let mut buffer = vec![0u8; 8192];

        loop {
            let bytes_read = file.read(&mut buffer).await?;
            if bytes_read == 0 {
                break;
            }
            hasher.update(&buffer[..bytes_read]);
        }

        Ok(format!("{:x}", hasher.finalize()))
    }

    /// Build the full remote path
    fn build_remote_path(&self, relative_path: &str) -> String {
        let remote_base = self.config.remote_path.trim_end_matches('/');
        format!("{}/{}", remote_base, relative_path)
    }

    /// Scan remote directory for changes
    pub async fn scan_remote(&mut self) -> Result<Vec<(String, i64, DateTime<Utc>)>> {
        let folder_handler = FolderHandler::new(self.client.clone());

        let mut remote_files = Vec::new();
        let mut cursor = None;

        // List all files in remote directory (with pagination)
        loop {
            let (files, pagination) = folder_handler
                .list_folder(&self.config.remote_path, None, cursor.clone())
                .await
                .with_context(|| {
                    format!(
                        "Failed to list remote directory: {}",
                        self.config.remote_path
                    )
                })?;

            for file in files {
                // Only process actual files, not directories
                if file.file_type.as_deref() == Some("file") {
                    if let Some(path) = &file.path {
                        let size = file.size.unwrap_or(0);

                        // Parse mtime
                        let mtime = file
                            .mtime
                            .as_ref()
                            .and_then(|s| DateTime::parse_from_rfc3339(s).ok())
                            .map(|dt| dt.with_timezone(&Utc))
                            .unwrap_or_else(Utc::now);

                        remote_files.push((path.clone(), size, mtime));
                    }
                }
            }

            // Check for next page
            if pagination.cursor_next.is_none() {
                break;
            }
            cursor = pagination.cursor_next;
        }

        Ok(remote_files)
    }

    /// Perform bidirectional sync
    pub async fn sync_bidirectional(
        &mut self,
        progress: Option<Arc<dyn ProgressCallback>>,
        conflict_resolution: ConflictResolution,
    ) -> Result<()> {
        info!("Starting bidirectional sync");

        // Scan remote files
        let remote_files = self.scan_remote().await?;

        // Build a map of remote files by relative path
        let remote_base = self.config.remote_path.trim_end_matches('/');
        let mut remote_map = std::collections::HashMap::new();

        for (remote_path, size, mtime) in remote_files {
            if let Some(relative) = remote_path.strip_prefix(remote_base) {
                let relative = relative.trim_start_matches('/');
                remote_map.insert(relative.to_string(), (size, mtime));
            }
        }

        // Walk local directory
        let mut local_files = Vec::new();
        self.collect_local_files(&self.config.local_path.clone(), &mut local_files)
            .await?;

        // Process each local file
        for local_path in &local_files {
            let relative_path = local_path
                .strip_prefix(&self.config.local_path)
                .context("File not in watched directory")?
                .to_string_lossy()
                .to_string();

            if self.should_ignore(&relative_path) {
                continue;
            }

            let metadata = fs::metadata(local_path).await?;
            let local_size = metadata.len();
            let local_mtime = metadata
                .modified()
                .ok()
                .and_then(|t| {
                    DateTime::from_timestamp(
                        t.duration_since(std::time::UNIX_EPOCH).ok()?.as_secs() as i64,
                        0,
                    )
                })
                .unwrap_or_else(Utc::now);

            if let Some((remote_size, remote_mtime)) = remote_map.get(&relative_path) {
                // File exists on both sides - check for conflict
                if local_mtime != *remote_mtime || local_size != *remote_size as u64 {
                    let conflict = FileConflict {
                        path: relative_path.clone(),
                        local_size,
                        local_mtime,
                        remote_size: *remote_size,
                        remote_mtime: *remote_mtime,
                    };

                    match conflict.resolve(conflict_resolution) {
                        ConflictWinner::Local => {
                            // Upload local version
                            info!(
                                "Conflict resolved: uploading local version of {}",
                                relative_path
                            );
                            self.sync_file(local_path, progress.clone()).await?;
                        }
                        ConflictWinner::Remote => {
                            // Download remote version
                            info!(
                                "Conflict resolved: downloading remote version of {}",
                                relative_path
                            );
                            let remote_path = self.build_remote_path(&relative_path);
                            self.download_file(
                                &remote_path,
                                local_path,
                                *remote_size,
                                *remote_mtime,
                                progress.clone(),
                            )
                            .await?;
                        }
                        ConflictWinner::Skip => {
                            warn!("Conflict detected, skipping: {}", relative_path);
                        }
                    }
                }
                // Remove from map - we've processed it
                remote_map.remove(&relative_path);
            } else {
                // File only exists locally - upload it
                self.sync_file(local_path, progress.clone()).await?;
            }
        }

        // Download files that only exist remotely
        for (relative_path, (remote_size, remote_mtime)) in remote_map {
            if self.should_ignore(&relative_path) {
                continue;
            }

            let local_path = self.config.local_path.join(&relative_path);
            let remote_path = self.build_remote_path(&relative_path);

            self.download_file(
                &remote_path,
                &local_path,
                remote_size,
                remote_mtime,
                progress.clone(),
            )
            .await?;
        }

        info!("Bidirectional sync complete");
        Ok(())
    }

    /// Collect all local files recursively
    #[async_recursion]
    #[allow(clippy::only_used_in_recursion)]
    async fn collect_local_files(&self, dir: &Path, files: &mut Vec<PathBuf>) -> Result<()> {
        let mut entries = fs::read_dir(dir)
            .await
            .with_context(|| format!("Failed to read directory: {}", dir.display()))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .context("Failed to read directory entry")?
        {
            let path = entry.path();

            if path.is_dir() {
                self.collect_local_files(&path, files).await?;
            } else if path.is_file() {
                files.push(path);
            }
        }

        Ok(())
    }

    /// Get current state
    #[allow(dead_code)]
    pub fn state(&self) -> &SyncState {
        &self.state
    }
}
