//! Real API integration tests for BehaviorHandler

use crate::real::*;
use files_sdk::{BehaviorHandler, FolderHandler};
use serde_json::json;

#[tokio::test]
async fn test_real_api_list_behaviors() {
    let client = get_test_client();
    let handler = BehaviorHandler::new(client);

    let result = handler.list(None, Some(10), None, None, None).await;

    match result {
        Ok((behaviors, pagination)) => {
            println!("Listed {} behaviors", behaviors.len());
            println!("Pagination: {:?}", pagination);
        }
        Err(e) => {
            println!("Behaviors list failed: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_create_and_delete_behavior() {
    let client = get_test_client();
    let behavior_handler = BehaviorHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client);

    let test_folder = "/integration-tests";
    let _ = folder_handler.create_folder(test_folder, true).await;

    let result = behavior_handler
        .create(
            test_folder,
            "webhook",
            Some(json!({"url": "https://example.com/webhook"})),
            None,
            None,
        )
        .await;

    match result {
        Ok(behavior) => {
            println!("Created behavior: {:?}", behavior);
            let behavior_id = behavior.id.expect("Behavior should have ID");
            let _ = behavior_handler.delete(behavior_id).await;
        }
        Err(e) => {
            println!("Behavior creation failed: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_list_behaviors_for_folder() {
    let client = get_test_client();
    let behavior_handler = BehaviorHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client);

    let test_folder = "/integration-tests";
    let _ = folder_handler.create_folder(test_folder, true).await;

    let result = behavior_handler
        .list_for_folder(test_folder, None, None)
        .await;

    match result {
        Ok((behaviors, pagination)) => {
            println!("Folder behaviors count: {}", behaviors.len());
            println!("Pagination: {:?}", pagination);
        }
        Err(e) => {
            println!("Folder behaviors list failed: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_update_behavior() {
    let client = get_test_client();
    let behavior_handler = BehaviorHandler::new(client.clone());
    let folder_handler = FolderHandler::new(client);

    let test_folder = "/integration-tests";
    let _ = folder_handler.create_folder(test_folder, true).await;

    let behavior = match behavior_handler
        .create(
            test_folder,
            "webhook",
            Some(json!({"url": "https://example.com/original"})),
            None,
            None,
        )
        .await
    {
        Ok(b) => b,
        Err(e) => {
            println!("Behavior creation failed: {:?}", e);
            return;
        }
    };

    let behavior_id = behavior.id.expect("Behavior should have ID");

    let update_result = behavior_handler
        .update(behavior_id, None, Some("Updated webhook behavior"), None)
        .await;

    match update_result {
        Ok(updated) => {
            println!("Updated behavior: {:?}", updated);
        }
        Err(e) => {
            println!("Behavior update failed: {:?}", e);
        }
    }

    let _ = behavior_handler.delete(behavior_id).await;
}
