use files_sdk::{FilesClient, NotificationHandler};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_notifications_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!([
        {
            "id": 1,
            "path": "/uploads",
            "group_id": 10,
            "group_name": "Engineering",
            "notify_on_upload": true,
            "notify_on_download": false,
            "send_interval": "hourly",
            "recursive": true,
        },
        {
            "id": 2,
            "path": "/documents",
            "user_id": 123,
            "username": "johndoe",
            "notify_on_delete": true,
            "send_interval": "daily",
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/notifications"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = NotificationHandler::new(client);
    let (notifications, _) = handler.list(None, None, None, None).await.unwrap();

    assert_eq!(notifications.len(), 2);
    assert_eq!(notifications[0].id, Some(1));
    assert_eq!(notifications[0].path.as_deref(), Some("/uploads"));
    assert_eq!(notifications[0].group_id, Some(10));
    assert_eq!(notifications[0].notify_on_upload, Some(true));
    assert_eq!(notifications[0].send_interval.as_deref(), Some("hourly"));
    assert_eq!(notifications[1].notify_on_delete, Some(true));
}

#[tokio::test]
async fn test_list_notifications_with_filters() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!([
        {
            "id": 1,
            "path": "/uploads",
            "group_id": 10,
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/notifications"))
        .and(query_param("path", "/uploads"))
        .and(query_param("group_id", "10"))
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

    let handler = NotificationHandler::new(client);
    let (notifications, _) = handler
        .list(None, Some(10), Some("/uploads"), Some(10))
        .await
        .unwrap();

    assert_eq!(notifications.len(), 1);
    assert_eq!(notifications[0].path.as_deref(), Some("/uploads"));
}

#[tokio::test]
async fn test_get_notification_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 123,
        "path": "/important",
        "group_id": 5,
        "group_name": "Admins",
        "notify_on_upload": true,
        "notify_on_download": true,
        "notify_on_delete": true,
        "notify_on_copy": false,
        "notify_on_move": false,
        "send_interval": "fifteen_minutes",
        "recursive": true,
        "message": "New activity in important folder",
        "triggering_filenames": ["*.pdf", "*.doc"],
    });

    Mock::given(method("GET"))
        .and(path("/notifications/123"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = NotificationHandler::new(client);
    let notification = handler.get(123).await.unwrap();

    assert_eq!(notification.id, Some(123));
    assert_eq!(notification.path.as_deref(), Some("/important"));
    assert_eq!(notification.notify_on_upload, Some(true));
    assert_eq!(notification.notify_on_download, Some(true));
    assert_eq!(notification.notify_on_delete, Some(true));
    assert_eq!(
        notification.send_interval.as_deref(),
        Some("fifteen_minutes")
    );
    assert_eq!(notification.recursive, Some(true));
    assert!(notification.triggering_filenames.is_some());
}

#[tokio::test]
async fn test_create_notification_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 456,
        "path": "/uploads",
        "group_id": 10,
        "notify_on_upload": true,
        "send_interval": "hourly",
        "recursive": true,
    });

    Mock::given(method("POST"))
        .and(path("/notifications"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = NotificationHandler::new(client);
    let notification = handler
        .create(
            Some("/uploads"),
            Some(10),
            Some(true),
            None,
            None,
            Some("hourly"),
            Some(true),
            None,
        )
        .await
        .unwrap();

    assert_eq!(notification.id, Some(456));
    assert_eq!(notification.path.as_deref(), Some("/uploads"));
    assert_eq!(notification.notify_on_upload, Some(true));
    assert_eq!(notification.send_interval.as_deref(), Some("hourly"));
}

#[tokio::test]
async fn test_create_notification_with_message() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 789,
        "path": "/sensitive",
        "message": "Alert: Activity detected",
    });

    Mock::given(method("POST"))
        .and(path("/notifications"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = NotificationHandler::new(client);
    let notification = handler
        .create(
            Some("/sensitive"),
            None,
            None,
            None,
            None,
            None,
            None,
            Some("Alert: Activity detected"),
        )
        .await
        .unwrap();

    assert_eq!(notification.id, Some(789));
    assert_eq!(
        notification.message.as_deref(),
        Some("Alert: Activity detected")
    );
}

#[tokio::test]
async fn test_update_notification_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 123,
        "notify_on_upload": false,
        "notify_on_download": true,
        "send_interval": "daily",
    });

    Mock::given(method("PATCH"))
        .and(path("/notifications/123"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = NotificationHandler::new(client);
    let notification = handler
        .update(123, Some(false), Some(true), None, Some("daily"))
        .await
        .unwrap();

    assert_eq!(notification.id, Some(123));
    assert_eq!(notification.notify_on_upload, Some(false));
    assert_eq!(notification.notify_on_download, Some(true));
    assert_eq!(notification.send_interval.as_deref(), Some("daily"));
}

#[tokio::test]
async fn test_delete_notification_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/notifications/123"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = NotificationHandler::new(client);
    let result = handler.delete(123).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_notification_not_found() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Not Found",
        "message": "Notification not found"
    });

    Mock::given(method("GET"))
        .and(path("/notifications/999"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = NotificationHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::NotFound { .. }) => {}
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_create_notification_bad_request() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Bad Request",
        "message": "Invalid send_interval value"
    });

    Mock::given(method("POST"))
        .and(path("/notifications"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(400).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = NotificationHandler::new(client);
    let result = handler
        .create(
            Some("/test"),
            None,
            None,
            None,
            None,
            Some("invalid"),
            None,
            None,
        )
        .await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::BadRequest { .. }) => {}
        _ => panic!("Expected BadRequest error"),
    }
}

#[tokio::test]
async fn test_delete_notification_forbidden() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Forbidden",
        "message": "You don't have permission to delete this notification"
    });

    Mock::given(method("DELETE"))
        .and(path("/notifications/123"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(403).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = NotificationHandler::new(client);
    let result = handler.delete(123).await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::Forbidden { .. }) => {}
        _ => panic!("Expected Forbidden error"),
    }
}
