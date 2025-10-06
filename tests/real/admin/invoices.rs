//! Real API integration tests for InvoiceHandler

use crate::real::*;
use files_sdk::InvoiceHandler;

#[tokio::test]
async fn test_real_api_list_invoices() {
    let client = get_test_client();
    let handler = InvoiceHandler::new(client);

    let result = handler.list(None, None).await;

    match result {
        Ok((invoices, pagination)) => {
            println!("Listed {} invoices", invoices.len());
            println!("Pagination: {:?}", pagination);
            if let Some(first) = invoices.first() {
                println!("Sample invoice: {:?}", first);
            }
        }
        Err(e) => {
            println!("List failed (may require billing): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_get_invoice() {
    let client = get_test_client();
    let handler = InvoiceHandler::new(client);

    let list_result = handler.list(None, None).await;

    match list_result {
        Ok((invoices, _)) =>
        {
            #[allow(clippy::collapsible_if)]
            if let Some(invoice) = invoices.first() {
                if let Some(id) = invoice.id {
                    let get_result = handler.get(id).await;
                    match get_result {
                        Ok(retrieved) => {
                            println!("Retrieved invoice: {:?}", retrieved);
                            assert_eq!(retrieved.id, Some(id));
                        }
                        Err(e) => {
                            println!("Failed to get invoice: {:?}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Could not list invoices: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_invoices_pagination() {
    let client = get_test_client();
    let handler = InvoiceHandler::new(client);

    let result = handler.list(None, Some(1)).await;

    match result {
        Ok((invoices, pagination)) => {
            println!("First page: {} invoices", invoices.len());

            if let Some(cursor) = pagination.cursor_next {
                let result2 = handler.list(Some(&cursor), Some(1)).await;
                match result2 {
                    Ok((invoices2, _)) => {
                        println!("Second page: {} invoices", invoices2.len());
                    }
                    Err(e) => {
                        println!("Second page fetch failed: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Invoices not available: {:?}", e);
        }
    }
}
