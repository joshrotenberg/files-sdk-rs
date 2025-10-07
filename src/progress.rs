//! Progress tracking for file operations
//!
//! This module provides traits and types for tracking progress during
//! file uploads and downloads.

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
