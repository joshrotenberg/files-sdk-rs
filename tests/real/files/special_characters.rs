//! Integration tests for files with special characters in paths
//!
//! These tests verify that the SDK properly handles file paths containing:
//! - Spaces
//! - Brackets
//! - Unicode characters
//! - Quotes and special symbols

use files_sdk::{FileHandler, FolderHandler};

use crate::real::{cleanup_file, get_test_client};

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_file_with_spaces() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    let path = "/integration-tests/file with spaces.txt";
    let data = b"Testing file with spaces in path";

    // Upload file with spaces in path
    let result = handler.upload_file(path, data).await;
    assert!(
        result.is_ok(),
        "Failed to upload file with spaces: {:?}",
        result.err()
    );

    // Verify we can download it
    let file = handler.download_file(path).await;
    assert!(
        file.is_ok(),
        "Failed to download file with spaces: {:?}",
        file.err()
    );

    // Cleanup
    cleanup_file(&client, path).await;
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_file_with_brackets() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    let path = "/integration-tests/data[2024].txt";
    let data = b"Testing file with brackets";

    // Upload file with brackets
    let result = handler.upload_file(path, data).await;
    assert!(
        result.is_ok(),
        "Failed to upload file with brackets: {:?}",
        result.err()
    );

    // Verify we can download it
    let file = handler.download_file(path).await;
    assert!(
        file.is_ok(),
        "Failed to download file with brackets: {:?}",
        file.err()
    );

    // Cleanup
    cleanup_file(&client, path).await;
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_file_with_unicode() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    // Chinese characters
    let path = "/integration-tests/测试文件.txt";
    let data = b"Testing file with Chinese characters";

    let result = handler.upload_file(path, data).await;
    assert!(
        result.is_ok(),
        "Failed to upload file with unicode: {:?}",
        result.err()
    );

    // Verify we can download it
    let file = handler.download_file(path).await;
    assert!(
        file.is_ok(),
        "Failed to download file with unicode: {:?}",
        file.err()
    );

    // Cleanup
    cleanup_file(&client, path).await;
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_file_with_special_chars() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    let path = "/integration-tests/file@test#data.txt";
    let data = b"Testing file with special characters";

    let result = handler.upload_file(path, data).await;
    assert!(
        result.is_ok(),
        "Failed to upload file with special chars: {:?}",
        result.err()
    );

    // Verify we can download it
    let file = handler.download_file(path).await;
    assert!(
        file.is_ok(),
        "Failed to download file with special chars: {:?}",
        file.err()
    );

    // Cleanup
    cleanup_file(&client, path).await;
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_folder_with_spaces() {
    let client = get_test_client();
    let folder_handler = FolderHandler::new(client.clone());

    let path = "/integration-tests/folder with spaces";

    // Create folder with spaces
    let result = folder_handler.create_folder(path, true).await;
    assert!(
        result.is_ok(),
        "Failed to create folder with spaces: {:?}",
        result.err()
    );

    // List the folder
    let (files, _) = folder_handler.list_folder(path, None, None).await.unwrap();
    assert_eq!(files.len(), 0, "Newly created folder should be empty");

    // Cleanup
    let _ = folder_handler.delete_folder(path, false).await;
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_complex_path() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    // Combination of spaces, brackets, and special chars
    let path = "/integration-tests/my folder/data [2024]/report#1.txt";
    let data = b"Testing complex path with multiple special characters";

    let result = handler.upload_file(path, data).await;
    assert!(
        result.is_ok(),
        "Failed to upload file with complex path: {:?}",
        result.err()
    );

    // Verify we can download it
    let file = handler.download_file(path).await;
    assert!(
        file.is_ok(),
        "Failed to download file with complex path: {:?}",
        file.err()
    );

    // Cleanup
    cleanup_file(&client, path).await;
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_file_copy_with_special_chars() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    let source = "/integration-tests/source file.txt";
    let dest = "/integration-tests/dest [copy].txt";
    let data = b"Testing copy with special chars";

    // Upload source file
    handler.upload_file(source, data).await.unwrap();

    // Copy to destination with special chars
    let result = handler.copy_file(source, dest).await;
    assert!(
        result.is_ok(),
        "Failed to copy file with special chars: {:?}",
        result.err()
    );

    // Verify destination exists
    let file = handler.download_file(dest).await;
    assert!(file.is_ok(), "Copied file with special chars not found");

    // Cleanup
    cleanup_file(&client, source).await;
    cleanup_file(&client, dest).await;
}

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_file_move_with_special_chars() {
    let client = get_test_client();
    let handler = FileHandler::new(client.clone());

    let source = "/integration-tests/move source.txt";
    let dest = "/integration-tests/moved [file].txt";
    let data = b"Testing move with special chars";

    // Upload source file
    handler.upload_file(source, data).await.unwrap();

    // Move to destination with special chars
    let result = handler.move_file(source, dest).await;
    assert!(
        result.is_ok(),
        "Failed to move file with special chars: {:?}",
        result.err()
    );

    // Verify destination exists
    let file = handler.download_file(dest).await;
    assert!(file.is_ok(), "Moved file with special chars not found");

    // Cleanup
    cleanup_file(&client, dest).await;
}
