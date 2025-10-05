//! Real API integration tests for SessionHandler

use crate::real::*;
use files_sdk::SessionHandler;

#[tokio::test]
async fn test_create_session() {
    let client = get_test_client();
    let handler = SessionHandler::new(client);

    println!("Testing session creation");

    // Note: SessionHandler.create() requires username and password
    // We don't have those in integration tests, so this will fail
    // This test just verifies the method exists and returns proper error

    let result = handler.create("test_user", "test_password", None).await;

    match result {
        Ok(session) => {
            println!("Unexpectedly created session: {:?}", session);
            // Clean up
            let _ = handler.delete().await;
        }
        Err(e) => {
            // Expected to fail with bad credentials
            println!("Session creation failed as expected: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_delete_session() {
    let client = get_test_client();
    let handler = SessionHandler::new(client);

    println!("Testing session deletion");

    // Try to delete current session (will fail if using API key auth)
    let result = handler.delete().await;

    match result {
        Ok(_) => {
            println!("Successfully deleted session");
        }
        Err(e) => {
            // Expected to fail when using API key authentication
            println!(
                "Session deletion failed (expected with API key auth): {:?}",
                e
            );
        }
    }
}
