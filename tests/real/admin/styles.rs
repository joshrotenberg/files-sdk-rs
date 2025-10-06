//! Real API integration tests for StyleHandler

use crate::real::*;
use files_sdk::StyleHandler;

#[tokio::test]
async fn test_real_api_get_style() {
    let client = get_test_client();
    let handler = StyleHandler::new(client);

    // Try to get the site style
    let result = handler.get("/").await;

    match result {
        Ok(style) => {
            println!("Retrieved style: {:?}", style);
        }
        Err(e) => {
            println!("Get failed: {:?}", e);
        }
    }
}
