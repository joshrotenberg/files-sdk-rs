use files_sdk::{FileHandler, FilesClient};
use wiremock::matchers::{header, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_download_file_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "path": "/documents/report.pdf",
        "display_name": "report.pdf",
        "type": "file",
        "size": 1024,
        "download_uri": "https://files.example.com/download/report.pdf"
    });

    Mock::given(method("GET"))
        .and(path("/files/documents/report.pdf"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FileHandler::new(client);
    let file = handler
        .download_file("/documents/report.pdf")
        .await
        .unwrap();

    assert_eq!(file.path.as_deref(), Some("/documents/report.pdf"));
    assert_eq!(file.display_name.as_deref(), Some("report.pdf"));
    assert_eq!(file.size, Some(1024));
}

#[tokio::test]
async fn test_download_file_not_found() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Not Found",
        "message": "File not found"
    });

    Mock::given(method("GET"))
        .and(path("/files/nonexistent.txt"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FileHandler::new(client);
    let result = handler.download_file("/nonexistent.txt").await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::NotFound { .. }) => {}
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_update_file_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "path": "/documents/report.pdf",
        "display_name": "monthly_report.pdf",
        "type": "file",
        "size": 1024,
    });

    Mock::given(method("PATCH"))
        .and(path("/files/documents/report.pdf"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FileHandler::new(client);
    let file = handler
        .update_file(
            "/documents/report.pdf",
            None,
            Some("monthly_report.pdf".to_string()),
            None,
        )
        .await
        .unwrap();

    assert_eq!(file.path.as_deref(), Some("/documents/report.pdf"));
    assert_eq!(file.display_name.as_deref(), Some("monthly_report.pdf"));
}

#[tokio::test]
async fn test_delete_file_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/files/documents/old_report.pdf"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FileHandler::new(client);
    let result = handler
        .delete_file("/documents/old_report.pdf", false)
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_copy_file_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "path": "/backup/report.pdf",
        "display_name": "report.pdf",
        "type": "file",
        "size": 1024,
    });

    Mock::given(method("POST"))
        .and(path("/file_actions/copy/documents/report.pdf"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FileHandler::new(client);
    let result = handler
        .copy_file("/documents/report.pdf", "/backup/report.pdf")
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_move_file_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "path": "/archive/report.pdf",
        "display_name": "report.pdf",
        "type": "file",
        "size": 1024,
    });

    Mock::given(method("POST"))
        .and(path("/file_actions/move/documents/report.pdf"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FileHandler::new(client);
    let result = handler
        .move_file("/documents/report.pdf", "/archive/report.pdf")
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_file_forbidden() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Forbidden",
        "message": "You don't have permission to delete this file"
    });

    Mock::given(method("DELETE"))
        .and(path("/files/protected/file.txt"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(403).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FileHandler::new(client);
    let result = handler.delete_file("/protected/file.txt", false).await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::Forbidden { .. }) => {}
        _ => panic!("Expected Forbidden error"),
    }
}

#[tokio::test]
async fn test_copy_file_bad_request() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Bad Request",
        "message": "Destination path is invalid"
    });

    Mock::given(method("POST"))
        .and(path("/file_actions/copy/source.txt"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(400).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = FileHandler::new(client);
    let result = handler.copy_file("/source.txt", "").await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::BadRequest { .. }) => {}
        _ => panic!("Expected BadRequest error"),
    }
}
