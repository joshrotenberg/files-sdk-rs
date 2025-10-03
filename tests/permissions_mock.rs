use files_sdk::{FilesClient, PermissionHandler};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_permissions_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!([
        {
            "id": 1,
            "path": "/documents",
            "user_id": 123,
            "username": "testuser",
            "permission": "full",
            "recursive": true,
        },
        {
            "id": 2,
            "path": "/shared",
            "group_id": 456,
            "group_name": "Engineering",
            "permission": "readonly",
            "recursive": false,
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/permissions"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PermissionHandler::new(client);
    let (permissions, _pagination) = handler.list(None, None).await.unwrap();

    assert_eq!(permissions.len(), 2);
    assert_eq!(permissions[0].id, Some(1));
    assert_eq!(permissions[0].path.as_deref(), Some("/documents"));
    assert_eq!(permissions[0].permission.as_deref(), Some("full"));
    assert_eq!(permissions[1].group_name.as_deref(), Some("Engineering"));
}

#[tokio::test]
async fn test_list_permissions_with_pagination() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!([
        {"id": 1, "path": "/path1", "permission": "full"},
        {"id": 2, "path": "/path2", "permission": "readonly"},
    ]);

    Mock::given(method("GET"))
        .and(path("/permissions"))
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

    let handler = PermissionHandler::new(client);
    let (permissions, _pagination) = handler.list(None, Some(2)).await.unwrap();

    assert_eq!(permissions.len(), 2);
}

#[tokio::test]
async fn test_create_permission_for_user_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 789,
        "path": "/documents",
        "user_id": 123,
        "username": "testuser",
        "permission": "full",
        "recursive": true,
    });

    Mock::given(method("POST"))
        .and(path("/permissions"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PermissionHandler::new(client);
    let permission = handler
        .create(
            "/documents",
            Some("full"),
            Some(123),
            None,
            None,
            None,
            Some(true),
        )
        .await
        .unwrap();

    assert_eq!(permission.id, Some(789));
    assert_eq!(permission.path.as_deref(), Some("/documents"));
    assert_eq!(permission.user_id, Some(123));
    assert_eq!(permission.permission.as_deref(), Some("full"));
    assert_eq!(permission.recursive, Some(true));
}

#[tokio::test]
async fn test_create_permission_for_group_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 790,
        "path": "/shared",
        "group_id": 456,
        "group_name": "Engineering",
        "permission": "readonly",
        "recursive": false,
    });

    Mock::given(method("POST"))
        .and(path("/permissions"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PermissionHandler::new(client);
    let permission = handler
        .create(
            "/shared",
            Some("readonly"),
            None,
            None,
            Some(456),
            None,
            Some(false),
        )
        .await
        .unwrap();

    assert_eq!(permission.id, Some(790));
    assert_eq!(permission.group_id, Some(456));
    assert_eq!(permission.permission.as_deref(), Some("readonly"));
}

#[tokio::test]
async fn test_delete_permission_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/permissions/123"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PermissionHandler::new(client);
    let result = handler.delete(123).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_permissions_for_user_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!([
        {
            "id": 1,
            "path": "/documents",
            "user_id": 123,
            "permission": "full",
        },
        {
            "id": 2,
            "path": "/reports",
            "user_id": 123,
            "permission": "readonly",
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/users/123/permissions"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PermissionHandler::new(client);
    let (permissions, _pagination) = handler.list_for_user(123, None, None).await.unwrap();

    assert_eq!(permissions.len(), 2);
    assert_eq!(permissions[0].user_id, Some(123));
    assert_eq!(permissions[1].user_id, Some(123));
}

#[tokio::test]
async fn test_list_permissions_for_group_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!([
        {
            "id": 1,
            "path": "/shared",
            "group_id": 456,
            "permission": "readonly",
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/groups/456/permissions"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PermissionHandler::new(client);
    let (permissions, _pagination) = handler.list_for_group(456, None, None).await.unwrap();

    assert_eq!(permissions.len(), 1);
    assert_eq!(permissions[0].group_id, Some(456));
}

#[tokio::test]
async fn test_create_permission_bad_request() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Bad Request",
        "message": "Path is required"
    });

    Mock::given(method("POST"))
        .and(path("/permissions"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(400).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PermissionHandler::new(client);
    let result = handler.create("", None, None, None, None, None, None).await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::BadRequest { .. }) => {}
        _ => panic!("Expected BadRequest error"),
    }
}

#[tokio::test]
async fn test_delete_permission_not_found() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Not Found",
        "message": "Permission not found"
    });

    Mock::given(method("DELETE"))
        .and(path("/permissions/999"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = PermissionHandler::new(client);
    let result = handler.delete(999).await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::NotFound { .. }) => {}
        _ => panic!("Expected NotFound error"),
    }
}
