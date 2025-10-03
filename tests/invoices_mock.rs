//! Mock tests for InvoiceHandler

use files_sdk::{FilesClient, InvoiceHandler};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, path_regex},
};

#[tokio::test]
async fn test_list_invoices() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/invoices"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 1,
                "amount": 99.99,
                "balance": 0.0,
                "created_at": "2024-01-01T00:00:00Z",
                "currency": "USD",
                "download_uri": "https://files.com/invoices/1.pdf",
                "invoice_line_items": [
                    {
                        "id": 10,
                        "amount": 99.99,
                        "description": "Premium Plan - January 2024",
                        "type": "invoice",
                        "plan": "Premium",
                        "site": "mycompany"
                    }
                ]
            },
            {
                "id": 2,
                "amount": 149.99,
                "balance": 149.99,
                "created_at": "2024-02-01T00:00:00Z",
                "currency": "USD",
                "invoice_line_items": []
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = InvoiceHandler::new(client);
    let (invoices, _) = handler.list(None, None).await.unwrap();

    assert_eq!(invoices.len(), 2);
    assert_eq!(invoices[0].id, Some(1));
    assert_eq!(invoices[0].amount, Some(99.99));
    assert_eq!(invoices[0].currency, Some("USD".to_string()));
    assert_eq!(invoices[0].balance, Some(0.0));

    let line_items = invoices[0].invoice_line_items.as_ref().unwrap();
    assert_eq!(line_items.len(), 1);
    assert_eq!(line_items[0].plan, Some("Premium".to_string()));
}

#[tokio::test]
async fn test_list_invoices_with_pagination() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/invoices"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 3,
                "amount": 199.99,
                "balance": 0.0,
                "created_at": "2024-03-01T00:00:00Z",
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

    let handler = InvoiceHandler::new(client);
    let (invoices, _) = handler.list(Some("cursor123"), Some(10)).await.unwrap();

    assert_eq!(invoices.len(), 1);
    assert_eq!(invoices[0].id, Some(3));
}

#[tokio::test]
async fn test_get_invoice() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/invoices/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "amount": 99.99,
            "balance": 0.0,
            "created_at": "2024-01-01T00:00:00Z",
            "currency": "USD",
            "download_uri": "https://files.com/invoices/1.pdf",
            "invoice_line_items": [
                {
                    "id": 10,
                    "amount": 79.99,
                    "created_at": "2024-01-01T00:00:00Z",
                    "description": "Premium Storage - January 2024",
                    "type": "invoice",
                    "service_start_at": "2024-01-01T00:00:00Z",
                    "service_end_at": "2024-01-31T23:59:59Z",
                    "plan": "Premium",
                    "site": "mycompany"
                },
                {
                    "id": 11,
                    "amount": 20.00,
                    "created_at": "2024-01-01T00:00:00Z",
                    "description": "Additional Transfer - January 2024",
                    "type": "invoice",
                    "plan": "Transfer Add-on"
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

    let handler = InvoiceHandler::new(client);
    let invoice = handler.get(1).await.unwrap();

    assert_eq!(invoice.id, Some(1));
    assert_eq!(invoice.amount, Some(99.99));
    assert_eq!(
        invoice.download_uri,
        Some("https://files.com/invoices/1.pdf".to_string())
    );

    let line_items = invoice.invoice_line_items.unwrap();
    assert_eq!(line_items.len(), 2);
    assert_eq!(
        line_items[0].description,
        Some("Premium Storage - January 2024".to_string())
    );
    assert_eq!(line_items[1].amount, Some(20.00));
}

#[tokio::test]
async fn test_get_invoice_paid() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/invoices/2"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 2,
            "amount": 500.00,
            "balance": 0.0,
            "created_at": "2024-01-15T00:00:00Z",
            "currency": "USD",
            "download_uri": "https://files.com/invoices/2.pdf"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = InvoiceHandler::new(client);
    let invoice = handler.get(2).await.unwrap();

    assert_eq!(invoice.balance, Some(0.0)); // Fully paid
}

#[tokio::test]
async fn test_get_invoice_unpaid() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/invoices/3"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 3,
            "amount": 299.99,
            "balance": 299.99,
            "created_at": "2024-02-01T00:00:00Z",
            "currency": "USD"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = InvoiceHandler::new(client);
    let invoice = handler.get(3).await.unwrap();

    assert_eq!(invoice.balance, Some(299.99)); // Unpaid
}

#[tokio::test]
async fn test_get_invoice_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/invoices/\d+$"))
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

    let handler = InvoiceHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_invoices_authentication_failed() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/invoices"))
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

    let handler = InvoiceHandler::new(client);
    let result = handler.list(None, None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_get_invoice_forbidden() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/invoices/\d+$"))
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

    let handler = InvoiceHandler::new(client);
    let result = handler.get(1).await;

    assert!(result.is_err());
}
