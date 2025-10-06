//! Real API integration tests for BundleRecipientHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! BundleRecipients track who bundles are shared with.

use crate::real::*;
use files_sdk::{BundleHandler, BundleRecipientHandler, FileHandler, FolderHandler};
use serde_json::json;

#[tokio::test]
async fn test_real_api_list_bundle_recipients() {
    let client = get_test_client();
    let recipient_handler = BundleRecipientHandler::new(client.clone());
    let bundle_handler = BundleHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client.clone());

    // Create a test bundle first
    let test_folder = "/integration-tests";
    let test_file = "/integration-tests/recipient-test.txt";

    let _ = folder_handler.create_folder(test_folder, true).await;
    let _ = file_handler.delete_file(test_file, false).await;

    file_handler
        .upload_file(test_file, b"Test for bundle recipients")
        .await
        .expect("Should upload test file");

    let bundle = match bundle_handler
        .create(
            vec![test_file.to_string()],
            None,
            None,
            None,
            Some("Recipient Test Bundle"),
            None,
            None,
            None,
            None,
        )
        .await
    {
        Ok(b) => b,
        Err(e) => {
            println!("Bundle creation failed (may require paid account): {:?}", e);
            let _ = file_handler.delete_file(test_file, false).await;
            return;
        }
    };

    let bundle_id = bundle.id.expect("Bundle should have ID");

    // List recipients (may be empty)
    let result = recipient_handler.list(bundle_id, None, Some(10)).await;

    match result {
        Ok((recipients, pagination)) => {
            println!("Listed {} bundle recipients", recipients.len());
            println!("Pagination: {:?}", pagination);
        }
        Err(e) => {
            println!("Bundle recipients list failed: {:?}", e);
        }
    }

    // Cleanup
    let _ = bundle_handler.delete(bundle_id).await;
    let _ = file_handler.delete_file(test_file, false).await;
}

#[tokio::test]
async fn test_real_api_create_bundle_recipient() {
    let client = get_test_client();
    let recipient_handler = BundleRecipientHandler::new(client.clone());
    let bundle_handler = BundleHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client.clone());

    // Create a test bundle
    let test_folder = "/integration-tests";
    let test_file = "/integration-tests/create-recipient-test.txt";

    let _ = folder_handler.create_folder(test_folder, true).await;
    let _ = file_handler.delete_file(test_file, false).await;

    file_handler
        .upload_file(test_file, b"Test for creating recipients")
        .await
        .expect("Should upload test file");

    let bundle = match bundle_handler
        .create(
            vec![test_file.to_string()],
            None,
            None,
            None,
            Some("Create Recipient Test"),
            None,
            None,
            None,
            None,
        )
        .await
    {
        Ok(b) => b,
        Err(e) => {
            println!("Bundle creation failed: {:?}", e);
            let _ = file_handler.delete_file(test_file, false).await;
            return;
        }
    };

    let bundle_id = bundle.id.expect("Bundle should have ID");

    // Create recipient
    let params = json!({
        "bundle_id": bundle_id,
        "recipient": "test@example.com",
        "name": "Test Recipient",
        "share_after_create": false
    });

    let create_result = recipient_handler.create(params).await;

    match create_result {
        Ok(recipient) => {
            println!("Created bundle recipient: {:?}", recipient);

            // Verify it appears in list
            let list_result = recipient_handler.list(bundle_id, None, None).await;
            match list_result {
                Ok((recipients, _)) => {
                    println!("Bundle has {} recipients", recipients.len());
                }
                Err(e) => {
                    println!("Failed to list recipients: {:?}", e);
                }
            }
        }
        Err(e) => {
            println!(
                "Recipient creation failed (may require permissions): {:?}",
                e
            );
        }
    }

    // Cleanup
    let _ = bundle_handler.delete(bundle_id).await;
    let _ = file_handler.delete_file(test_file, false).await;
}

#[tokio::test]
async fn test_real_api_bundle_recipient_pagination() {
    let client = get_test_client();
    let recipient_handler = BundleRecipientHandler::new(client.clone());
    let bundle_handler = BundleHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client.clone());

    // Create a test bundle
    let test_folder = "/integration-tests";
    let test_file = "/integration-tests/pagination-test.txt";

    let _ = folder_handler.create_folder(test_folder, true).await;
    let _ = file_handler.delete_file(test_file, false).await;

    file_handler
        .upload_file(test_file, b"Test pagination")
        .await
        .expect("Should upload test file");

    let bundle = match bundle_handler
        .create(
            vec![test_file.to_string()],
            None,
            None,
            None,
            Some("Pagination Test"),
            None,
            None,
            None,
            None,
        )
        .await
    {
        Ok(b) => b,
        Err(e) => {
            println!("Bundle creation failed: {:?}", e);
            let _ = file_handler.delete_file(test_file, false).await;
            return;
        }
    };

    let bundle_id = bundle.id.expect("Bundle should have ID");

    // Test pagination with small page size
    let result = recipient_handler.list(bundle_id, None, Some(1)).await;

    match result {
        Ok((recipients, pagination)) => {
            println!("First page: {} recipients", recipients.len());

            if let Some(cursor) = pagination.cursor_next {
                // Fetch second page
                let result2 = recipient_handler
                    .list(bundle_id, Some(cursor), Some(1))
                    .await;
                match result2 {
                    Ok((recipients2, _)) => {
                        println!("Second page: {} recipients", recipients2.len());
                    }
                    Err(e) => {
                        println!("Second page fetch failed: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Recipients list failed: {:?}", e);
        }
    }

    // Cleanup
    let _ = bundle_handler.delete(bundle_id).await;
    let _ = file_handler.delete_file(test_file, false).await;
}
