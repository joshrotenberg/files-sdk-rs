//! Mock tests for HistoryHandler

use files_sdk::{FilesClient, HistoryHandler};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn test_list_history_for_file() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/history/files/document.pdf"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 1,
                "action": "read",
                "when": "2024-01-15T10:30:00Z",
                "user_id": 123,
                "username": "john",
                "path": "/document.pdf"
            },
            {
                "id": 2,
                "action": "update",
                "when": "2024-01-15T11:00:00Z",
                "user_id": 456,
                "username": "jane",
                "path": "/document.pdf"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = HistoryHandler::new(client);
    let (history, _) = handler
        .list_for_file("document.pdf", None, None)
        .await
        .unwrap();

    assert_eq!(history.len(), 2);
    assert_eq!(history[0]["action"], "read");
    assert_eq!(history[1]["action"], "update");
}

#[tokio::test]
async fn test_list_history_for_folder() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/history/folders/uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 10,
                "action": "create",
                "when": "2024-01-15T09:00:00Z",
                "user_id": 123,
                "path": "/uploads/file1.txt"
            },
            {
                "id": 11,
                "action": "create",
                "when": "2024-01-15T09:15:00Z",
                "user_id": 123,
                "path": "/uploads/file2.txt"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = HistoryHandler::new(client);
    let (history, _) = handler
        .list_for_folder("uploads", None, None)
        .await
        .unwrap();

    assert_eq!(history.len(), 2);
    assert_eq!(history[0]["action"], "create");
}

#[tokio::test]
async fn test_list_history_for_user() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/history/users/123"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 20,
                "action": "login",
                "when": "2024-01-15T08:00:00Z",
                "user_id": 123,
                "username": "john",
                "ip": "192.168.1.100"
            },
            {
                "id": 21,
                "action": "read",
                "when": "2024-01-15T08:05:00Z",
                "user_id": 123,
                "path": "/reports/data.csv"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = HistoryHandler::new(client);
    let (history, _) = handler.list_for_user(123, None, None).await.unwrap();

    assert_eq!(history.len(), 2);
    assert_eq!(history[0]["action"], "login");
    assert_eq!(history[0]["ip"], "192.168.1.100");
}

#[tokio::test]
async fn test_list_login_history() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/history/login"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 30,
                "action": "login",
                "when": "2024-01-15T07:00:00Z",
                "user_id": 123,
                "username": "john",
                "ip": "192.168.1.100",
                "success": true
            },
            {
                "id": 31,
                "action": "failedlogin",
                "when": "2024-01-15T07:05:00Z",
                "username": "hacker",
                "ip": "10.0.0.1",
                "success": false,
                "failure_reason": "bad_password"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = HistoryHandler::new(client);
    let (history, _) = handler.list_logins(None, None).await.unwrap();

    assert_eq!(history.len(), 2);
    assert_eq!(history[0]["success"], true);
    assert_eq!(history[1]["success"], false);
}

#[tokio::test]
async fn test_create_history_export() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/history_exports"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 100,
            "status": "building",
            "start_at": "2024-01-01T00:00:00Z",
            "end_at": "2024-01-31T23:59:59Z",
            "query_action": "create,update,delete",
            "query_folder": "/uploads"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = HistoryHandler::new(client);
    let export = handler
        .create_export(
            Some("2024-01-01T00:00:00Z"),
            Some("2024-01-31T23:59:59Z"),
            Some("create,update,delete"),
            None,
            Some("/uploads"),
        )
        .await
        .unwrap();

    assert_eq!(export.id, Some(100));
    assert_eq!(export.status, Some("building".to_string()));
    assert_eq!(export.query_folder, Some("/uploads".to_string()));
}

#[tokio::test]
async fn test_get_history_export() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/history_exports/100"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 100,
            "status": "ready",
            "start_at": "2024-01-01T00:00:00Z",
            "end_at": "2024-01-31T23:59:59Z",
            "results_url": "https://files.com/history_exports/100.csv"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = HistoryHandler::new(client);
    let export = handler.get_export(100).await.unwrap();

    assert_eq!(export.id, Some(100));
    assert_eq!(export.status, Some("ready".to_string()));
    assert!(export.results_url.is_some());
}

#[tokio::test]
async fn test_get_history_export_results() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/history_export_results"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "history_export_id": 100,
                "result": {
                    "action": "create",
                    "path": "/uploads/file1.txt",
                    "when": "2024-01-15T10:00:00Z"
                }
            },
            {
                "history_export_id": 100,
                "result": {
                    "action": "delete",
                    "path": "/uploads/old.txt",
                    "when": "2024-01-16T11:00:00Z"
                }
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = HistoryHandler::new(client);
    let (results, _) = handler
        .get_export_results(None, None, Some(100))
        .await
        .unwrap();

    assert_eq!(results.len(), 2);
    assert_eq!(results[0].history_export_id, Some(100));
}

#[tokio::test]
async fn test_create_export_user_filter() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/history_exports"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 101,
            "status": "building",
            "query_user_id": "123",
            "query_action": "login,failedlogin"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = HistoryHandler::new(client);
    let export = handler
        .create_export(None, None, Some("login,failedlogin"), Some("123"), None)
        .await
        .unwrap();

    assert_eq!(export.query_user_id, Some("123".to_string()));
    assert_eq!(export.query_action, Some("login,failedlogin".to_string()));
}

#[tokio::test]
async fn test_export_failed_status() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/history_exports/102"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 102,
            "status": "failed",
            "start_at": "2024-01-01T00:00:00Z",
            "end_at": "2024-01-31T23:59:59Z"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = HistoryHandler::new(client);
    let export = handler.get_export(102).await.unwrap();

    assert_eq!(export.status, Some("failed".to_string()));
}

#[tokio::test]
async fn test_list_history_authentication_failed() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/history/login"))
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

    let handler = HistoryHandler::new(client);
    let result = handler.list_logins(None, None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_export_forbidden() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/history_exports"))
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

    let handler = HistoryHandler::new(client);
    let result = handler.create_export(None, None, None, None, None).await;

    assert!(result.is_err());
}
