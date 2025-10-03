use files_sdk::{BundleHandler, FilesClient};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_bundles_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!([
        {
            "id": 1,
            "code": "abc123",
            "url": "https://subdomain.files.com/f/abc123",
            "description": "Q4 Reports",
            "password_protected": false,
            "permissions": "read",
            "expires_at": "2024-12-31T23:59:59Z",
            "created_at": "2024-01-01T00:00:00Z",
            "user_id": 123,
            "username": "testuser",
        },
        {
            "id": 2,
            "code": "xyz789",
            "url": "https://subdomain.files.com/f/xyz789",
            "description": "Client Deliverables",
            "password_protected": true,
            "permissions": "read_write",
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/bundles"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BundleHandler::new(client);
    let (bundles, _pagination) = handler.list(None, None, None).await.unwrap();

    assert_eq!(bundles.len(), 2);
    assert_eq!(bundles[0].id, Some(1));
    assert_eq!(bundles[0].code.as_deref(), Some("abc123"));
    assert_eq!(bundles[0].description.as_deref(), Some("Q4 Reports"));
    assert_eq!(bundles[1].password_protected, Some(true));
}

#[tokio::test]
async fn test_list_bundles_with_filters() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!([
        {
            "id": 1,
            "code": "abc123",
            "user_id": 123,
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/bundles"))
        .and(query_param("user_id", "123"))
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

    let handler = BundleHandler::new(client);
    let (bundles, _pagination) = handler.list(Some(123), None, Some(10)).await.unwrap();

    assert_eq!(bundles.len(), 1);
    assert_eq!(bundles[0].user_id, Some(123));
}

#[tokio::test]
async fn test_get_bundle_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 123,
        "code": "abc123",
        "url": "https://subdomain.files.com/f/abc123",
        "description": "Important Files",
        "note": "Internal note about this bundle",
        "password_protected": false,
        "permissions": "read",
        "require_registration": true,
        "max_uses": 100,
        "paths": ["/folder1/file1.txt", "/folder2/file2.txt"],
    });

    Mock::given(method("GET"))
        .and(path("/bundles/123"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BundleHandler::new(client);
    let bundle = handler.get(123).await.unwrap();

    assert_eq!(bundle.id, Some(123));
    assert_eq!(bundle.code.as_deref(), Some("abc123"));
    assert_eq!(bundle.description.as_deref(), Some("Important Files"));
    assert_eq!(
        bundle.note.as_deref(),
        Some("Internal note about this bundle")
    );
    assert_eq!(bundle.require_registration, Some(true));
    assert_eq!(bundle.max_uses, Some(100));
    assert!(bundle.paths.is_some());
    assert_eq!(bundle.paths.as_ref().unwrap().len(), 2);
}

#[tokio::test]
async fn test_create_bundle_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 456,
        "code": "newbundle",
        "url": "https://subdomain.files.com/f/newbundle",
        "description": "New Share Link",
        "permissions": "read",
        "require_registration": true,
    });

    Mock::given(method("POST"))
        .and(path("/bundles"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BundleHandler::new(client);
    let bundle = handler
        .create(
            vec!["/documents/report.pdf".to_string()],
            None,
            None,
            None,
            Some("New Share Link"),
            None,
            Some("newbundle"),
            Some(true),
            Some("read"),
        )
        .await
        .unwrap();

    assert_eq!(bundle.id, Some(456));
    assert_eq!(bundle.code.as_deref(), Some("newbundle"));
    assert_eq!(bundle.description.as_deref(), Some("New Share Link"));
}

#[tokio::test]
async fn test_create_bundle_with_password() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 789,
        "code": "secure",
        "password_protected": true,
    });

    Mock::given(method("POST"))
        .and(path("/bundles"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BundleHandler::new(client);
    let bundle = handler
        .create(
            vec!["/secure/data.zip".to_string()],
            Some("secret123"),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(bundle.password_protected, Some(true));
}

#[tokio::test]
async fn test_update_bundle_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 123,
        "code": "abc123",
        "description": "Updated Description",
        "max_uses": 50,
    });

    Mock::given(method("PATCH"))
        .and(path("/bundles/123"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BundleHandler::new(client);
    let bundle = handler
        .update(123, None, None, Some(50), Some("Updated Description"), None)
        .await
        .unwrap();

    assert_eq!(bundle.id, Some(123));
    assert_eq!(bundle.description.as_deref(), Some("Updated Description"));
    assert_eq!(bundle.max_uses, Some(50));
}

#[tokio::test]
async fn test_delete_bundle_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/bundles/123"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BundleHandler::new(client);
    let result = handler.delete(123).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_share_bundle_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "success": true
    });

    Mock::given(method("POST"))
        .and(path("/bundles/123/share"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BundleHandler::new(client);
    let result = handler
        .share(
            123,
            vec!["user@example.com".to_string()],
            Some("Check out these files!"),
        )
        .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_bundle_not_found() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Not Found",
        "message": "Bundle not found"
    });

    Mock::given(method("GET"))
        .and(path("/bundles/999"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BundleHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::NotFound { .. }) => {}
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_create_bundle_bad_request() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Bad Request",
        "message": "Paths are required"
    });

    Mock::given(method("POST"))
        .and(path("/bundles"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(400).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BundleHandler::new(client);
    let result = handler
        .create(vec![], None, None, None, None, None, None, None, None)
        .await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::BadRequest { .. }) => {}
        _ => panic!("Expected BadRequest error"),
    }
}

#[tokio::test]
async fn test_delete_bundle_forbidden() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Forbidden",
        "message": "You don't have permission to delete this bundle"
    });

    Mock::given(method("DELETE"))
        .and(path("/bundles/123"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(403).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BundleHandler::new(client);
    let result = handler.delete(123).await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::Forbidden { .. }) => {}
        _ => panic!("Expected Forbidden error"),
    }
}
