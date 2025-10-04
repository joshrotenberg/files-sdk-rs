use files_sdk::{FilesClient, RequestHandler};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_requests_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!([
        {
            "id": 1,
            "path": "/uploads",
            "destination": "monthly_report",
            "user_display_name": "John Doe",
        },
        {
            "id": 2,
            "path": "/documents",
            "destination": "invoice",
            "automation_id": 123,
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/requests"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RequestHandler::new(client);
    let (requests, _pagination) = handler.list(None, None, None, None).await.unwrap();

    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].id, Some(1));
    assert_eq!(requests[0].path.as_deref(), Some("/uploads"));
    assert_eq!(requests[0].destination.as_deref(), Some("monthly_report"));
    assert_eq!(requests[0].user_display_name.as_deref(), Some("John Doe"));
    assert_eq!(requests[1].automation_id, Some(123));
}

#[tokio::test]
async fn test_list_requests_with_filters() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!([
        {
            "id": 1,
            "path": "/uploads",
            "destination": "report",
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/requests"))
        .and(query_param("path", "/uploads"))
        .and(query_param("mine", "true"))
        .and(query_param("per_page", "10"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RequestHandler::new(client);
    let (requests, _pagination) = handler
        .list(None, Some(10), Some("/uploads"), Some(true))
        .await
        .unwrap();

    assert_eq!(requests.len(), 1);
    assert_eq!(requests[0].path.as_deref(), Some("/uploads"));
}

#[tokio::test]
async fn test_list_for_folder_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!([
        {
            "id": 1,
            "path": "/uploads",
            "destination": "file1",
        },
        {
            "id": 2,
            "path": "/uploads",
            "destination": "file2",
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/requests/folders/uploads"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RequestHandler::new(client);
    let (requests, _pagination) = handler
        .list_for_folder("uploads", None, None)
        .await
        .unwrap();

    assert_eq!(requests.len(), 2);
    assert_eq!(requests[0].path.as_deref(), Some("/uploads"));
    assert_eq!(requests[1].path.as_deref(), Some("/uploads"));
}

#[tokio::test]
async fn test_create_request_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 456,
        "path": "/uploads",
        "destination": "monthly_report",
    });

    Mock::given(method("POST"))
        .and(path("/requests"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RequestHandler::new(client);
    let request = handler
        .create("/uploads", "monthly_report", None, None)
        .await
        .unwrap();

    assert_eq!(request.id, Some(456));
    assert_eq!(request.path.as_deref(), Some("/uploads"));
    assert_eq!(request.destination.as_deref(), Some("monthly_report"));
}

#[tokio::test]
async fn test_create_request_with_users() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 789,
        "path": "/uploads",
        "destination": "report",
    });

    Mock::given(method("POST"))
        .and(path("/requests"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RequestHandler::new(client);
    let request = handler
        .create("/uploads", "report", Some("123,456"), None)
        .await
        .unwrap();

    assert_eq!(request.id, Some(789));
}

#[tokio::test]
async fn test_create_request_with_groups() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 999,
        "path": "/team/uploads",
        "destination": "team_report",
    });

    Mock::given(method("POST"))
        .and(path("/requests"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RequestHandler::new(client);
    let request = handler
        .create("/team/uploads", "team_report", None, Some("10,20"))
        .await
        .unwrap();

    assert_eq!(request.id, Some(999));
}

#[tokio::test]
async fn test_delete_request_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/requests/123"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RequestHandler::new(client);
    let result = handler.delete(123).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_request_bad_request() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Bad Request",
        "message": "Path and destination are required"
    });

    Mock::given(method("POST"))
        .and(path("/requests"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(400).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RequestHandler::new(client);
    let result = handler.create("", "", None, None).await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::BadRequest { .. }) => {}
        _ => panic!("Expected BadRequest error"),
    }
}

#[tokio::test]
async fn test_delete_request_not_found() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Not Found",
        "message": "Request not found"
    });

    Mock::given(method("DELETE"))
        .and(path("/requests/999"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RequestHandler::new(client);
    let result = handler.delete(999).await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::NotFound { .. }) => {}
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_list_for_folder_forbidden() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Forbidden",
        "message": "You don't have permission to access this folder"
    });

    Mock::given(method("GET"))
        .and(path("/requests/folders/restricted"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(403).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RequestHandler::new(client);
    let result = handler.list_for_folder("restricted", None, None).await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::Forbidden { .. }) => {}
        _ => panic!("Expected Forbidden error"),
    }
}
