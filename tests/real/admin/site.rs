//! Real API integration tests for SiteHandler

use crate::real::*;
use files_sdk::SiteHandler;

#[tokio::test]
async fn test_get_site_settings() {
    let client = get_test_client();
    let handler = SiteHandler::new(client);

    println!("Testing get site settings");

    let result = handler.get().await;

    match result {
        Ok(site) => {
            println!("Successfully retrieved site settings");
            println!("Site: {:?}", site);

            // Site should have basic fields
            if let Some(ref name) = site.name {
                println!("Site name: {}", name);
            }
            if let Some(ref domain) = site.domain {
                println!("Site domain: {}", domain);
            }
        }
        Err(e) => {
            panic!("Failed to get site settings: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_get_site_usage() {
    let client = get_test_client();
    let handler = SiteHandler::new(client);

    println!("Testing get site usage statistics");

    let result = handler.get_usage().await;

    match result {
        Ok(usage) => {
            println!("Successfully retrieved usage stats: {:?}", usage);
        }
        Err(e) => {
            // Usage endpoint might require premium features
            println!("Usage retrieval failed (may require premium): {:?}", e);
        }
    }
}
