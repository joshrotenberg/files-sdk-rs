use files_sdk::{FilesClient, GroupHandler};
use wiremock::matchers::{header, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

#[tokio::test]
async fn test_list_groups_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!([
        {
            "id": 1,
            "name": "Engineering",
            "notes": "Engineering team",
            "admin_ids": "1,2",
            "user_ids": "1,2,3",
            "usernames": "user1,user2,user3",
            "allowed_ips": null,
            "ftp_permission": true,
            "sftp_permission": true,
        }
    ]);

    Mock::given(method("GET"))
        .and(path("/groups"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = GroupHandler::new(client);
    let (groups, _pagination) = handler.list(None, None).await.unwrap();

    assert_eq!(groups.len(), 1);
    assert_eq!(groups[0].id, Some(1));
    assert_eq!(groups[0].name.as_deref(), Some("Engineering"));
    assert_eq!(groups[0].user_ids.as_deref(), Some("1,2,3"));
}

#[tokio::test]
async fn test_list_groups_with_pagination() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!([
        {"id": 1, "name": "Group 1"},
        {"id": 2, "name": "Group 2"},
    ]);

    Mock::given(method("GET"))
        .and(path("/groups"))
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

    let handler = GroupHandler::new(client);
    let (groups, _pagination) = handler.list(None, Some(2)).await.unwrap();

    assert_eq!(groups.len(), 2);
}

#[tokio::test]
async fn test_get_group_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 123,
        "name": "Engineering",
        "notes": "Engineering team members",
        "admin_ids": "1",
        "user_ids": "1,2,3,4",
        "ftp_permission": true,
        "sftp_permission": true,
    });

    Mock::given(method("GET"))
        .and(path("/groups/123"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = GroupHandler::new(client);
    let group = handler.get(123).await.unwrap();

    assert_eq!(group.id, Some(123));
    assert_eq!(group.name.as_deref(), Some("Engineering"));
    assert_eq!(group.notes.as_deref(), Some("Engineering team members"));
}

#[tokio::test]
async fn test_create_group_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 456,
        "name": "New Team",
        "notes": "Brand new team",
    });

    Mock::given(method("POST"))
        .and(path("/groups"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(201).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = GroupHandler::new(client);
    let group = handler
        .create("New Team", Some("Brand new team"), None)
        .await
        .unwrap();

    assert_eq!(group.id, Some(456));
    assert_eq!(group.name.as_deref(), Some("New Team"));
}

#[tokio::test]
async fn test_update_group_success() {
    let mock_server = MockServer::start().await;

    let response_body = serde_json::json!({
        "id": 123,
        "name": "Updated Name",
        "notes": "Updated notes",
    });

    Mock::given(method("PATCH"))
        .and(path("/groups/123"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = GroupHandler::new(client);
    let group = handler
        .update(123, Some("Updated Name"), Some("Updated notes"))
        .await
        .unwrap();

    assert_eq!(group.id, Some(123));
    assert_eq!(group.name.as_deref(), Some("Updated Name"));
}

#[tokio::test]
async fn test_delete_group_success() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/groups/123"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = GroupHandler::new(client);
    let result = handler.delete(123).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_group_not_found() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Not Found",
        "message": "Group not found"
    });

    Mock::given(method("GET"))
        .and(path("/groups/999"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(404).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = GroupHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::NotFound { .. }) => {}
        _ => panic!("Expected NotFound error"),
    }
}

#[tokio::test]
async fn test_create_group_bad_request() {
    let mock_server = MockServer::start().await;

    let error_body = serde_json::json!({
        "error": "Bad Request",
        "message": "Name is required"
    });

    Mock::given(method("POST"))
        .and(path("/groups"))
        .and(header("X-FilesAPI-Key", "test-key"))
        .respond_with(ResponseTemplate::new(400).set_body_json(&error_body))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = GroupHandler::new(client);
    let result = handler.create("", None, None).await;

    assert!(result.is_err());
    match result {
        Err(files_sdk::FilesError::BadRequest { .. }) => {}
        _ => panic!("Expected BadRequest error"),
    }
}
