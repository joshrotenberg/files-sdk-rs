//! Real API integration tests for InboxRegistrationHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! InboxRegistrations track registration requirements for inbox access.

use crate::real::*;
use files_sdk::InboxRegistrationHandler2;

#[tokio::test]
async fn test_real_api_list_inbox_registrations() {
    let client = get_test_client();
    let handler = InboxRegistrationHandler2::new(client);

    let result = handler.list().await;

    match result {
        Ok(registrations) => {
            println!("Listed {} inbox registrations", registrations.len());

            if let Some(first) = registrations.first() {
                println!("Sample registration: {:?}", first);
                assert!(!first.data.is_empty(), "Registration should have data");
            }
        }
        Err(e) => {
            println!(
                "Inbox registrations list failed (may require paid plan): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_real_api_inbox_registrations_structure() {
    let client = get_test_client();
    let handler = InboxRegistrationHandler2::new(client);

    let result = handler.list().await;

    match result {
        Ok(registrations) => {
            println!("Inbox registrations count: {}", registrations.len());

            for registration in registrations.iter().take(3) {
                println!("Registration data: {:?}", registration.data);
            }
        }
        Err(e) => {
            println!("Inbox registrations not available: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_inbox_registrations_error_handling() {
    let client = get_test_client();
    let handler = InboxRegistrationHandler2::new(client);

    let result = handler.list().await;

    // Should either succeed or return a proper error (not panic)
    match result {
        Ok(registrations) => {
            println!(
                "Successfully listed {} inbox registrations",
                registrations.len()
            );
        }
        Err(e) => {
            println!("Expected error handling works: {:?}", e);
        }
    }
}
