//! Real API integration tests for SettingsChangeHandler

use crate::real::*;
use files_sdk::SettingsChangeHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_settings_changes() {
    let client = get_test_client();
    let handler = SettingsChangeHandler::new(client);

    println!("Testing settings change listing");

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((changes, pagination)) => {
            println!("Successfully listed {} settings changes", changes.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            if !changes.is_empty() {
                let first = &changes[0];
                println!(
                    "First settings change fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No settings changes available");
            }
        }
        Err(e) => {
            println!("Settings change listing failed: {:?}", e);
        }
    }
}
