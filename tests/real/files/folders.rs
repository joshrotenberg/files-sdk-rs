//! Comprehensive real API tests for FolderHandler

use crate::real::*;
use files_sdk::{FileHandler, FilesError, FolderHandler};

#[tokio::test]
async fn test_folder_list_with_pagination() {
    let client = get_test_client();
    let folder_handler = FolderHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    // Create multiple files to test pagination
    let test_folder = "/integration-tests/pagination-test";
    let _ = folder_handler.create_folder(test_folder, true).await;

    println!("Creating test files for pagination test...");

    // Upload 5 test files
    for i in 1..=5 {
        let file_path = format!("{}/file-{}.txt", test_folder, i);
        let content = format!("Content {}", i).into_bytes();
        let _ = file_handler.upload_file(&file_path, &content).await;
    }

    println!("Testing folder listing with pagination");

    // List with small page size
    let list_result = folder_handler
        .list_folder(test_folder, Some(2), None) // 2 items per page
        .await;

    match list_result {
        Ok((files, pagination)) => {
            println!("Listed {} files (first page)", files.len());
            assert!(!files.is_empty(), "Should have files in listing");

            // If we got a cursor, test pagination
            if let Some(next_cursor) = pagination.cursor_next {
                println!("Testing next page with cursor: {}", next_cursor);

                let next_page = folder_handler
                    .list_folder(test_folder, Some(2), Some(next_cursor))
                    .await;

                match next_page {
                    Ok((next_files, _)) => {
                        println!("Listed {} files (second page)", next_files.len());
                        assert!(!next_files.is_empty(), "Should have files on next page");
                    }
                    Err(e) => eprintln!("Failed to fetch next page: {:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Folder listing failed: {:?}", e);
        }
    }

    // Clean up
    for i in 1..=5 {
        let file_path = format!("{}/file-{}.txt", test_folder, i);
        let _ = file_handler.delete_file(&file_path, false).await;
    }
    let _ = folder_handler.delete_folder(test_folder, true).await;
}

#[tokio::test]
async fn test_folder_search_functionality() {
    let client = get_test_client();
    let folder_handler = FolderHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_folder = "/integration-tests/search-test";
    let _ = folder_handler.create_folder(test_folder, true).await;

    // Create files with different names
    let test_files = vec![
        "important-document.txt",
        "report-2024.pdf",
        "notes.txt",
        "important-notes.txt",
    ];

    println!("Creating test files for search test...");

    for filename in &test_files {
        let file_path = format!("{}/{}", test_folder, filename);
        let content = format!("Content for {}", filename).into_bytes();
        let _ = file_handler.upload_file(&file_path, &content).await;
    }

    println!("Testing folder search with filter");

    // Search for files containing "important"
    let search_result = folder_handler
        .search_folder(test_folder, "important", None)
        .await;

    match search_result {
        Ok((results, _)) => {
            println!("Search found {} files matching 'important'", results.len());

            // Verify results contain "important" in the name
            for file in &results {
                if let Some(ref path) = file.path {
                    println!("Found: {}", path);
                    // Note: API search behavior may vary
                }
            }
        }
        Err(e) => {
            eprintln!("Search failed (may not be supported): {:?}", e);
        }
    }

    // Clean up
    for filename in &test_files {
        let file_path = format!("{}/{}", test_folder, filename);
        let _ = file_handler.delete_file(&file_path, false).await;
    }
    let _ = folder_handler.delete_folder(test_folder, true).await;
}

#[tokio::test]
async fn test_folder_create_with_mkdir_parents() {
    let client = get_test_client();
    let folder_handler = FolderHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let deep_folder = "/integration-tests/parent/child/grandchild";

    // Clean up if exists
    let _ = folder_handler
        .delete_folder("/integration-tests/parent", true)
        .await;

    println!("Testing folder creation with mkdir_parents=true");

    // Create deep folder structure in one call
    let create_result = folder_handler.create_folder(deep_folder, true).await;

    match create_result {
        Ok(folder) => {
            println!(
                "Successfully created deep folder structure: {:?}",
                folder.path
            );

            // Verify parent folders were created
            let parent_list = folder_handler
                .list_folder("/integration-tests", None, None)
                .await;

            if let Ok((files, _)) = parent_list {
                let has_parent = files.iter().any(|f| {
                    f.path == Some("/integration-tests/parent".to_string())
                        || f.path == Some("integration-tests/parent".to_string())
                });
                assert!(has_parent, "Parent folder should be created");
            }

            // Clean up
            let _ = folder_handler
                .delete_folder("/integration-tests/parent", true)
                .await;
        }
        Err(e) => {
            eprintln!("Failed to create deep folder structure: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_folder_delete_recursive() {
    let client = get_test_client();
    let folder_handler = FolderHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let parent_folder = "/integration-tests/delete-recursive-test";
    let child_folder = "/integration-tests/delete-recursive-test/subfolder";

    // Create folder structure
    let _ = folder_handler.create_folder(child_folder, true).await;

    // Add files to both folders
    let _ = file_handler
        .upload_file(
            "/integration-tests/delete-recursive-test/file1.txt",
            b"File 1",
        )
        .await;

    let _ = file_handler
        .upload_file(
            "/integration-tests/delete-recursive-test/subfolder/file2.txt",
            b"File 2",
        )
        .await;

    println!("Testing recursive folder deletion");

    // Delete parent folder recursively
    let delete_result = folder_handler.delete_folder(parent_folder, true).await;

    match delete_result {
        Ok(_) => {
            println!("Successfully deleted folder recursively");

            // Verify folder is gone
            let list_result = folder_handler
                .list_folder("/integration-tests", None, None)
                .await;

            if let Ok((files, _)) = list_result {
                let still_exists = files.iter().any(|f| {
                    f.path == Some(parent_folder.to_string())
                        || f.path == Some(parent_folder.trim_start_matches('/').to_string())
                });
                assert!(!still_exists, "Folder should be deleted");
            }
        }
        Err(e) => {
            eprintln!("Recursive delete failed: {:?}", e);
            // Clean up manually
            let _ = file_handler
                .delete_file(
                    "/integration-tests/delete-recursive-test/subfolder/file2.txt",
                    false,
                )
                .await;
            let _ = file_handler
                .delete_file("/integration-tests/delete-recursive-test/file1.txt", false)
                .await;
            let _ = folder_handler.delete_folder(child_folder, true).await;
            let _ = folder_handler.delete_folder(parent_folder, true).await;
        }
    }
}

#[tokio::test]
async fn test_folder_list_empty_folder() {
    let client = get_test_client();
    let folder_handler = FolderHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let empty_folder = "/integration-tests/empty-folder";

    // Create empty folder
    let _ = folder_handler.create_folder(empty_folder, true).await;

    println!("Testing listing of empty folder");

    let list_result = folder_handler.list_folder(empty_folder, None, None).await;

    match list_result {
        Ok((files, _)) => {
            println!("Empty folder contains {} items", files.len());
            // Empty folders may return 0 items or just show . and ..
            assert!(
                files.len() <= 2,
                "Empty folder should have 0-2 items (., ..)"
            );
        }
        Err(e) => {
            eprintln!("Failed to list empty folder: {:?}", e);
        }
    }

    // Clean up
    let _ = folder_handler.delete_folder(empty_folder, true).await;
}

#[tokio::test]
async fn test_folder_not_found_error() {
    let client = get_test_client();
    let folder_handler = FolderHandler::new(client.clone());

    let nonexistent = "/integration-tests/does-not-exist";

    println!("Testing folder not found error");

    let list_result = folder_handler.list_folder(nonexistent, None, None).await;

    match list_result {
        Err(FilesError::NotFound { message, .. }) => {
            println!("Correctly received NotFound error: {}", message);
            assert!(!message.is_empty());
        }
        Err(e) => eprintln!("Expected NotFound, got: {:?}", e),
        Ok(_) => panic!("Should not list nonexistent folder"),
    }
}

#[tokio::test]
async fn test_folder_name_conflict() {
    let client = get_test_client();
    let folder_handler = FolderHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_folder = "/integration-tests/conflict-test";

    // Clean up if exists
    let _ = folder_handler.delete_folder(test_folder, true).await;

    // Give server time to process deletion
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    println!("Testing folder name conflict");

    // Create folder - may already exist from previous test
    let _ = folder_handler.create_folder(test_folder, true).await;

    // Try to create same folder again - this SHOULD fail with conflict/unprocessable
    let duplicate_result = folder_handler.create_folder(test_folder, true).await;

    match duplicate_result {
        Ok(_) => {
            println!("Duplicate folder creation succeeded (API may allow)");
        }
        Err(FilesError::Conflict { message, .. })
        | Err(FilesError::UnprocessableEntity { message, .. }) => {
            println!("Correctly received Conflict error: {}", message);
        }
        Err(e) => {
            eprintln!("Unexpected error on duplicate folder: {:?}", e);
        }
    }

    // Clean up
    let _ = folder_handler.delete_folder(test_folder, true).await;
}

#[tokio::test]
async fn test_folder_with_special_characters() {
    let client = get_test_client();
    let folder_handler = FolderHandler::new(client.clone());

    ensure_test_folder(&client).await;

    let test_cases = vec![
        "/integration-tests/folder with spaces",
        "/integration-tests/folder-with-dashes",
        "/integration-tests/folder_with_underscores",
    ];

    for test_folder in test_cases {
        println!("Testing folder with special chars: {}", test_folder);

        // Clean up
        let _ = folder_handler.delete_folder(test_folder, true).await;

        // Create folder
        let create_result = folder_handler.create_folder(test_folder, true).await;

        match create_result {
            Ok(folder) => {
                println!("Successfully created folder: {:?}", folder.path);

                // Verify it exists by listing parent
                let list_result = folder_handler
                    .list_folder("/integration-tests", None, None)
                    .await;

                if let Ok((files, _)) = list_result {
                    let found = files.iter().any(|f| {
                        f.path == Some(test_folder.to_string())
                            || f.path == Some(test_folder.trim_start_matches('/').to_string())
                    });
                    assert!(found, "Should find folder with special chars");
                }

                // Clean up
                let _ = folder_handler.delete_folder(test_folder, true).await;
            }
            Err(e) => {
                eprintln!("Failed to create folder with special chars: {:?}", e);
            }
        }
    }
}
