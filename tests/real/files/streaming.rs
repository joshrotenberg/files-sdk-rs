//! Real API integration tests for streaming file operations

use crate::real::*;
use files_sdk::FileHandler;
use std::io::Write;

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
        .upload_stream(test_path, cursor, Some(test_content.len() as i64))
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
        .upload_stream(test_path, file, Some(size as i64))
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
    let result = handler.download_stream(test_path, &mut buffer).await;

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
    let result = handler.download_stream(test_path, &mut file).await;

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
    let result = handler.upload_stream(test_path, file, Some(size)).await;

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
        .upload_stream(test_path, upload_file, Some(size))
        .await
        .unwrap();

    // Download using streaming
    let mut download_file = tokio::fs::File::create(local_download).await.unwrap();
    let result = handler.download_stream(test_path, &mut download_file).await;

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
