//! Integration tests for files-watch

use std::fs;
use std::path::PathBuf;
use tempfile::TempDir;

/// Test helper to create a temporary directory with test files
fn setup_test_dir() -> (TempDir, Vec<PathBuf>) {
    let temp_dir = TempDir::new().unwrap();
    let mut files = Vec::new();

    // Create some test files
    let file1 = temp_dir.path().join("test1.txt");
    fs::write(&file1, "Hello, world!").unwrap();
    files.push(file1);

    let file2 = temp_dir.path().join("test2.txt");
    fs::write(&file2, "Another test file").unwrap();
    files.push(file2);

    // Create a subdirectory with a file
    let subdir = temp_dir.path().join("subdir");
    fs::create_dir(&subdir).unwrap();
    let file3 = subdir.join("nested.txt");
    fs::write(&file3, "Nested file").unwrap();
    files.push(file3);

    (temp_dir, files)
}

#[test]
fn test_config_creation() {
    use files_watch::config::{Config, WatchConfig};

    let temp_dir = TempDir::new().unwrap();

    let watch = WatchConfig {
        local_path: temp_dir.path().to_path_buf(),
        remote_path: "/test/remote".to_string(),
        direction: "up".to_string(),
        ignore_patterns: vec!["*.tmp".to_string()],
    };

    let mut config = Config::default();
    config.watch.push(watch.clone());

    assert_eq!(config.watch.len(), 1);
    assert_eq!(config.watch[0].direction, "up");
    assert_eq!(config.watch[0].remote_path, "/test/remote");
}

#[test]
fn test_sync_state_management() {
    use chrono::Utc;
    use files_watch::state::SyncState;

    let temp_dir = TempDir::new().unwrap();
    let state = SyncState::new(temp_dir.path().to_path_buf());

    assert_eq!(state.files.len(), 0);
    assert!(state.last_sync.is_none());

    // Test needs_sync for new file
    let now = Utc::now();
    assert!(state.needs_sync("test.txt", 100, now, None));
}

#[test]
fn test_ignore_patterns() {
    use files_watch::ignore::IgnoreMatcher;

    let matcher = IgnoreMatcher::new(vec![
        "*.tmp".to_string(),
        "*.log".to_string(),
        "node_modules/".to_string(),
    ]);

    assert!(matcher.is_ignored("file.tmp"));
    assert!(matcher.is_ignored("debug.log"));
    assert!(matcher.is_ignored("node_modules"));
    assert!(matcher.is_ignored("node_modules/package.json"));
    assert!(!matcher.is_ignored("file.txt"));
}

#[test]
fn test_ignore_from_file() {
    use files_watch::ignore::IgnoreMatcher;

    let temp_dir = TempDir::new().unwrap();
    let ignore_file = temp_dir.path().join(".filesignore");

    fs::write(
        &ignore_file,
        "# Comment\n*.tmp\n*.log\n\nnode_modules/\ntarget/\n",
    )
    .unwrap();

    let matcher = IgnoreMatcher::from_file(temp_dir.path()).unwrap();

    assert!(matcher.is_ignored("file.tmp"));
    assert!(matcher.is_ignored("app.log"));
    assert!(matcher.is_ignored("node_modules"));
    assert!(matcher.is_ignored("target"));
    assert!(!matcher.is_ignored("src/main.rs"));
}

#[test]
fn test_conflict_resolution_newest() {
    use chrono::{Duration, Utc};
    use files_watch::conflict::{ConflictResolution, ConflictWinner, FileConflict};

    let now = Utc::now();
    let earlier = now - Duration::seconds(60);

    let conflict = FileConflict {
        path: "test.txt".to_string(),
        local_size: 100,
        local_mtime: earlier,
        remote_size: 100,
        remote_mtime: now,
    };

    let winner = conflict.resolve(ConflictResolution::Newest);
    assert_eq!(winner, ConflictWinner::Remote);
}

#[test]
fn test_conflict_resolution_largest() {
    use chrono::Utc;
    use files_watch::conflict::{ConflictResolution, ConflictWinner, FileConflict};

    let now = Utc::now();

    let conflict = FileConflict {
        path: "test.txt".to_string(),
        local_size: 200,
        local_mtime: now,
        remote_size: 100,
        remote_mtime: now,
    };

    let winner = conflict.resolve(ConflictResolution::Largest);
    assert_eq!(winner, ConflictWinner::Local);
}

#[test]
fn test_conflict_resolution_manual() {
    use chrono::Utc;
    use files_watch::conflict::{ConflictResolution, ConflictWinner, FileConflict};

    let now = Utc::now();

    let conflict = FileConflict {
        path: "test.txt".to_string(),
        local_size: 100,
        local_mtime: now,
        remote_size: 100,
        remote_mtime: now,
    };

    let winner = conflict.resolve(ConflictResolution::Manual);
    assert_eq!(winner, ConflictWinner::Skip);
}

#[test]
fn test_progress_tracker() {
    use files_sdk::progress::{Progress, ProgressCallback};
    use files_watch::progress::ProgressBarTracker;

    let tracker = ProgressBarTracker::new(Some(1000));
    tracker.on_progress(&Progress {
        bytes_transferred: 500,
        total_bytes: Some(1000),
    });
    tracker.on_progress(&Progress {
        bytes_transferred: 1000,
        total_bytes: Some(1000),
    });
    tracker.finish();
}

#[test]
fn test_file_watcher_creation() {
    use files_watch::watcher::FileWatcher;

    let temp_dir = TempDir::new().unwrap();
    let watcher = FileWatcher::new(temp_dir.path());
    assert!(watcher.is_ok());
}

#[test]
fn test_config_serialization() {
    use files_watch::config::{Config, WatchConfig};

    let temp_dir = TempDir::new().unwrap();

    let watch = WatchConfig {
        local_path: temp_dir.path().to_path_buf(),
        remote_path: "/test/remote".to_string(),
        direction: "both".to_string(),
        ignore_patterns: vec!["*.tmp".to_string(), ".git/".to_string()],
    };

    let mut config = Config::default();
    config.watch.push(watch);

    // Serialize to TOML
    let toml_str = toml::to_string_pretty(&config).unwrap();

    // Deserialize back
    let parsed: Config = toml::from_str(&toml_str).unwrap();

    assert_eq!(parsed.watch.len(), 1);
    assert_eq!(parsed.watch[0].direction, "both");
    assert_eq!(parsed.watch[0].ignore_patterns.len(), 2);
}

#[test]
fn test_state_serialization() {
    use chrono::Utc;
    use files_watch::state::SyncState;

    let temp_dir = TempDir::new().unwrap();
    let mut state = SyncState::new(temp_dir.path().to_path_buf());

    state.record_sync(
        "test.txt".to_string(),
        100,
        Utc::now(),
        Some("abc123".to_string()),
        "up".to_string(),
    );

    // Serialize to JSON
    let json_str = serde_json::to_string_pretty(&state).unwrap();

    // Deserialize back
    let parsed: SyncState = serde_json::from_str(&json_str).unwrap();

    assert_eq!(parsed.files.len(), 1);
    assert!(parsed.files.contains_key("test.txt"));
}

#[test]
fn test_multiple_ignore_patterns() {
    use files_watch::ignore::IgnoreMatcher;

    let matcher = IgnoreMatcher::new(vec![
        "*.tmp".to_string(),
        "*.log".to_string(),
        ".git/".to_string(),
        "node_modules/".to_string(),
        "target/".to_string(),
        "*.swp".to_string(),
    ]);

    // Test all patterns
    assert!(matcher.is_ignored("file.tmp"));
    assert!(matcher.is_ignored("app.log"));
    assert!(matcher.is_ignored(".git/config"));
    assert!(matcher.is_ignored("node_modules/express"));
    assert!(matcher.is_ignored("target/debug"));
    assert!(matcher.is_ignored(".main.rs.swp"));

    // Test non-matching files
    assert!(!matcher.is_ignored("src/main.rs"));
    assert!(!matcher.is_ignored("README.md"));
    assert!(!matcher.is_ignored("Cargo.toml"));
}

#[test]
fn test_directory_recursion() {
    let (temp_dir, _files) = setup_test_dir();

    // Count files recursively
    let mut file_count = 0;
    for entry in walkdir::WalkDir::new(temp_dir.path()).into_iter().flatten() {
        if entry.file_type().is_file() {
            file_count += 1;
        }
    }

    assert_eq!(file_count, 3); // test1.txt, test2.txt, nested.txt
}

#[test]
fn test_empty_config() {
    use files_watch::config::Config;

    let config = Config::default();
    assert_eq!(config.watch.len(), 0);
    assert_eq!(config.sync.check_interval_secs, 60);
    assert_eq!(config.sync.concurrent_uploads, 5);
    assert_eq!(config.conflict.resolution, "newest");
}

#[test]
fn test_hash_based_sync_decision() {
    use chrono::Utc;
    use files_watch::state::SyncState;

    let temp_dir = TempDir::new().unwrap();
    let mut state = SyncState::new(temp_dir.path().to_path_buf());

    let now = Utc::now();
    let hash = "abc123";

    // Record initial sync
    state.record_sync(
        "test.txt".to_string(),
        100,
        now,
        Some(hash.to_string()),
        "up".to_string(),
    );

    // Same hash - should not need sync
    assert!(!state.needs_sync("test.txt", 100, now, Some(hash)));

    // Different hash - should need sync
    assert!(state.needs_sync("test.txt", 100, now, Some("def456")));

    // No hash provided - fall back to size/mtime
    assert!(!state.needs_sync("test.txt", 100, now, None));
}
