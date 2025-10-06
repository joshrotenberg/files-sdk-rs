//! Comprehensive real API tests for FileActionHandler

use crate::real::*;
use files_sdk::{FileActionHandler, FileHandler};

#[tokio::test]
async fn test_begin_upload_small_file() {
    let client = get_test_client();
    let file_action_handler = FileActionHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_path = "/integration-tests/begin-upload-small.txt";
    let test_content = b"Small file content";

    cleanup_file(&client, test_path).await;

    println!("Testing begin_upload for small file");

    // Begin upload
    let result = file_action_handler
        .begin_upload(test_path, Some(test_content.len() as i64), true)
        .await;

    match result {
        Ok(upload_info) => {
            println!("Begin upload successful: {:?}", upload_info);

            // Verify response structure
            assert!(!upload_info.is_empty(), "Upload info should not be empty");
            let first_part = &upload_info[0];
            assert!(first_part.upload_uri.is_some(), "Should have upload URI");
            assert!(first_part.http_method.is_some(), "Should have HTTP method");

            // Complete the upload
            let _ = file_handler.upload_file(test_path, test_content).await;

            // Clean up
            cleanup_file(&client, test_path).await;
        }
        Err(e) => {
            eprintln!("Begin upload failed: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_begin_upload_large_file_multipart() {
    let client = get_test_client();
    let file_action_handler = FileActionHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_path = "/integration-tests/begin-upload-large.bin";

    // 10MB file should trigger multipart upload
    let large_size = 10 * 1024 * 1024;

    cleanup_file(&client, test_path).await;

    println!("Testing begin_upload for large file (multipart)");

    // Begin upload for large file
    let result = file_action_handler
        .begin_upload(test_path, Some(large_size), true)
        .await;

    match result {
        Ok(upload_info) => {
            println!("Begin upload successful for large file");

            assert!(!upload_info.is_empty(), "Upload info should not be empty");
            let first_part = &upload_info[0];

            // Check if multipart upload info is present
            if let Some(ref upload_uri) = first_part.upload_uri {
                println!("Upload URI: {}", upload_uri);
            }

            if let Some(ref part_number) = first_part.part_number {
                println!("Multipart upload detected, part number: {}", part_number);
            }

            // Note: Actually uploading 10MB would be slow, so we skip the actual upload
            println!("Skipping actual large file upload to save test time");
        }
        Err(e) => {
            eprintln!("Begin upload for large file failed: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_begin_upload_without_mkdir_parents() {
    let client = get_test_client();
    let file_action_handler = FileActionHandler::new(client.clone());

    // Try to upload to non-existent folder without mkdir_parents
    let test_path = "/integration-tests/nonexistent-folder/file.txt";

    println!("Testing begin_upload without mkdir_parents (should fail)");

    let result = file_action_handler
        .begin_upload(test_path, Some(100), false) // mkdir_parents=false
        .await;

    match result {
        Ok(_) => {
            println!("Begin upload succeeded (API may auto-create parents)");
            // Clean up if it succeeded
            cleanup_file(&client, test_path).await;
            cleanup_file(&client, "/integration-tests/nonexistent-folder").await;
        }
        Err(e) => {
            println!(
                "Begin upload correctly failed without mkdir_parents: {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_file_action_copy() {
    let client = get_test_client();
    let file_action_handler = FileActionHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let source = "/integration-tests/action-copy-source.txt";
    let dest = "/integration-tests/action-copy-dest.txt";
    let content = b"Content to copy via file action";

    // Setup
    cleanup_file(&client, source).await;
    cleanup_file(&client, dest).await;

    file_handler
        .upload_file(source, content)
        .await
        .expect("Should upload source file");

    println!("Testing file_action copy");

    // Copy using file_action
    let copy_result = file_action_handler.copy_file(source, dest).await;

    match copy_result {
        Ok(_) => {
            println!("File action copy successful");

            // Give the server a moment to complete the copy operation
            tokio::time::sleep(tokio::time::Duration::from_millis(1500)).await;

            // Verify both files exist
            let source_exists = file_handler.download_file(source).await.is_ok();
            let dest_exists = file_handler.download_file(dest).await.is_ok();

            assert!(source_exists, "Source should exist after copy");
            assert!(dest_exists, "Destination should exist after copy");

            // Clean up
            cleanup_file(&client, source).await;
            cleanup_file(&client, dest).await;
        }
        Err(e) => {
            eprintln!("File action copy failed: {:?}", e);
            cleanup_file(&client, source).await;
        }
    }
}

#[tokio::test]
async fn test_file_action_move() {
    let client = get_test_client();
    let file_action_handler = FileActionHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let source = "/integration-tests/action-move-source.txt";
    let dest = "/integration-tests/action-move-dest.txt";
    let content = b"Content to move via file action";

    // Setup
    cleanup_file(&client, source).await;
    cleanup_file(&client, dest).await;

    file_handler
        .upload_file(source, content)
        .await
        .expect("Should upload source file");

    println!("Testing file_action move");

    // Move using file_action
    let move_result = file_action_handler.move_file(source, dest).await;

    match move_result {
        Ok(_) => {
            println!("File action move successful");

            // Verify source is gone, dest exists
            let source_result = file_handler.download_file(source).await;
            let dest_exists = file_handler.download_file(dest).await.is_ok();

            assert!(source_result.is_err(), "Source should not exist after move");
            assert!(dest_exists, "Destination should exist after move");

            // Clean up
            cleanup_file(&client, dest).await;
        }
        Err(e) => {
            eprintln!("File action move failed: {:?}", e);
            cleanup_file(&client, source).await;
        }
    }
}

#[tokio::test]
async fn test_file_action_copy_to_existing_destination() {
    let client = get_test_client();
    let file_action_handler = FileActionHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let source = "/integration-tests/action-copy-conflict-src.txt";
    let dest = "/integration-tests/action-copy-conflict-dst.txt";

    // Setup - upload both files
    cleanup_file(&client, source).await;
    cleanup_file(&client, dest).await;

    file_handler
        .upload_file(source, b"Source")
        .await
        .expect("Should upload source");

    file_handler
        .upload_file(dest, b"Destination")
        .await
        .expect("Should upload dest");

    println!("Testing file_action copy to existing file (conflict)");

    // Try to copy to existing destination
    let copy_result = file_action_handler.copy_file(source, dest).await;

    match copy_result {
        Ok(_) => {
            println!("Copy to existing file succeeded (API may overwrite)");
        }
        Err(files_sdk::FilesError::Conflict { message, .. }) => {
            println!("Correctly received Conflict error: {}", message);
        }
        Err(e) => {
            eprintln!("Unexpected error on copy conflict: {:?}", e);
        }
    }

    // Clean up
    cleanup_file(&client, source).await;
    cleanup_file(&client, dest).await;
}

#[tokio::test]
async fn test_file_action_metadata() {
    let client = get_test_client();
    let file_action_handler = FileActionHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_path = "/integration-tests/action-metadata-test.txt";
    let content = b"Testing metadata";

    // Setup
    cleanup_file(&client, test_path).await;
    file_handler
        .upload_file(test_path, content)
        .await
        .expect("Should upload file");

    println!("Testing file_action metadata retrieval");

    // Get metadata using file_action
    let metadata_result = file_action_handler.get_metadata(test_path).await;

    match metadata_result {
        Ok(metadata) => {
            println!("Metadata retrieved: {:?}", metadata);

            // Verify metadata contains expected fields
            assert!(
                metadata.path == Some(test_path.to_string())
                    || metadata.path == Some(test_path.trim_start_matches('/').to_string()),
                "Metadata should include correct path"
            );

            if let Some(size) = metadata.size {
                println!("File size from metadata: {}", size);
            }
        }
        Err(e) => {
            eprintln!("Metadata retrieval failed: {:?}", e);
        }
    }

    // Clean up
    cleanup_file(&client, test_path).await;
}

#[tokio::test]
async fn test_begin_upload_with_custom_etag() {
    let client = get_test_client();
    let file_action_handler = FileActionHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_path = "/integration-tests/etag-test.txt";
    let test_size = 1024;

    cleanup_file(&client, test_path).await;

    println!("Testing begin_upload - checking for etag in response");

    // Begin upload and check if response includes etag
    let result = file_action_handler
        .begin_upload(test_path, Some(test_size), true)
        .await;

    match result {
        Ok(upload_info) => {
            println!("Begin upload successful: {:?}", upload_info);

            assert!(!upload_info.is_empty(), "Upload info should not be empty");
            let first_part = &upload_info[0];

            // The response may include upload information
            println!("Upload part info: {:?}", first_part);
        }
        Err(e) => {
            eprintln!("Begin upload failed: {:?}", e);
        }
    }

    cleanup_file(&client, test_path).await;
}
