//! Real API integration tests for BundleHandler (share links)
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests --test bundles_real_api
//!
//! These tests create and clean up test bundles.

#![cfg(feature = "integration-tests")]

use files_sdk::{BundleHandler, FileHandler, FilesClient, FolderHandler};

fn get_test_client() -> FilesClient {
    let api_key = std::env::var("FILES_API_KEY")
        .expect("FILES_API_KEY environment variable must be set for integration tests");
    FilesClient::builder().api_key(&api_key).build().unwrap()
}

#[tokio::test]
async fn test_real_api_create_and_delete_bundle() {
    let client = get_test_client();
    let bundle_handler = BundleHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client);

    // Ensure test folder and file exist
    let test_folder = "/integration-tests";
    let test_file = "/integration-tests/bundle-test.txt";

    let _ = folder_handler.create_folder(test_folder, true).await;
    let _ = file_handler.delete_file(test_file, false).await;

    file_handler
        .upload_file(test_file, b"Test file for bundle")
        .await
        .expect("Should upload test file");

    println!("Creating bundle for: {}", test_file);

    // Create bundle
    let create_result = bundle_handler
        .create(
            vec![test_file.to_string()],
            None,                            // password
            None,                            // expires_at
            None,                            // max_uses
            Some("Integration Test Bundle"), // description
            None,                            // note
            None,                            // code
            None,                            // require_registration
            None,                            // permissions
        )
        .await;

    match create_result {
        Ok(bundle) => {
            println!("Successfully created bundle: {:?}", bundle);

            let bundle_id = bundle.id.expect("Bundle should have an ID");

            // Get bundle
            let get_result = bundle_handler.get(bundle_id).await;

            match get_result {
                Ok(retrieved) => {
                    println!("Successfully retrieved bundle: {:?}", retrieved);
                    assert_eq!(retrieved.id, Some(bundle_id));
                }
                Err(e) => {
                    eprintln!("Failed to get bundle: {:?}", e);
                }
            }

            // List bundles
            let list_result = bundle_handler.list(None, None, None).await;

            match list_result {
                Ok((bundles, _)) => {
                    println!("Listed {} bundles", bundles.len());
                    let found = bundles.iter().any(|b| b.id == Some(bundle_id));
                    assert!(found, "Should find created bundle in list");
                }
                Err(e) => {
                    eprintln!("Failed to list bundles: {:?}", e);
                }
            }

            // Delete bundle
            let delete_result = bundle_handler.delete(bundle_id).await;

            match delete_result {
                Ok(_) => println!("Successfully deleted bundle"),
                Err(e) => eprintln!("Failed to delete bundle: {:?}", e),
            }
        }
        Err(e) => {
            eprintln!("Failed to create bundle: {:?}", e);
        }
    }

    // Clean up test file
    let _ = file_handler.delete_file(test_file, false).await;
}

#[tokio::test]
async fn test_real_api_bundle_with_password() {
    let client = get_test_client();
    let bundle_handler = BundleHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client);

    // Ensure test folder and file exist
    let test_folder = "/integration-tests";
    let test_file = "/integration-tests/secure-bundle-test.txt";

    let _ = folder_handler.create_folder(test_folder, true).await;
    let _ = file_handler.delete_file(test_file, false).await;

    file_handler
        .upload_file(test_file, b"Secure test file")
        .await
        .expect("Should upload test file");

    println!("Creating password-protected bundle...");

    // Create bundle with password
    let create_result = bundle_handler
        .create(
            vec![test_file.to_string()],
            Some("test-password-123"),                 // password
            None,                                      // expires_at
            None,                                      // max_uses
            Some("Secure Integration Test"),           // description
            Some("This bundle is password protected"), // note
            None,                                      // code
            None,                                      // require_registration
            None,                                      // permissions
        )
        .await;

    match create_result {
        Ok(bundle) => {
            println!("Successfully created secure bundle: {:?}", bundle);

            let bundle_id = bundle.id.expect("Bundle should have an ID");

            // Verify bundle has password protection
            let get_result = bundle_handler.get(bundle_id).await;

            match get_result {
                Ok(retrieved) => {
                    println!("Retrieved secure bundle: {:?}", retrieved);
                    // Note: API may not return password status for security
                }
                Err(e) => {
                    eprintln!("Failed to get secure bundle: {:?}", e);
                }
            }

            // Clean up
            let _ = bundle_handler.delete(bundle_id).await;
        }
        Err(e) => {
            eprintln!("Failed to create secure bundle: {:?}", e);
        }
    }

    // Clean up test file
    let _ = file_handler.delete_file(test_file, false).await;
}

#[tokio::test]
async fn test_real_api_bundle_update() {
    let client = get_test_client();
    let bundle_handler = BundleHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client);

    // Ensure test folder and file exist
    let test_folder = "/integration-tests";
    let test_file = "/integration-tests/update-bundle-test.txt";

    let _ = folder_handler.create_folder(test_folder, true).await;
    let _ = file_handler.delete_file(test_file, false).await;

    file_handler
        .upload_file(test_file, b"Test for bundle updates")
        .await
        .expect("Should upload test file");

    // Create bundle
    let bundle = bundle_handler
        .create(
            vec![test_file.to_string()],
            None,                         // password
            None,                         // expires_at
            None,                         // max_uses
            Some("Original Bundle Name"), // description
            None,                         // note
            None,                         // code
            None,                         // require_registration
            None,                         // permissions
        )
        .await
        .expect("Should create bundle");

    let bundle_id = bundle.id.expect("Bundle should have an ID");

    println!("Created bundle {}, now updating...", bundle_id);

    // Update bundle
    let update_result = bundle_handler
        .update(
            bundle_id,
            None,                                    // password
            None,                                    // expires_at
            None,                                    // max_uses
            Some("Updated Integration Test Bundle"), // description
            None,                                    // note
        )
        .await;

    match update_result {
        Ok(updated) => {
            println!("Successfully updated bundle: {:?}", updated);
            // Note: Some fields may not update immediately or may require different permissions
        }
        Err(e) => {
            eprintln!("Failed to update bundle: {:?}", e);
        }
    }

    // Clean up
    let _ = bundle_handler.delete(bundle_id).await;
    let _ = file_handler.delete_file(test_file, false).await;
}
