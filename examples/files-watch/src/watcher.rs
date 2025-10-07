//! Filesystem watching with notify

use anyhow::{Context, Result};
use notify::{Event, RecommendedWatcher, RecursiveMode, Watcher};
use notify_debouncer_full::{DebounceEventResult, Debouncer, FileIdMap, new_debouncer};
use std::path::{Path, PathBuf};
use std::time::Duration;
use tokio::sync::mpsc;
use tracing::{debug, error, info};

/// File system event
#[derive(Debug, Clone)]
pub enum FileEvent {
    /// File was created
    Created(PathBuf),
    /// File was modified
    Modified(PathBuf),
    /// File was deleted
    Deleted(PathBuf),
}

/// Filesystem watcher
pub struct FileWatcher {
    _debouncer: Debouncer<RecommendedWatcher, FileIdMap>,
    receiver: mpsc::Receiver<FileEvent>,
}

impl FileWatcher {
    /// Create a new file watcher for the given path
    pub fn new(watch_path: &Path) -> Result<Self> {
        let (tx, rx) = mpsc::channel(100);

        let tx_clone = tx.clone();
        let mut debouncer = new_debouncer(
            Duration::from_millis(500),
            None,
            move |result: DebounceEventResult| match result {
                Ok(events) => {
                    for event in events {
                        if let Some(file_event) = process_event(event.event) {
                            if let Err(e) = tx_clone.blocking_send(file_event) {
                                error!("Failed to send file event: {}", e);
                            }
                        }
                    }
                }
                Err(errors) => {
                    for error in errors {
                        error!("Watch error: {:?}", error);
                    }
                }
            },
        )
        .context("Failed to create file watcher")?;

        debouncer
            .watcher()
            .watch(watch_path, RecursiveMode::Recursive)
            .with_context(|| format!("Failed to watch path: {}", watch_path.display()))?;

        info!("Started watching: {}", watch_path.display());

        Ok(Self {
            _debouncer: debouncer,
            receiver: rx,
        })
    }

    /// Receive the next file event
    pub async fn next_event(&mut self) -> Option<FileEvent> {
        self.receiver.recv().await
    }
}

/// Process a notify event into our FileEvent type
fn process_event(event: Event) -> Option<FileEvent> {
    use notify::EventKind;

    debug!("Processing event: {:?}", event);

    match event.kind {
        EventKind::Create(_) => {
            if let Some(path) = event.paths.first() {
                if path.is_file() {
                    return Some(FileEvent::Created(path.clone()));
                }
            }
        }
        EventKind::Modify(_) => {
            if let Some(path) = event.paths.first() {
                if path.is_file() {
                    return Some(FileEvent::Modified(path.clone()));
                }
            }
        }
        EventKind::Remove(_) => {
            if let Some(path) = event.paths.first() {
                return Some(FileEvent::Deleted(path.clone()));
            }
        }
        _ => {}
    }

    None
}
