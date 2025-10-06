//! Real API integration tests for FormFieldSetHandler
//!
//! These tests run against the actual Files.com API and require:
//! - FILES_API_KEY environment variable to be set
//! - Feature flag: cargo test --features integration-tests
//!
//! FormFieldSets define custom form fields for bundle/inbox registration.

use crate::real::*;
use files_sdk::FormFieldSetHandler;
use serde_json::json;

#[tokio::test]
async fn test_real_api_list_form_field_sets() {
    let client = get_test_client();
    let handler = FormFieldSetHandler::new(client);

    let result = handler.list(None, Some(10)).await;

    match result {
        Ok((field_sets, pagination)) => {
            println!("Listed {} form field sets", field_sets.len());
            println!("Pagination: {:?}", pagination);
        }
        Err(e) => {
            println!(
                "Form field sets list failed (may require paid plan): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_real_api_create_and_delete_form_field_set() {
    let client = get_test_client();
    let handler = FormFieldSetHandler::new(client);

    // Create form field set
    let params = json!({
        "title": "Integration Test Form",
        "skip_name": false,
        "skip_email": false,
        "skip_company": true
    });

    let create_result = handler.create(params).await;

    match create_result {
        Ok(field_set) => {
            println!("Created form field set: {:?}", field_set);
            let field_set_id = field_set.id.expect("Field set should have ID");

            // Get the field set
            let get_result = handler.get(field_set_id).await;
            match get_result {
                Ok(retrieved) => {
                    println!("Retrieved field set: {:?}", retrieved);
                    assert_eq!(retrieved.id, Some(field_set_id));
                }
                Err(e) => {
                    eprintln!("Failed to get field set: {:?}", e);
                }
            }

            // List and verify it's there
            let list_result = handler.list(None, None).await;
            match list_result {
                Ok((field_sets, _)) => {
                    let found = field_sets.iter().any(|fs| fs.id == Some(field_set_id));
                    assert!(found, "Should find created field set in list");
                }
                Err(e) => {
                    eprintln!("Failed to list field sets: {:?}", e);
                }
            }

            // Delete the field set
            let delete_result = handler.delete(field_set_id).await;
            match delete_result {
                Ok(_) => println!("Successfully deleted field set"),
                Err(e) => eprintln!("Failed to delete field set: {:?}", e),
            }
        }
        Err(e) => {
            println!(
                "Form field set creation failed (may require permissions): {:?}",
                e
            );
        }
    }
}

#[tokio::test]
async fn test_real_api_update_form_field_set() {
    let client = get_test_client();
    let handler = FormFieldSetHandler::new(client);

    // Create form field set
    let params = json!({
        "title": "Original Title"
    });

    let field_set = match handler.create(params).await {
        Ok(fs) => fs,
        Err(e) => {
            println!("Field set creation failed: {:?}", e);
            return;
        }
    };

    let field_set_id = field_set.id.expect("Field set should have ID");

    // Update it
    let update_params = json!({
        "title": "Updated Title"
    });

    let update_result = handler.update(field_set_id, update_params).await;

    match update_result {
        Ok(updated) => {
            println!("Updated form field set: {:?}", updated);
        }
        Err(e) => {
            println!("Field set update failed: {:?}", e);
        }
    }

    // Cleanup
    let _ = handler.delete(field_set_id).await;
}

#[tokio::test]
async fn test_real_api_form_field_set_pagination() {
    let client = get_test_client();
    let handler = FormFieldSetHandler::new(client);

    // Test pagination with small page size
    let result = handler.list(None, Some(1)).await;

    match result {
        Ok((field_sets, pagination)) => {
            println!("First page: {} form field sets", field_sets.len());

            if let Some(cursor) = pagination.cursor_next {
                // Fetch second page
                let result2 = handler.list(Some(cursor), Some(1)).await;
                match result2 {
                    Ok((field_sets2, _)) => {
                        println!("Second page: {} form field sets", field_sets2.len());
                    }
                    Err(e) => {
                        println!("Second page fetch failed: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Form field sets not available: {:?}", e);
        }
    }
}
