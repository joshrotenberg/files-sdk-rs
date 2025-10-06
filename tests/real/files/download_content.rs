//! Integration tests for downloading actual file content

use files_sdk::FileHandler;
use std::path::Path;

use crate::real::{cleanup_file, get_test_client};

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_download_content() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    let path = "/integration-tests/download-test.txt";
    let expected_content = b"This is test content for download";

    // Upload a test file
    handler.upload_file(path, expected_content).await.unwrap();

    // Download the actual content
    let content = handler.download_content(path).await.unwrap();

    // Verify content matches
    assert_eq!(content, expected_content.to_vec());

    // Cleanup
    cleanup_file(&client, path).await;
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_download_content_large_file() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    let path = "/integration-tests/large-download-test.bin";
    // Create 100KB of test data
    let expected_content: Vec<u8> = (0..100_000).map(|i| (i % 256) as u8).collect();

    // Upload
    handler.upload_file(path, &expected_content).await.unwrap();

    // Download the actual content
    let content = handler.download_content(path).await.unwrap();

    // Verify size and content
    assert_eq!(content.len(), expected_content.len());
    assert_eq!(content, expected_content);

    // Cleanup
    cleanup_file(&client, path).await;
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_download_to_file() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    let remote_path = "/integration-tests/download-to-file-test.txt";
    let local_path = Path::new("/tmp/files-sdk-test-download.txt");
    let expected_content = b"Content to download to local file";

    // Upload test file
    handler
        .upload_file(remote_path, expected_content)
        .await
        .unwrap();

    // Download to local file
    handler
        .download_to_file(remote_path, local_path)
        .await
        .unwrap();

    // Verify local file exists and has correct content
    let local_content = std::fs::read(local_path).unwrap();
    assert_eq!(local_content, expected_content.to_vec());

    // Cleanup
    cleanup_file(&client, remote_path).await;
    let _ = std::fs::remove_file(local_path);
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_download_content_nonexistent_file() {
    let client = get_test_client();
    let handler = FileHandler::new(client);

    let path = "/integration-tests/nonexistent-file.txt";

    // Should fail with NotFound
    let result = handler.download_content(path).await;
    assert!(result.is_err());

    match result {
        Err(files_sdk::FilesError::NotFound { .. }) => {
            // Expected error
        }
        other => panic!("Expected NotFound error, got: {:?}", other),
    }
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_download_metadata_vs_content() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    let path = "/integration-tests/metadata-vs-content-test.txt";
    let content = b"Test content for metadata vs content comparison";

    // Upload
    handler.upload_file(path, content).await.unwrap();

    // download_file returns metadata (FileEntity)
    let file_entity = handler.download_file(path).await.unwrap();
    assert!(file_entity.download_uri.is_some());
    assert_eq!(file_entity.path, Some(path.to_string()));

    // download_content returns actual bytes
    let actual_content = handler.download_content(path).await.unwrap();
    assert_eq!(actual_content, content.to_vec());

    // Cleanup
    cleanup_file(&client, path).await;
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_download_content_with_special_chars() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    let path = "/integration-tests/download test [file].txt";
    let content = b"Content with special chars in path";

    // Upload
    handler.upload_file(path, content).await.unwrap();

    // Download content
    let downloaded = handler.download_content(path).await.unwrap();
    assert_eq!(downloaded, content.to_vec());

    // Cleanup
    cleanup_file(&client, path).await;
}
