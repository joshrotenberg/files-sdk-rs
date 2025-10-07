//! Progress tracking for file operations
//!
//! This module provides traits and types for tracking progress during
//! file uploads and downloads with the streaming API.
//!
//! # Overview
//!
//! The progress tracking system consists of:
//! - [`Progress`] - Immutable snapshot of current progress
//! - [`ProgressCallback`] - Trait for receiving progress updates
//! - [`PrintProgressCallback`] - Built-in stdout progress logger
//!
//! # Usage
//!
//! Progress callbacks are optional and can be passed to streaming methods
//! like [`FileHandler::upload_stream()`](crate::files::FileHandler::upload_stream)
//! and [`FileHandler::download_stream()`](crate::files::FileHandler::download_stream).
//!
//! ## Basic Example
//!
//! ```rust
//! use files_sdk::progress::{Progress, ProgressCallback};
//! use std::sync::Arc;
//!
//! // Simple progress tracker
//! struct SimpleTracker;
//!
//! impl ProgressCallback for SimpleTracker {
//!     fn on_progress(&self, progress: &Progress) {
//!         if let Some(pct) = progress.percentage() {
//!             println!("Upload: {:.1}%", pct);
//!         }
//!     }
//! }
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! # use files_sdk::{FilesClient, files::FileHandler};
//! # let client = FilesClient::builder().api_key("key").build()?;
//! let handler = FileHandler::new(client);
//! let callback = Arc::new(SimpleTracker);
//!
//! // Use with streaming upload
//! let file = tokio::fs::File::open("large-file.bin").await?;
//! let size = file.metadata().await?.len() as i64;
//! handler.upload_stream("/remote/path.bin", file, Some(size), Some(callback)).await?;
//! # Ok(())
//! # }
//! ```
//!
//! ## Advanced Example with State
//!
//! ```rust
//! use files_sdk::progress::{Progress, ProgressCallback};
//! use std::sync::{Arc, Mutex};
//! use std::time::Instant;
//!
//! // Progress tracker with transfer rate calculation
//! struct RateTracker {
//!     start: Instant,
//!     last_bytes: Arc<Mutex<u64>>,
//! }
//!
//! impl RateTracker {
//!     fn new() -> Self {
//!         Self {
//!             start: Instant::now(),
//!             last_bytes: Arc::new(Mutex::new(0)),
//!         }
//!     }
//! }
//!
//! impl ProgressCallback for RateTracker {
//!     fn on_progress(&self, progress: &Progress) {
//!         let elapsed = self.start.elapsed().as_secs_f64();
//!         let rate = progress.bytes_transferred as f64 / elapsed / 1024.0 / 1024.0;
//!
//!         if let Some(pct) = progress.percentage() {
//!             println!("Progress: {:.1}% @ {:.2} MB/s", pct, rate);
//!         }
//!     }
//! }
//! ```

/// Progress information for a file operation
#[derive(Debug, Clone)]
pub struct Progress {
    /// Bytes transferred so far
    pub bytes_transferred: u64,
    /// Total bytes to transfer (if known)
    pub total_bytes: Option<u64>,
}

impl Progress {
    /// Create a new Progress instance
    pub fn new(bytes_transferred: u64, total_bytes: Option<u64>) -> Self {
        Self {
            bytes_transferred,
            total_bytes,
        }
    }

    /// Calculate progress percentage (0-100)
    ///
    /// Returns None if total_bytes is unknown
    pub fn percentage(&self) -> Option<f64> {
        self.total_bytes.map(|total| {
            if total == 0 {
                100.0
            } else {
                (self.bytes_transferred as f64 / total as f64) * 100.0
            }
        })
    }
}

/// Trait for receiving progress updates during file operations
///
/// Implement this trait to receive callbacks as data is transferred.
///
/// # Examples
///
/// ```rust
/// use files_sdk::progress::{Progress, ProgressCallback};
///
/// struct MyProgressTracker;
///
/// impl ProgressCallback for MyProgressTracker {
///     fn on_progress(&self, progress: &Progress) {
///         if let Some(pct) = progress.percentage() {
///             println!("Progress: {:.1}%", pct);
///         } else {
///             println!("Transferred: {} bytes", progress.bytes_transferred);
///         }
///     }
/// }
/// ```
pub trait ProgressCallback: Send + Sync {
    /// Called when progress is made during a file operation
    ///
    /// # Arguments
    ///
    /// * `progress` - Current progress information
    fn on_progress(&self, progress: &Progress);
}

/// A simple progress callback that prints to stdout
#[derive(Debug)]
pub struct PrintProgressCallback;

impl ProgressCallback for PrintProgressCallback {
    fn on_progress(&self, progress: &Progress) {
        if let Some(pct) = progress.percentage() {
            println!(
                "Progress: {:.1}% ({} / {} bytes)",
                pct,
                progress.bytes_transferred,
                progress.total_bytes.unwrap_or(0)
            );
        } else {
            println!("Transferred: {} bytes", progress.bytes_transferred);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_progress_percentage() {
        let progress = Progress::new(50, Some(100));
        assert_eq!(progress.percentage(), Some(50.0));

        let progress = Progress::new(0, Some(100));
        assert_eq!(progress.percentage(), Some(0.0));

        let progress = Progress::new(100, Some(100));
        assert_eq!(progress.percentage(), Some(100.0));

        let progress = Progress::new(50, None);
        assert_eq!(progress.percentage(), None);
    }

    #[test]
    fn test_progress_percentage_zero_total() {
        let progress = Progress::new(0, Some(0));
        assert_eq!(progress.percentage(), Some(100.0));
    }

    #[test]
    fn test_print_progress_callback() {
        let callback = PrintProgressCallback;
        let progress = Progress::new(50, Some(100));
        callback.on_progress(&progress);

        let progress = Progress::new(50, None);
        callback.on_progress(&progress);
    }
}
