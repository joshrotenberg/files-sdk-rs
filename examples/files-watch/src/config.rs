//! Configuration file handling

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};

/// Main configuration structure
#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Config {
    /// List of watch configurations
    #[serde(default)]
    pub watch: Vec<WatchConfig>,

    /// Sync settings
    #[serde(default)]
    pub sync: SyncSettings,

    /// Conflict resolution settings
    #[serde(default)]
    pub conflict: ConflictSettings,
}

/// Individual watch configuration
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WatchConfig {
    /// Local directory path
    pub local_path: PathBuf,

    /// Remote path on Files.com
    pub remote_path: String,

    /// Sync direction: "up", "down", or "both"
    #[serde(default = "default_direction")]
    pub direction: String,

    /// Patterns to ignore (glob patterns)
    #[serde(default)]
    pub ignore_patterns: Vec<String>,
}

/// Sync settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SyncSettings {
    /// How often to check for remote changes (seconds)
    #[serde(default = "default_check_interval")]
    pub check_interval_secs: u64,

    /// Maximum number of concurrent uploads
    #[serde(default = "default_concurrent_uploads")]
    pub concurrent_uploads: usize,

    /// Chunk size for streaming operations
    #[serde(default = "default_chunk_size")]
    pub chunk_size: usize,
}

/// Conflict resolution settings
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ConflictSettings {
    /// Resolution strategy: "newest", "largest", or "manual"
    #[serde(default = "default_resolution")]
    pub resolution: String,
}

impl Default for SyncSettings {
    fn default() -> Self {
        Self {
            check_interval_secs: default_check_interval(),
            concurrent_uploads: default_concurrent_uploads(),
            chunk_size: default_chunk_size(),
        }
    }
}

impl Default for ConflictSettings {
    fn default() -> Self {
        Self {
            resolution: default_resolution(),
        }
    }
}

fn default_direction() -> String {
    "up".to_string()
}

fn default_check_interval() -> u64 {
    60
}

fn default_concurrent_uploads() -> usize {
    5
}

fn default_chunk_size() -> usize {
    65536
}

fn default_resolution() -> String {
    "newest".to_string()
}

impl Config {
    /// Get the default config directory
    pub fn config_dir() -> Result<PathBuf> {
        let home = dirs::home_dir().context("Could not determine home directory")?;
        Ok(home.join(".files-watch"))
    }

    /// Get the default config file path
    pub fn config_path() -> Result<PathBuf> {
        Ok(Self::config_dir()?.join("config.toml"))
    }

    /// Load configuration from file
    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let contents = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Config = toml::from_str(&contents)
            .with_context(|| format!("Failed to parse config file: {}", path.display()))?;

        Ok(config)
    }

    /// Save configuration to file
    pub fn save(&self) -> Result<()> {
        let config_dir = Self::config_dir()?;
        fs::create_dir_all(&config_dir).with_context(|| {
            format!(
                "Failed to create config directory: {}",
                config_dir.display()
            )
        })?;

        let path = Self::config_path()?;
        let contents = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&path, contents)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }

    /// Add a new watch configuration
    pub fn add_watch(&mut self, watch: WatchConfig) -> Result<()> {
        // Check if this path is already configured
        if self.watch.iter().any(|w| w.local_path == watch.local_path) {
            anyhow::bail!("Path {} is already configured", watch.local_path.display());
        }

        self.watch.push(watch);
        self.save()?;
        Ok(())
    }

    /// Remove a watch configuration
    #[allow(dead_code)]
    pub fn remove_watch(&mut self, local_path: &Path) -> Result<()> {
        let original_len = self.watch.len();
        self.watch.retain(|w| w.local_path != local_path);

        if self.watch.len() == original_len {
            anyhow::bail!("No configuration found for path: {}", local_path.display());
        }

        self.save()?;
        Ok(())
    }

    /// Find a watch configuration by local path
    pub fn find_watch(&self, local_path: &Path) -> Option<&WatchConfig> {
        self.watch.iter().find(|w| w.local_path == local_path)
    }
}
