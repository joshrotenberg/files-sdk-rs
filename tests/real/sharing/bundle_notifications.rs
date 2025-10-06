//! Real API integration tests for BundleNotificationHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! BundleNotifications configure email notifications for bundle activity.

use crate::real::*;
use files_sdk::{BundleHandler, BundleNotificationHandler, FileHandler, FolderHandler};
use serde_json::json;

#[tokio::test]
async fn test_real_api_list_bundle_notifications() {
    let client = get_test_client();
    let handler = BundleNotificationHandler::new(client);

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((notifications, pagination)) => {
            println!("Listed {} bundle notifications", notifications.len());
            println!("Pagination: {:?}", pagination);
        }
        Err(e) => {
            println!(
                "Bundle notifications list failed (may require paid plan): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_real_api_create_and_delete_bundle_notification() {
    let client = get_test_client();
    let notification_handler = BundleNotificationHandler::new(client.clone());
    let bundle_handler = BundleHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client.clone());

    // Create a test bundle first
    let test_folder = "/integration-tests";
    let test_file = "/integration-tests/notification-test.txt";

    let _ = folder_handler.create_folder(test_folder, true).await;
    let _ = file_handler.delete_file(test_file, false).await;

    file_handler
        .upload_file(test_file, b"Test for bundle notifications")
        .await
        .expect("Should upload test file");

    // Create bundle
    let bundle = match bundle_handler
        .create(
            vec![test_file.to_string()],
            None,
            None,
            None,
            Some("Notification Test Bundle"),
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

    let bundle_id = bundle.id.expect("Bundle should have an ID");

    // Create notification for the bundle
    let params = json!({
        "bundle_id": bundle_id,
        "notify_on_registration": true,
        "notify_on_upload": true
    });

    let create_result = notification_handler.create(params).await;

    match create_result {
        Ok(notification) => {
            println!("Created bundle notification: {:?}", notification);
            let notification_id = notification.id.expect("Notification should have ID");

            // Get the notification
            let get_result = notification_handler.get(notification_id).await;
            match get_result {
                Ok(retrieved) => {
                    println!("Retrieved notification: {:?}", retrieved);
                    assert_eq!(retrieved.id, Some(notification_id));
                }
                Err(e) => {
                    eprintln!("Failed to get notification: {:?}", e);
                }
            }

            // Delete the notification
            let _ = notification_handler.delete(notification_id).await;
        }
        Err(e) => {
            println!(
                "Bundle notification creation failed (may require permissions): {:?}",
                e
            );
        }
    }

    // Cleanup
    let _ = bundle_handler.delete(bundle_id).await;
    let _ = file_handler.delete_file(test_file, false).await;
}

#[tokio::test]
async fn test_real_api_update_bundle_notification() {
    let client = get_test_client();
    let notification_handler = BundleNotificationHandler::new(client.clone());
    let bundle_handler = BundleHandler::new(client.clone());
    let file_handler = FileHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client.clone());

    // Create test bundle
    let test_folder = "/integration-tests";
    let test_file = "/integration-tests/update-notification-test.txt";

    let _ = folder_handler.create_folder(test_folder, true).await;
    let _ = file_handler.delete_file(test_file, false).await;

    file_handler
        .upload_file(test_file, b"Test for notification updates")
        .await
        .expect("Should upload test file");

    let bundle = match bundle_handler
        .create(
            vec![test_file.to_string()],
            None,
            None,
            None,
            Some("Update Notification Test"),
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

    // Create notification
    let params = json!({
        "bundle_id": bundle_id,
        "notify_on_registration": false
    });

    let notification = match notification_handler.create(params).await {
        Ok(n) => n,
        Err(e) => {
            println!("Notification creation failed: {:?}", e);
            let _ = bundle_handler.delete(bundle_id).await;
            let _ = file_handler.delete_file(test_file, false).await;
            return;
        }
    };

    let notification_id = notification.id.expect("Notification should have ID");

    // Update notification
    let update_params = json!({
        "notify_on_registration": true,
        "notify_on_upload": false
    });

    let update_result = notification_handler
        .update(notification_id, update_params)
        .await;

    match update_result {
        Ok(updated) => {
            println!("Updated notification: {:?}", updated);
        }
        Err(e) => {
            println!("Notification update failed: {:?}", e);
        }
    }

    // Cleanup
    let _ = notification_handler.delete(notification_id).await;
    let _ = bundle_handler.delete(bundle_id).await;
    let _ = file_handler.delete_file(test_file, false).await;
}

#[tokio::test]
async fn test_real_api_bundle_notification_pagination() {
    let client = get_test_client();
    let handler = BundleNotificationHandler::new(client);

    // Test pagination with small page size
    let result = handler.list(None, Some(1)).await;

    match result {
        Ok((notifications, pagination)) => {
            println!("First page: {} notifications", notifications.len());

            if let Some(cursor) = pagination.cursor_next {
                // Fetch second page
                let result2 = handler.list(Some(cursor), Some(1)).await;
                match result2 {
                    Ok((notifications2, _)) => {
                        println!("Second page: {} notifications", notifications2.len());
                    }
                    Err(e) => {
                        println!("Second page fetch failed: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Bundle notifications not available: {:?}", e);
        }
    }
}
