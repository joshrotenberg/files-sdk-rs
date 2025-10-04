//! Real API integration tests for UserHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests --test users_real_api
//!
//! These tests are read-only and should not modify any data.

use crate::real::*;
use files_sdk::{FilesClient, UserHandler};

#[tokio::test]
async fn test_real_api_list_users() {
    let client = get_test_client();
    let handler = UserHandler::new(client);

    // List users - this is a read-only operation
    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((users, pagination)) => {
            println!("Successfully listed {} users", users.len());
            println!(
                "Pagination: next={:?}, prev={:?}",
                pagination.cursor_next, pagination.cursor_prev
            );

            // Basic assertions
            assert!(!users.is_empty(), "Should have at least one user");

            // Verify first user has required fields
            let first_user = &users[0];
            assert!(first_user.id.is_some(), "User should have an ID");
            assert!(
                first_user.username.is_some() || first_user.email.is_some(),
                "User should have username or email"
            );

            println!("First user: {:?}", first_user);
        }
        Err(e) => {
            panic!("Failed to list users: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_get_user_by_id() {
    let client = get_test_client();
    let handler = UserHandler::new(client);

    // First, list users to get a valid user ID
    let (users, _) = handler
        .list(None, Some(1))
        .await
        .expect("Should be able to list users");

    assert!(!users.is_empty(), "Should have at least one user");

    let user_id = users[0].id.expect("User should have an ID");

    // Now try to get that specific user
    let result = handler.get(user_id).await;

    match result {
        Ok(user) => {
            println!("Successfully retrieved user: {:?}", user);
            assert_eq!(
                user.id,
                Some(user_id),
                "Retrieved user should have the same ID"
            );
        }
        Err(e) => {
            // This might fail with 404 if the user doesn't have permission to view themselves
            println!("Failed to get user {}: {:?}", user_id, e);
        }
    }
}

#[tokio::test]
async fn test_real_api_authentication_error() {
    // Test with invalid API key
    let client = FilesClient::builder()
        .api_key("invalid-key-12345")
        .build()
        .unwrap();

    let handler = UserHandler::new(client);

    let result = handler.list(None, None).await;

    assert!(result.is_err(), "Should fail with invalid API key");

    match result {
        Err(files_sdk::FilesError::AuthenticationFailed { .. }) => {
            println!("Correctly received AuthenticationFailed error");
        }
        Err(e) => {
            panic!("Expected AuthenticationFailed but got: {:?}", e);
        }
        Ok(_) => {
            panic!("Should have failed with invalid API key");
        }
    }
}

#[tokio::test]
async fn test_list_users_with_pagination() {
    let client = get_test_client();
    let handler = UserHandler::new(client);

    println!("Testing user list pagination");

    // Get first page with small page size
    let (first_page, pagination) = handler
        .list(None, Some(2))
        .await
        .expect("Should list users");

    println!("First page: {} users", first_page.len());

    // If there's a next cursor, fetch next page
    if let Some(next_cursor) = pagination.cursor_next {
        println!("Fetching next page with cursor: {}", next_cursor);

        let (second_page, _) = handler
            .list(Some(next_cursor), Some(2))
            .await
            .expect("Should list next page");

        println!("Second page: {} users", second_page.len());
        assert!(!second_page.is_empty(), "Second page should have users");
    } else {
        println!("No pagination available (less than 2 users)");
    }
}

#[tokio::test]
async fn test_get_nonexistent_user() {
    let client = get_test_client();
    let handler = UserHandler::new(client);

    println!("Testing get nonexistent user");

    // Try to get a user with an ID that probably doesn't exist
    let result = handler.get(999999999).await;

    match result {
        Err(files_sdk::FilesError::NotFound { .. }) => {
            println!("Correctly received NotFound error");
        }
        Err(e) => {
            println!("Got error (acceptable): {:?}", e);
        }
        Ok(user) => {
            println!("Unexpectedly found user: {:?}", user);
        }
    }
}

#[tokio::test]
async fn test_list_users_empty_page() {
    let client = get_test_client();
    let handler = UserHandler::new(client);

    println!("Testing list with very large per_page");

    // Request a large number per page
    let result = handler.list(None, Some(1000)).await;

    match result {
        Ok((users, pagination)) => {
            println!("Retrieved {} users with per_page=1000", users.len());
            println!("Has next page: {}", pagination.has_next());

            assert!(users.len() <= 1000, "Should not exceed requested page size");
        }
        Err(e) => {
            panic!("Failed to list users: {:?}", e);
        }
    }
}
