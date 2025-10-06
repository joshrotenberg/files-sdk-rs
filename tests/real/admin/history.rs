//! Real API integration tests for HistoryHandler

use crate::real::*;
use files_sdk::HistoryHandler;

#[tokio::test]
async fn test_real_api_list_history_for_folder() {
    let client = get_test_client();
    let handler = HistoryHandler::new(client);

    let result = handler
        .list_for_folder("/integration-tests", None, Some(10))
        .await;

    match result {
        Ok((entries, pagination)) => {
            println!("Listed {} history entries for folder", entries.len());
            println!("Pagination: {:?}", pagination);
        }
        Err(e) => {
            println!("Folder history list failed: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_list_history_for_file() {
    let client = get_test_client();
    let handler = HistoryHandler::new(client);

    let result = handler
        .list_for_file("/integration-tests/test.txt", None, Some(10))
        .await;

    match result {
        Ok((entries, pagination)) => {
            println!("Listed {} history entries for file", entries.len());
            println!("Pagination: {:?}", pagination);
        }
        Err(e) => {
            println!("File history list failed: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_list_logins() {
    let client = get_test_client();
    let handler = HistoryHandler::new(client);

    let result = handler.list_logins(None, Some(10)).await;

    match result {
        Ok((entries, pagination)) => {
            println!("Listed {} login history entries", entries.len());
            println!("Pagination: {:?}", pagination);
        }
        Err(e) => {
            println!("Login history list failed: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_history_pagination() {
    let client = get_test_client();
    let handler = HistoryHandler::new(client);

    let result = handler.list_for_folder("/", None, Some(1)).await;

    match result {
        Ok((entries, pagination)) => {
            println!("First page: {} entries", entries.len());

            if let Some(cursor) = pagination.cursor_next {
                let result2 = handler.list_for_folder("/", Some(&cursor), Some(1)).await;
                match result2 {
                    Ok((entries2, _)) => {
                        println!("Second page: {} entries", entries2.len());
                    }
                    Err(e) => {
                        println!("Second page fetch failed: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("History not available: {:?}", e);
        }
    }
}
