//! Real API integration tests for PaymentHandler

use crate::real::*;
use files_sdk::PaymentHandler;

#[tokio::test]
async fn test_real_api_list_payments() {
    let client = get_test_client();
    let handler = PaymentHandler::new(client);

    let result = handler.list(None, None).await;

    match result {
        Ok((payments, pagination)) => {
            println!("Listed {} payments", payments.len());
            println!("Pagination: {:?}", pagination);
            if let Some(first) = payments.first() {
                println!("Sample payment: {:?}", first);
            }
        }
        Err(e) => {
            println!("List failed (may require billing): {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_get_payment() {
    let client = get_test_client();
    let handler = PaymentHandler::new(client);

    let list_result = handler.list(None, None).await;

    match list_result {
        Ok((payments, _)) =>
        {
            #[allow(clippy::collapsible_if)]
            if let Some(payment) = payments.first() {
                if let Some(id) = payment.id {
                    let get_result = handler.get(id).await;
                    match get_result {
                        Ok(retrieved) => {
                            println!("Retrieved payment: {:?}", retrieved);
                            assert_eq!(retrieved.id, Some(id));
                        }
                        Err(e) => {
                            println!("Failed to get payment: {:?}", e);
                        }
                    }
                }
            }
        }
        Err(e) => {
            println!("Could not list payments: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_real_api_payments_pagination() {
    let client = get_test_client();
    let handler = PaymentHandler::new(client);

    let result = handler.list(None, Some(1)).await;

    match result {
        Ok((payments, pagination)) => {
            println!("First page: {} payments", payments.len());

            if let Some(cursor) = pagination.cursor_next {
                let result2 = handler.list(Some(&cursor), Some(1)).await;
                match result2 {
                    Ok((payments2, _)) => {
                        println!("Second page: {} payments", payments2.len());
                    }
                    Err(e) => {
                        println!("Second page fetch failed: {:?}", e);
                    }
                }
            }
        }
        Err(e) => {
            println!("Payments not available: {:?}", e);
        }
    }
}
