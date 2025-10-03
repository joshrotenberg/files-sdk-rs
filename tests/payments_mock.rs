//! Mock tests for PaymentHandler

use files_sdk::{FilesClient, PaymentHandler};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, path_regex},
};

#[tokio::test]
async fn test_list_payments() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payments"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 1,
                "amount": 99.99,
                "balance": 0.0,
                "created_at": "2024-01-05T00:00:00Z",
                "currency": "USD",
                "download_uri": "https://files.com/payments/1.pdf",
                "payment_line_items": [
                    {
                        "id": 100,
                        "amount": 99.99,
                        "created_at": "2024-01-05T00:00:00Z",
                        "invoice_id": 1,
                        "payment_id": 1
                    }
                ]
            },
            {
                "id": 2,
                "amount": 149.99,
                "balance": 0.0,
                "created_at": "2024-02-05T00:00:00Z",
                "currency": "USD",
                "payment_line_items": []
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PaymentHandler::new(client);
    let (payments, _) = handler.list(None, None).await.unwrap();

    assert_eq!(payments.len(), 2);
    assert_eq!(payments[0].id, Some(1));
    assert_eq!(payments[0].amount, Some(99.99));
    assert_eq!(payments[0].currency, Some("USD".to_string()));

    let line_items = payments[0].payment_line_items.as_ref().unwrap();
    assert_eq!(line_items.len(), 1);
    assert_eq!(line_items[0].invoice_id, Some(1));
}

#[tokio::test]
async fn test_list_payments_with_pagination() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payments"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 3,
                "amount": 199.99,
                "balance": 0.0,
                "created_at": "2024-03-05T00:00:00Z",
                "currency": "USD"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PaymentHandler::new(client);
    let (payments, _) = handler.list(Some("cursor456"), Some(10)).await.unwrap();

    assert_eq!(payments.len(), 1);
    assert_eq!(payments[0].id, Some(3));
}

#[tokio::test]
async fn test_get_payment() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payments/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "amount": 99.99,
            "balance": 0.0,
            "created_at": "2024-01-05T00:00:00Z",
            "currency": "USD",
            "download_uri": "https://files.com/payments/1.pdf",
            "payment_line_items": [
                {
                    "id": 100,
                    "amount": 99.99,
                    "created_at": "2024-01-05T00:00:00Z",
                    "invoice_id": 1,
                    "payment_id": 1
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PaymentHandler::new(client);
    let payment = handler.get(1).await.unwrap();

    assert_eq!(payment.id, Some(1));
    assert_eq!(payment.amount, Some(99.99));
    assert_eq!(
        payment.download_uri,
        Some("https://files.com/payments/1.pdf".to_string())
    );

    let line_items = payment.payment_line_items.unwrap();
    assert_eq!(line_items.len(), 1);
    assert_eq!(line_items[0].invoice_id, Some(1));
}

#[tokio::test]
async fn test_get_payment_multiple_invoices() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payments/2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 2,
            "amount": 500.00,
            "balance": 0.0,
            "created_at": "2024-01-15T00:00:00Z",
            "currency": "USD",
            "payment_line_items": [
                {
                    "id": 200,
                    "amount": 300.00,
                    "created_at": "2024-01-15T00:00:00Z",
                    "invoice_id": 10,
                    "payment_id": 2
                },
                {
                    "id": 201,
                    "amount": 200.00,
                    "created_at": "2024-01-15T00:00:00Z",
                    "invoice_id": 11,
                    "payment_id": 2
                }
            ]
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PaymentHandler::new(client);
    let payment = handler.get(2).await.unwrap();

    let line_items = payment.payment_line_items.unwrap();
    assert_eq!(line_items.len(), 2);
    assert_eq!(line_items[0].invoice_id, Some(10));
    assert_eq!(line_items[1].invoice_id, Some(11));
}

#[tokio::test]
async fn test_get_payment_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/payments/\d+$"))
        .respond_with(ResponseTemplate::new(404).set_body_json(serde_json::json!({
            "error": "Not Found",
            "http-code": 404
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PaymentHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_payments_authentication_failed() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/payments"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "error": "Authentication failed",
            "http-code": 401
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("invalid-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PaymentHandler::new(client);
    let result = handler.list(None, None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_payment_forbidden() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/payments/\d+$"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "error": "Forbidden - Insufficient permissions",
            "http-code": 403
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("limited-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PaymentHandler::new(client);
    let result = handler.get(1).await;

    assert!(result.is_err());
}
