//! Progress tracking UI with indicatif

use files_sdk::progress::{Progress, ProgressCallback};
use indicatif::{ProgressBar, ProgressStyle};
use std::sync::Arc;

/// Progress tracker that updates an indicatif progress bar
pub struct ProgressBarTracker {
    bar: Arc<ProgressBar>,
}

impl ProgressBarTracker {
    /// Create a new progress tracker with a progress bar
    pub fn new(total_bytes: Option<u64>) -> Self {
        let bar = if let Some(total) = total_bytes {
            ProgressBar::new(total)
        } else {
            ProgressBar::new_spinner()
        };

        bar.set_style(
            ProgressStyle::default_bar()
                .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
                .unwrap()
                .progress_chars("#>-"),
        );

        Self { bar: Arc::new(bar) }
    }

    /// Finish the progress bar
    pub fn finish(&self) {
        self.bar.finish_with_message("done");
    }
}

impl ProgressCallback for ProgressBarTracker {
    fn on_progress(&self, progress: &Progress) {
        self.bar.set_position(progress.bytes_transferred);
        if let Some(total) = progress.total_bytes {
            self.bar.set_length(total);
        }
    }
}
