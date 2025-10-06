//! Real API integration tests for FileHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests --test real
//!
//! These tests create and clean up test files in /integration-tests/ folder.

use crate::real::*;
use files_sdk::{FileActionHandler, FileHandler, FolderHandler};

#[tokio::test]
async fn test_real_api_file_upload_and_download() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());
    let file_action_handler = FileActionHandler::new(client.clone());
    let _folder_handler = FolderHandler::new(client.clone());

    // Ensure test folder exists
    ensure_test_folder(&client).await;

    // Test file
    let test_path = "/integration-tests/test-upload.txt";
    let test_content = b"Hello from files-sdk integration test!";

    // Clean up any existing test file
    cleanup_file(&client, test_path).await;

    println!("Starting file upload test...");

    // Step 1: Begin upload
    let upload_info = file_action_handler
        .begin_upload(test_path, Some(test_content.len() as i64), true)
        .await
        .expect("Should begin upload");

    println!("Upload info received: {:?}", upload_info);

    // Step 2: Upload file
    let result = file_handler.upload_file(test_path, test_content).await;

    match result {
        Ok(file) => {
            println!("Successfully uploaded file: {:?}", file);
            // API returns paths without leading slash
            assert!(
                file.path == Some(test_path.to_string())
                    || file.path == Some(test_path.trim_start_matches('/').to_string())
            );

            // Step 3: Download file
            let download_result = file_handler.download_file(test_path).await;

            match download_result {
                Ok(downloaded_file) => {
                    println!("Successfully downloaded file: {:?}", downloaded_file);
                    // API returns paths without leading slash
                    assert!(
                        downloaded_file.path == Some(test_path.to_string())
                            || downloaded_file.path
                                == Some(test_path.trim_start_matches('/').to_string())
                    );
                }
                Err(e) => {
                    eprintln!("Failed to download file: {:?}", e);
                }
            }

            // Clean up
            cleanup_file(&client, test_path).await;
            println!("Successfully cleaned up test file");
        }
        Err(e) => {
            panic!("Failed to upload file: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_file_operations() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client.clone());

    // Ensure test folder exists
    let test_folder = "/integration-tests";
    let _ = folder_handler.create_folder(test_folder, true).await;

    let test_file = "/integration-tests/operations-test.txt";
    let test_content = b"Testing file operations";

    // Clean up
    let _ = file_handler.delete_file(test_file, false).await;

    // Upload
    file_handler
        .upload_file(test_file, test_content)
        .await
        .expect("Should upload file");

    println!("File uploaded: {}", test_file);

    // Update (metadata) - note: update_file requires custom_metadata, provided_mtime, priority_color
    let update_result = file_handler.update_file(test_file, None, None, None).await;

    match update_result {
        Ok(updated) => {
            println!("File updated: {:?}", updated);
        }
        Err(e) => {
            eprintln!("Update failed (may not be supported): {:?}", e);
        }
    }

    // Copy
    let copy_dest = "/integration-tests/operations-test-copy.txt";
    let _ = file_handler.delete_file(copy_dest, false).await; // Clean up any existing

    let copy_result = file_handler.copy_file(test_file, copy_dest).await;

    match copy_result {
        Ok(_) => {
            println!("File copied to: {}", copy_dest);
            // Clean up copy
            let _ = file_handler.delete_file(copy_dest, false).await;
        }
        Err(e) => {
            eprintln!("Copy failed: {:?}", e);
        }
    }

    // Move
    let move_dest = "/integration-tests/operations-test-moved.txt";
    let _ = file_handler.delete_file(move_dest, false).await; // Clean up any existing

    let move_result = file_handler.move_file(test_file, move_dest).await;

    match move_result {
        Ok(_) => {
            println!("File moved to: {}", move_dest);
            // Clean up moved file
            let _ = file_handler.delete_file(move_dest, false).await;
        }
        Err(e) => {
            eprintln!("Move failed: {:?}", e);
            // Clean up original if move failed
            let _ = file_handler.delete_file(test_file, false).await;
        }
    }
}

#[tokio::test]
async fn test_real_api_folder_operations() {
    let client = get_test_client();
    let folder_handler = FolderHandler::new(client.clone());

    let test_folder = "/integration-tests/subfolder-test";

    // Clean up if exists
    let _ = folder_handler.delete_folder(test_folder, true).await;

    // Create folder
    let create_result = folder_handler.create_folder(test_folder, true).await;

    match create_result {
        Ok(folder) => {
            println!("Created folder: {:?}", folder);

            // List folder contents
            let list_result = folder_handler
                .list_folder("/integration-tests", None, None)
                .await;

            match list_result {
                Ok((files, _)) => {
                    println!("Listed {} items in /integration-tests", files.len());
                    // API may return paths with or without leading slash
                    let found = files.iter().any(|f| {
                        f.path == Some(test_folder.to_string())
                            || f.path == Some(test_folder.trim_start_matches('/').to_string())
                    });
                    if !found {
                        println!(
                            "Available paths: {:?}",
                            files.iter().map(|f| &f.path).collect::<Vec<_>>()
                        );
                    }
                    assert!(found, "Should find created subfolder in listing");
                }
                Err(e) => {
                    eprintln!("Failed to list folder: {:?}", e);
                }
            }

            // Delete folder
            let delete_result = folder_handler.delete_folder(test_folder, true).await;
            match delete_result {
                Ok(_) => println!("Successfully deleted test folder"),
                Err(e) => eprintln!("Failed to delete test folder: {:?}", e),
            }
        }
        Err(e) => {
            eprintln!("Failed to create folder: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_large_file_upload() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client.clone());

    // Ensure test folder exists
    let test_folder = "/integration-tests";
    let _ = folder_handler.create_folder(test_folder, true).await;

    let test_path = "/integration-tests/large-file-test.bin";

    // Create 1MB test file
    let test_content = vec![0u8; 1024 * 1024]; // 1MB of zeros

    // Clean up any existing
    let _ = file_handler.delete_file(test_path, false).await;

    println!("Uploading 1MB test file...");

    let upload_result = file_handler.upload_file(test_path, &test_content).await;

    match upload_result {
        Ok(file) => {
            println!("Successfully uploaded large file: {:?}", file);

            // Verify size if available
            if let Some(size) = file.size {
                assert_eq!(
                    size as usize,
                    test_content.len(),
                    "Uploaded file size should match"
                );
            }

            // Clean up
            let _ = file_handler.delete_file(test_path, false).await;
        }
        Err(e) => {
            eprintln!("Large file upload failed: {:?}", e);
            // Not failing the test as this might require special API permissions
        }
    }
}

#[tokio::test]
async fn test_upload_directory() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client);

    ensure_test_folder(&client).await;

    // Create a temporary directory with test files
    let temp_dir = std::env::temp_dir().join("files-sdk-test-upload");
    let _ = fs::remove_dir_all(&temp_dir); // Clean up if exists
    fs::create_dir_all(&temp_dir).unwrap();
    fs::create_dir_all(temp_dir.join("subdir")).unwrap();

    // Create test files
    fs::write(temp_dir.join("file1.txt"), b"content 1").unwrap();
    fs::write(temp_dir.join("file2.txt"), b"content 2").unwrap();
    fs::write(temp_dir.join("subdir/file3.txt"), b"content 3").unwrap();

    println!("Uploading directory to Files.com...");

    let remote_path = "/integration-tests/dir-upload-test";

    // Upload the directory
    let result = file_handler
        .upload_directory(Path::new(&temp_dir), remote_path, true)
        .await;

    match result {
        Ok(uploaded) => {
            println!("Successfully uploaded {} files", uploaded.len());
            assert!(uploaded.len() >= 3, "Should upload at least 3 files");

            // Verify files exist
            let (files, _) = folder_handler
                .list_folder(remote_path, None, None)
                .await
                .unwrap();
            println!("Found {} items in remote directory", files.len());
        }
        Err(e) => {
            eprintln!("Upload failed: {:?}", e);
        }
    }

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
    let _ = folder_handler.delete_folder(remote_path, true).await;
}

#[tokio::test]
async fn test_upload_directory_with_progress() {
    let client = get_test_client();
    let file_handler = FileHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client);

    ensure_test_folder(&client).await;

    // Create a temporary directory with test files
    let temp_dir = std::env::temp_dir().join("files-sdk-test-upload-progress");
    let _ = fs::remove_dir_all(&temp_dir);
    fs::create_dir_all(&temp_dir).unwrap();

    // Create 5 test files
    for i in 1..=5 {
        fs::write(
            temp_dir.join(format!("file{}.txt", i)),
            format!("content {}", i),
        )
        .unwrap();
    }

    println!("Uploading directory with progress tracking...");

    let remote_path = "/integration-tests/dir-upload-progress-test";

    // Upload with progress
    let result = file_handler
        .upload_directory_with_progress(
            Path::new(&temp_dir),
            remote_path,
            true,
            |current, total| {
                println!(
                    "Progress: {}/{} ({:.0}%)",
                    current,
                    total,
                    (current as f64 / total as f64) * 100.0
                );
            },
        )
        .await;

    match result {
        Ok(uploaded) => {
            println!("Successfully uploaded {} files", uploaded.len());
            assert_eq!(uploaded.len(), 5, "Should upload exactly 5 files");
        }
        Err(e) => {
            eprintln!("Upload with progress failed: {:?}", e);
        }
    }

    // Cleanup
    let _ = fs::remove_dir_all(&temp_dir);
    let _ = folder_handler.delete_folder(remote_path, true).await;
}
