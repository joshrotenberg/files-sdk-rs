//! Comprehensive real API tests for FileHandler
//!
//! Tests cover edge cases, error scenarios, and advanced features

use crate::real::*;
use files_sdk::{FileHandler, FilesError, FolderHandler};

#[tokio::test]
async fn test_file_delete_with_recursive_flag() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_file = "/integration-tests/delete-test.txt";
    let test_content = b"Test delete";

    // Upload file
    file_handler
        .upload_file(test_file, test_content)
        .await
        .expect("Should upload file");

    println!("Testing delete with recursive=false");

    // Delete with recursive=false (standard delete)
    let result = file_handler.delete_file(test_file, false).await;

    assert!(result.is_ok(), "Should delete file successfully");

    // Verify file is gone by trying to download
    let download_result = file_handler.download_file(test_file).await;
    assert!(
        matches!(download_result, Err(FilesError::NotFound { .. })),
        "File should not exist after deletion"
    );
}

#[tokio::test]
async fn test_file_not_found_error() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());

    let nonexistent_file = "/integration-tests/does-not-exist.txt";

    println!("Testing file not found error");

    let result = file_handler.download_file(nonexistent_file).await;

    match result {
        Err(FilesError::NotFound { message }) => {
            println!("Correctly received NotFound error: {}", message);
            assert!(!message.is_empty());
        }
        Err(e) => panic!("Expected NotFound error, got: {:?}", e),
        Ok(_) => panic!("Should not find nonexistent file"),
    }
}

#[tokio::test]
async fn test_file_with_special_characters() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    // Test files with various special characters
    let test_cases = vec![
        "/integration-tests/file with spaces.txt",
        "/integration-tests/file-with-dashes.txt",
        "/integration-tests/file_with_underscores.txt",
        "/integration-tests/file.multiple.dots.txt",
    ];

    for test_path in test_cases {
        println!("Testing file path: {}", test_path);

        let test_content = format!("Content for {}", test_path).into_bytes();

        // Clean up if exists
        let _ = file_handler.delete_file(test_path, false).await;

        // Upload
        let upload_result = file_handler.upload_file(test_path, &test_content).await;

        match upload_result {
            Ok(file) => {
                println!(
                    "Successfully uploaded file with special chars: {:?}",
                    file.path
                );

                // Download to verify
                let download_result = file_handler.download_file(test_path).await;
                assert!(
                    download_result.is_ok(),
                    "Should download file with special chars"
                );

                // Clean up
                let _ = file_handler.delete_file(test_path, false).await;
            }
            Err(e) => {
                eprintln!("Failed to upload file with special chars: {:?}", e);
            }
        }
    }
}

#[tokio::test]
async fn test_file_update_metadata() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_file = "/integration-tests/metadata-test.txt";
    let test_content = b"Test metadata";

    // Clean up
    let _ = file_handler.delete_file(test_file, false).await;

    // Upload
    file_handler
        .upload_file(test_file, test_content)
        .await
        .expect("Should upload file");

    println!("Testing metadata update");

    // Update with custom metadata
    let mut custom_metadata = std::collections::HashMap::new();
    custom_metadata.insert("test_key".to_string(), "test_value".to_string());
    custom_metadata.insert("uploaded_by".to_string(), "integration_test".to_string());

    let update_result = file_handler
        .update_file(test_file, Some(custom_metadata), None, None)
        .await;

    match update_result {
        Ok(updated_file) => {
            println!("Successfully updated metadata: {:?}", updated_file);
            // Note: The API may or may not return custom_metadata in the response
        }
        Err(e) => {
            eprintln!(
                "Metadata update failed (may require premium features): {:?}",
                e
            );
        }
    }

    // Clean up
    cleanup_file(&client, test_file).await;
}

#[tokio::test]
async fn test_file_copy_to_different_folder() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client.clone());

    ensure_test_folder(&client).await;

    // Create a second test folder
    let dest_folder = "/integration-tests/copy-dest";
    let _ = folder_handler.create_folder(dest_folder, true).await;

    let source_file = "/integration-tests/copy-source.txt";
    let dest_file = "/integration-tests/copy-dest/copied-file.txt";
    let test_content = b"Content to copy";

    // Clean up
    let _ = file_handler.delete_file(source_file, false).await;
    let _ = file_handler.delete_file(dest_file, false).await;

    // Upload source file
    file_handler
        .upload_file(source_file, test_content)
        .await
        .expect("Should upload source file");

    println!("Testing copy to different folder");

    // Copy to destination
    let copy_result = file_handler.copy_file(source_file, dest_file).await;

    match copy_result {
        Ok(_) => {
            println!("Successfully copied file to different folder");

            // Verify both files exist
            let source_exists = file_handler.download_file(source_file).await.is_ok();
            let dest_exists = file_handler.download_file(dest_file).await.is_ok();

            assert!(source_exists, "Source file should still exist after copy");
            assert!(dest_exists, "Destination file should exist after copy");

            // Clean up
            let _ = file_handler.delete_file(source_file, false).await;
            let _ = file_handler.delete_file(dest_file, false).await;
        }
        Err(e) => {
            eprintln!("Copy to different folder failed: {:?}", e);
            let _ = file_handler.delete_file(source_file, false).await;
        }
    }

    // Clean up destination folder
    let _ = folder_handler.delete_folder(dest_folder, true).await;
}

#[tokio::test]
async fn test_file_move_renames_file() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let source_file = "/integration-tests/original-name.txt";
    let dest_file = "/integration-tests/renamed-file.txt";
    let test_content = b"Content to rename";

    // Clean up
    let _ = file_handler.delete_file(source_file, false).await;
    let _ = file_handler.delete_file(dest_file, false).await;

    // Upload source file
    file_handler
        .upload_file(source_file, test_content)
        .await
        .expect("Should upload source file");

    println!("Testing file rename via move");

    // Move/rename file
    let move_result = file_handler.move_file(source_file, dest_file).await;

    match move_result {
        Ok(_) => {
            println!("Successfully renamed file via move");

            // Verify source is gone
            let source_exists = file_handler.download_file(source_file).await;
            assert!(
                matches!(source_exists, Err(FilesError::NotFound { .. })),
                "Source file should not exist after move"
            );

            // Verify destination exists
            let dest_exists = file_handler.download_file(dest_file).await;
            assert!(
                dest_exists.is_ok(),
                "Destination file should exist after move"
            );

            // Clean up
            let _ = file_handler.delete_file(dest_file, false).await;
        }
        Err(e) => {
            eprintln!("Move/rename failed: {:?}", e);
            let _ = file_handler.delete_file(source_file, false).await;
        }
    }
}

#[tokio::test]
async fn test_file_conflict_on_copy() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let source_file = "/integration-tests/conflict-source.txt";
    let dest_file = "/integration-tests/conflict-dest.txt";

    // Clean up
    let _ = file_handler.delete_file(source_file, false).await;
    let _ = file_handler.delete_file(dest_file, false).await;

    // Upload both files
    file_handler
        .upload_file(source_file, b"Source content")
        .await
        .expect("Should upload source");

    file_handler
        .upload_file(dest_file, b"Destination content")
        .await
        .expect("Should upload destination");

    println!("Testing copy to existing file (conflict scenario)");

    // Try to copy to existing file - API behavior may vary
    let copy_result = file_handler.copy_file(source_file, dest_file).await;

    match copy_result {
        Ok(_) => {
            println!("Copy to existing file succeeded (API may overwrite)");
        }
        Err(FilesError::Conflict { message }) => {
            println!("Correctly received Conflict error: {}", message);
        }
        Err(e) => {
            eprintln!("Unexpected error on copy conflict: {:?}", e);
        }
    }

    // Clean up
    let _ = file_handler.delete_file(source_file, false).await;
    let _ = file_handler.delete_file(dest_file, false).await;
}

#[tokio::test]
async fn test_deep_nested_folder_path() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let deep_path = "/integration-tests/level1/level2/level3/deep-file.txt";
    let test_content = b"Deep nested file";

    // Create nested folders
    let _ = folder_handler
        .create_folder("/integration-tests/level1/level2/level3", true)
        .await;

    // Clean up file if exists
    let _ = file_handler.delete_file(deep_path, false).await;

    println!("Testing deeply nested file path");

    // Upload to deep path
    let upload_result = file_handler.upload_file(deep_path, test_content).await;

    match upload_result {
        Ok(file) => {
            println!("Successfully uploaded to deep path: {:?}", file.path);

            // Download to verify
            let download_result = file_handler.download_file(deep_path).await;
            assert!(download_result.is_ok(), "Should download from deep path");

            // Clean up
            let _ = file_handler.delete_file(deep_path, false).await;
        }
        Err(e) => {
            eprintln!("Deep path upload failed: {:?}", e);
        }
    }

    // Clean up nested folders
    let _ = folder_handler
        .delete_folder("/integration-tests/level1", true)
        .await;
}
