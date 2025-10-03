use files_sdk::{FilesClient, UserHandler};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_users_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!([
        {
            "id": 1,
            "username": "testuser",
            "email": "test@example.com",
            "name": "Test User",
            "site_admin": false,
            "group_ids": "",
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = UserHandler::new(client);
    let (users, _pagination) = handler.list(None, None).await.unwrap();

    assert_eq!(users.len(), 1);
    assert_eq!(users[0].id, Some(1));
    assert_eq!(users[0].username.as_deref(), Some("testuser"));
    assert_eq!(users[0].email.as_deref(), Some("test@example.com"));
}

#[tokio::test]
async fn test_list_users_with_pagination() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!([
        {"id": 1, "username": "user1", "email": "user1@example.com"},
        {"id": 2, "username": "user2", "email": "user2@example.com"},
    ]);

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(query_param("per_page", "2"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = UserHandler::new(client);
    let (users, _pagination) = handler.list(None, Some(2)).await.unwrap();

    assert_eq!(users.len(), 2);
}

#[tokio::test]
async fn test_get_user_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 123,
        "username": "testuser",
        "email": "test@example.com",
        "name": "Test User",
        "site_admin": true,
    });

    Mock::given(method("GET"))
        .and(path("/users/123"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = UserHandler::new(client);
    let user = handler.get(123).await.unwrap();

    assert_eq!(user.id, Some(123));
    assert_eq!(user.username.as_deref(), Some("testuser"));
    assert_eq!(user.site_admin, Some(true));
}

#[tokio::test]
async fn test_get_user_not_found() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Not Found",
        "message": "User not found"
    });

    Mock::given(method("GET"))
        .and(path("/users/999"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = UserHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::NotFound { .. }) => {}
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_authentication_failed() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Unauthorized",
        "message": "Invalid API key"
    });

    Mock::given(method("GET"))
        .and(path("/users"))
        .and(header("X-FilesAPI-Key", "invalid-key"))
        .respond_with(ResponseTemplate::new(401).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("invalid-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = UserHandler::new(client);
    let result = handler.list(None, None).await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::AuthenticationFailed { .. }) => {}
        _ => panic!("Expected AuthenticationFailed error"),
    }
}

#[tokio::test]
async fn test_delete_user_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/users/123"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = UserHandler::new(client);
    let result = handler.delete(123).await;

    assert!(result.is_ok());
}
