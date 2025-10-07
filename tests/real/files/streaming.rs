//! Real API integration tests for streaming file operations

use crate::real::*;
use files_sdk::FileHandler;
use files_sdk::progress::{Progress, ProgressCallback};
use std::io::Write;
use std::sync::{Arc, Mutex};

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_upload_stream_small_file() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_path = "/integration-tests/stream-upload-small.txt";
    let test_content = b"Hello from streaming upload!";

    // Cleanup
    cleanup_file(&client, test_path).await;

    println!("Testing streaming upload with small file");

    // Create a cursor from bytes (implements AsyncRead)
    let cursor = std::io::Cursor::new(test_content.to_vec());

    // Upload using streaming
    let result = handler
        .upload_stream(test_path, cursor, Some(test_content.len() as i64), None)
        .await;

    assert!(
        result.is_ok(),
        "Upload stream should succeed: {:?}",
        result.err()
    );

    // Verify the file was uploaded by downloading it
    let downloaded = handler.download_content(test_path).await.unwrap();
    assert_eq!(downloaded, test_content);

    // Cleanup
    cleanup_file(&client, test_path).await;
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_upload_stream_with_tokio_file() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_path = "/integration-tests/stream-upload-file.txt";
    let local_path = "/tmp/files-sdk-test-upload.txt";

    // Cleanup
    cleanup_file(&client, test_path).await;

    println!("Testing streaming upload with tokio::fs::File");

    // Create a temporary local file
    let test_content = b"This is a test file for streaming upload using tokio::fs::File";
    std::fs::write(local_path, test_content).unwrap();

    // Open file with tokio
    let file = tokio::fs::File::open(local_path).await.unwrap();
    let metadata = file.metadata().await.unwrap();
    let size = metadata.len();

    // Upload using streaming
    let result = handler
        .upload_stream(test_path, file, Some(size as i64), None)
        .await;

    assert!(
        result.is_ok(),
        "Upload stream from file should succeed: {:?}",
        result.err()
    );

    // Verify content
    let downloaded = handler.download_content(test_path).await.unwrap();
    assert_eq!(downloaded, test_content);

    // Cleanup
    cleanup_file(&client, test_path).await;
    let _ = std::fs::remove_file(local_path);
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_download_stream_small_file() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_path = "/integration-tests/stream-download-small.txt";
    let test_content = b"Hello from streaming download!";

    // Cleanup and upload test file
    cleanup_file(&client, test_path).await;
    handler.upload_file(test_path, test_content).await.unwrap();

    println!("Testing streaming download with small file");

    // Download using streaming to a Vec
    let mut buffer = Vec::new();
    let result = handler.download_stream(test_path, &mut buffer, None).await;

    assert!(
        result.is_ok(),
        "Download stream should succeed: {:?}",
        result.err()
    );

    assert_eq!(buffer, test_content);

    // Cleanup
    cleanup_file(&client, test_path).await;
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_download_stream_to_file() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_path = "/integration-tests/stream-download-file.txt";
    let local_path = "/tmp/files-sdk-test-download.txt";
    let test_content = b"This is a test file for streaming download to tokio::fs::File";

    // Cleanup and upload test file
    cleanup_file(&client, test_path).await;
    handler.upload_file(test_path, test_content).await.unwrap();

    println!("Testing streaming download to tokio::fs::File");

    // Download using streaming to a file
    let mut file = tokio::fs::File::create(local_path).await.unwrap();
    let result = handler.download_stream(test_path, &mut file, None).await;

    assert!(
        result.is_ok(),
        "Download stream to file should succeed: {:?}",
        result.err()
    );

    // Verify file content
    let downloaded = std::fs::read(local_path).unwrap();
    assert_eq!(downloaded, test_content);

    // Cleanup
    cleanup_file(&client, test_path).await;
    let _ = std::fs::remove_file(local_path);
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_upload_stream_larger_file() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_path = "/integration-tests/stream-upload-large.bin";
    let local_path = "/tmp/files-sdk-test-large.bin";

    // Cleanup
    cleanup_file(&client, test_path).await;

    println!("Testing streaming upload with larger file (1MB)");

    // Create a 1MB test file
    let size = 1024 * 1024; // 1MB
    let mut file = std::fs::File::create(local_path).unwrap();
    let chunk = vec![0xAB; 1024]; // 1KB chunk
    for _ in 0..1024 {
        file.write_all(&chunk).unwrap();
    }
    drop(file);

    // Upload using streaming
    let file = tokio::fs::File::open(local_path).await.unwrap();
    let result = handler
        .upload_stream(test_path, file, Some(size), None)
        .await;

    assert!(
        result.is_ok(),
        "Upload large file stream should succeed: {:?}",
        result.err()
    );

    // Verify size (not content, that would take too long)
    let file_info = handler.download_file(test_path).await.unwrap();
    assert_eq!(file_info.size, Some(size));

    // Cleanup
    cleanup_file(&client, test_path).await;
    let _ = std::fs::remove_file(local_path);
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_download_stream_larger_file() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_path = "/integration-tests/stream-download-large.bin";
    let local_upload = "/tmp/files-sdk-test-upload-large.bin";
    let local_download = "/tmp/files-sdk-test-download-large.bin";

    // Cleanup
    cleanup_file(&client, test_path).await;

    println!("Testing streaming download with larger file (1MB)");

    // Create and upload a 1MB test file
    let size = 1024 * 1024; // 1MB
    let mut file = std::fs::File::create(local_upload).unwrap();
    let chunk = vec![0xCD; 1024]; // 1KB chunk
    for _ in 0..1024 {
        file.write_all(&chunk).unwrap();
    }
    drop(file);

    // Upload first
    let upload_file = tokio::fs::File::open(local_upload).await.unwrap();
    handler
        .upload_stream(test_path, upload_file, Some(size), None)
        .await
        .unwrap();

    // Download using streaming
    let mut download_file = tokio::fs::File::create(local_download).await.unwrap();
    let result = handler
        .download_stream(test_path, &mut download_file, None)
        .await;

    assert!(
        result.is_ok(),
        "Download large file stream should succeed: {:?}",
        result.err()
    );

    // Verify size
    let metadata = std::fs::metadata(local_download).unwrap();
    assert_eq!(metadata.len(), size as u64);

    // Cleanup
    cleanup_file(&client, test_path).await;
    let _ = std::fs::remove_file(local_upload);
    let _ = std::fs::remove_file(local_download);
}

// Progress callback implementation for testing
type ProgressUpdate = (u64, Option<u64>);
type ProgressUpdates = Arc<Mutex<Vec<ProgressUpdate>>>;

struct TestProgressCallback {
    updates: ProgressUpdates,
}

impl TestProgressCallback {
    fn new() -> Self {
        Self {
            updates: Arc::new(Mutex::new(Vec::new())),
        }
    }

    fn get_updates(&self) -> Vec<ProgressUpdate> {
        self.updates.lock().unwrap().clone()
    }
}

impl ProgressCallback for TestProgressCallback {
    fn on_progress(&self, progress: &Progress) {
        let mut updates = self.updates.lock().unwrap();
        updates.push((progress.bytes_transferred, progress.total_bytes));
    }
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_upload_stream_with_progress() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_path = "/integration-tests/stream-upload-progress.txt";
    let test_content = vec![0x42; 50000]; // 50KB file

    // Cleanup
    cleanup_file(&client, test_path).await;

    println!("Testing streaming upload with progress callback");

    let callback = Arc::new(TestProgressCallback::new());
    let cursor = std::io::Cursor::new(test_content.clone());

    // Upload with progress tracking
    let result = handler
        .upload_stream(
            test_path,
            cursor,
            Some(test_content.len() as i64),
            Some(callback.clone()),
        )
        .await;

    assert!(
        result.is_ok(),
        "Upload with progress should succeed: {:?}",
        result.err()
    );

    // Verify progress updates were received
    let updates = callback.get_updates();
    assert!(!updates.is_empty(), "Should receive progress updates");

    // Verify final update shows complete transfer
    let last_update = updates.last().unwrap();
    assert_eq!(
        last_update.0,
        test_content.len() as u64,
        "Final progress should match file size"
    );
    assert_eq!(
        last_update.1,
        Some(test_content.len() as u64),
        "Total bytes should be known"
    );

    println!("Received {} progress updates", updates.len());

    // Cleanup
    cleanup_file(&client, test_path).await;
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_download_stream_with_progress() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_path = "/integration-tests/stream-download-progress.txt";
    let test_content = vec![0x43; 50000]; // 50KB file

    // Cleanup and upload test file
    cleanup_file(&client, test_path).await;
    handler.upload_file(test_path, &test_content).await.unwrap();

    println!("Testing streaming download with progress callback");

    let callback = Arc::new(TestProgressCallback::new());
    let mut buffer = Vec::new();

    // Download with progress tracking
    let result = handler
        .download_stream(test_path, &mut buffer, Some(callback.clone()))
        .await;

    assert!(
        result.is_ok(),
        "Download with progress should succeed: {:?}",
        result.err()
    );

    // Verify content
    assert_eq!(buffer, test_content);

    // Verify progress updates were received
    let updates = callback.get_updates();
    assert!(!updates.is_empty(), "Should receive progress updates");

    // Verify final update shows complete transfer
    let last_update = updates.last().unwrap();
    assert_eq!(
        last_update.0,
        test_content.len() as u64,
        "Final progress should match file size"
    );

    println!("Received {} progress updates", updates.len());

    // Cleanup
    cleanup_file(&client, test_path).await;
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_upload_stream_progress_large_file() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_path = "/integration-tests/stream-upload-progress-large.bin";
    let local_path = "/tmp/files-sdk-test-progress-large.bin";

    // Cleanup
    cleanup_file(&client, test_path).await;

    println!("Testing streaming upload with progress on larger file (500KB)");

    // Create a 500KB test file
    let size = 500 * 1024; // 500KB
    let mut file = std::fs::File::create(local_path).unwrap();
    let chunk = vec![0xEE; 1024]; // 1KB chunk
    for _ in 0..(size / 1024) {
        file.write_all(&chunk).unwrap();
    }
    drop(file);

    let callback = Arc::new(TestProgressCallback::new());
    let file = tokio::fs::File::open(local_path).await.unwrap();

    // Upload with progress tracking
    let result = handler
        .upload_stream(test_path, file, Some(size as i64), Some(callback.clone()))
        .await;

    assert!(
        result.is_ok(),
        "Upload large file with progress should succeed: {:?}",
        result.err()
    );

    // Verify progress updates
    let updates = callback.get_updates();
    assert!(
        updates.len() > 1,
        "Should receive multiple progress updates for large file"
    );

    // Verify monotonic increase
    let mut last_bytes = 0;
    for (bytes, _total) in &updates {
        assert!(*bytes >= last_bytes, "Progress should only increase");
        last_bytes = *bytes;
    }

    // Verify final progress
    let last_update = updates.last().unwrap();
    assert_eq!(
        last_update.0, size as u64,
        "Final progress should match file size"
    );

    println!(
        "Received {} progress updates for {}KB upload",
        updates.len(),
        size / 1024
    );

    // Cleanup
    cleanup_file(&client, test_path).await;
    let _ = std::fs::remove_file(local_path);
}
