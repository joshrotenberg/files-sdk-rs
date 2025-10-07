//! Real API integration tests for AppHandler

use crate::real::*;
use files_sdk::AppHandler;

#[cfg(feature = "integration-tests")]
#[tokio::test]
async fn test_real_api_list_apps() {
    let client = get_test_client();
    let handler = AppHandler::new(client);

    println!("Testing app listing");

    let result = handler.list().await;

    match result {
        Ok(apps) => {
            println!("Successfully listed {} apps", apps.len());

            if !apps.is_empty() {
                let first = &apps[0];
                println!(
                    "First app fields: {:?}",
                    first.data.keys().collect::<Vec<_>>()
                );
            } else {
                println!("No apps available");
            }
        }
        Err(e) => {
            println!("App listing failed: {:?}", e);
        }
    }
}
