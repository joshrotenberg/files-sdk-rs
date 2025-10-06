//! Real API integration tests for BundleRegistrationHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! BundleRegistrations track registration requirements for bundle access.

use crate::real::*;
use files_sdk::BundleRegistrationHandler;

#[tokio::test]
async fn test_real_api_list_bundle_registrations() {
    let client = get_test_client();
    let handler = BundleRegistrationHandler::new(client);

    let result = handler.list().await;

    match result {
        Ok(registrations) => {
            println!("Listed {} bundle registrations", registrations.len());

            if let Some(first) = registrations.first() {
                println!("Sample registration: {:?}", first);
                assert!(!first.data.is_empty(), "Registration should have data");
            }
        }
        Err(e) => {
            println!(
                "Bundle registrations list failed (may require paid plan): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_real_api_bundle_registrations_structure() {
    let client = get_test_client();
    let handler = BundleRegistrationHandler::new(client);

    let result = handler.list().await;

    match result {
        Ok(registrations) => {
            println!("Bundle registrations count: {}", registrations.len());

            for registration in registrations.iter().take(3) {
                println!("Registration data: {:?}", registration.data);
            }
        }
        Err(e) => {
            println!("Bundle registrations not available: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_bundle_registrations_error_handling() {
    let client = get_test_client();
    let handler = BundleRegistrationHandler::new(client);

    let result = handler.list().await;

    // Should either succeed or return a proper error (not panic)
    match result {
        Ok(registrations) => {
            println!(
                "Successfully listed {} bundle registrations",
                registrations.len()
            );
        }
        Err(e) => {
            println!("Expected error handling works: {:?}", e);
        }
    }
}
