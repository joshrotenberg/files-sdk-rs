//! State persistence for tracking synced files

use anyhow::{Context, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

/// Sync state for a watch configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncState {
    /// Local path this state is for
    pub local_path: PathBuf,

    /// Map of relative file paths to their sync info
    #[serde(default)]
    pub files: HashMap<String, FileSyncInfo>,

    /// Last successful sync time
    pub last_sync: Option<DateTime<Utc>>,
}

/// Information about a synced file
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileSyncInfo {
    /// File size in bytes
    pub size: u64,

    /// Last modified time
    pub modified: DateTime<Utc>,

    /// SHA256 hash of the file content
    pub hash: Option<String>,

    /// Last sync time for this file
    pub synced_at: DateTime<Utc>,

    /// Sync direction: "up", "down", or "both"
    pub direction: String,
}

impl SyncState {
    /// Create a new sync state for a local path
    pub fn new(local_path: PathBuf) -> Self {
        Self {
            local_path,
            files: HashMap::new(),
            last_sync: None,
        }
    }

    /// Get the state file path for a given local path
    fn state_path(local_path: &Path) -> Result<PathBuf> {
        let config_dir = crate::config::Config::config_dir()?;
        let state_dir = config_dir.join("state");
        fs::create_dir_all(&state_dir).with_context(|| {
            format!("Failed to create state directory: {}", state_dir.display())
        })?;

        // Create a safe filename from the local path
        let path_str = local_path.to_string_lossy();
        let safe_name = path_str.replace(['/', '\\'], "_");
        Ok(state_dir.join(format!("{}.json", safe_name)))
    }

    /// Load state from disk
    pub fn load(local_path: &Path) -> Result<Self> {
        let path = Self::state_path(local_path)?;

        if !path.exists() {
            return Ok(Self::new(local_path.to_path_buf()));
        }

        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read state file: {}", path.display()))?;

        let state: SyncState = serde_json::from_str(&contents)
            .with_context(|| format!("Failed to parse state file: {}", path.display()))?;

        Ok(state)
    }

    /// Save state to disk
    pub fn save(&self) -> Result<()> {
        let path = Self::state_path(&self.local_path)?;
        let contents = serde_json::to_string_pretty(self).context("Failed to serialize state")?;

        fs::write(&path, contents)
            .with_context(|| format!("Failed to write state file: {}", path.display()))?;

        Ok(())
    }

    /// Check if a file needs to be synced based on state
    pub fn needs_sync(&self, relative_path: &str, size: u64, modified: DateTime<Utc>) -> bool {
        match self.files.get(relative_path) {
            None => true, // Never synced
            Some(info) => {
                // Sync if size or modified time changed
                info.size != size || info.modified != modified
            }
        }
    }

    /// Record that a file was synced
    pub fn record_sync(
        &mut self,
        relative_path: String,
        size: u64,
        modified: DateTime<Utc>,
        hash: Option<String>,
        direction: String,
    ) {
        let info = FileSyncInfo {
            size,
            modified,
            hash,
            synced_at: Utc::now(),
            direction,
        };

        self.files.insert(relative_path, info);
        self.last_sync = Some(Utc::now());
    }

    /// Remove a file from state (when deleted)
    pub fn remove_file(&mut self, relative_path: &str) {
        self.files.remove(relative_path);
    }

    /// Clear all state
    #[allow(dead_code)]
    pub fn clear(&mut self) {
        self.files.clear();
        self.last_sync = None;
    }

    /// Get count of synced files
    pub fn file_count(&self) -> usize {
        self.files.len()
    }
}
