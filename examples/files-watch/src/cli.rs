//! CLI command definitions

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "files-watch")]
#[command(about = "Filesystem sync daemon for Files.com", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Initialize a new sync configuration
    Init {
        /// Local directory to watch
        local_path: PathBuf,

        /// Remote path on Files.com
        #[arg(short, long)]
        remote: String,

        /// Sync direction: up, down, or both
        #[arg(short, long, default_value = "up")]
        direction: String,

        /// Patterns to ignore (can be specified multiple times)
        #[arg(short, long)]
        ignore: Vec<String>,
    },

    /// Start syncing (foreground by default)
    Start {
        /// Run as background daemon
        #[arg(short, long)]
        daemon: bool,

        /// Configuration to start (local path)
        path: Option<PathBuf>,
    },

    /// Check sync status
    Status {
        /// Configuration to check (local path, or all if not specified)
        path: Option<PathBuf>,
    },

    /// Pause syncing
    Pause {
        /// Configuration to pause (local path, or all if not specified)
        path: Option<PathBuf>,
    },

    /// Resume syncing
    Resume {
        /// Configuration to resume (local path, or all if not specified)
        path: Option<PathBuf>,
    },

    /// Perform one-time sync without watching
    Sync {
        /// Configuration to sync (local path)
        path: PathBuf,

        /// Sync direction: up, down, or both
        #[arg(short, long)]
        direction: Option<String>,

        /// Force full sync (ignore state)
        #[arg(short, long)]
        full: bool,
    },

    /// List all configured syncs
    List,

    /// Remove a sync configuration
    Remove {
        /// Configuration to remove (local path)
        path: PathBuf,
    },

    /// Show sync history
    History {
        /// Configuration to show history for (local path, or all if not specified)
        path: Option<PathBuf>,

        /// Number of entries to show
        #[arg(short, long, default_value = "10")]
        limit: usize,
    },
}
