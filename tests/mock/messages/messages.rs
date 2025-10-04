//! Mock tests for MessageHandler

use files_sdk::{FilesClient, MessageHandler};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, path_regex},
};

#[tokio::test]
async fn test_list_messages() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/messages"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 1,
                "subject": "Weekly Update",
                "body": "This week's progress...",
                "comments": []
            },
            {
                "id": 2,
                "subject": "Planning Session",
                "body": "Next sprint planning",
                "comments": [
                    {
                        "id": 10,
                        "body": "Great idea!",
                        "reactions": []
                    }
                ]
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = MessageHandler::new(client);
    let (messages, _) = handler.list(None, None, None).await.unwrap();

    assert_eq!(messages.len(), 2);
    assert_eq!(messages[0].id, Some(1));
    assert_eq!(messages[0].subject, Some("Weekly Update".to_string()));
    assert_eq!(messages[1].id, Some(2));
    assert!(messages[1].comments.is_some());
    assert_eq!(messages[1].comments.as_ref().unwrap().len(), 1);
}

#[tokio::test]
async fn test_list_messages_with_project_filter() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/messages"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 1,
                "subject": "Project A Update",
                "body": "Status update...",
                "comments": []
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = MessageHandler::new(client);
    let (messages, _) = handler.list(None, None, Some(1)).await.unwrap();

    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].subject, Some("Project A Update".to_string()));
}

#[tokio::test]
async fn test_list_messages_with_pagination() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/messages"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 3,
                "subject": "Page 2 Message",
                "body": "Content...",
                "comments": []
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = MessageHandler::new(client);
    let (messages, _) = handler
        .list(Some("cursor123"), Some(10), None)
        .await
        .unwrap();

    assert_eq!(messages.len(), 1);
}

#[tokio::test]
async fn test_get_message() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/messages/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "subject": "Important Announcement",
            "body": "Please read carefully...",
            "comments": [
                {
                    "id": 5,
                    "body": "Thanks for sharing",
                    "reactions": [{"emoji": "thumbsup", "count": 3}]
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

    let handler = MessageHandler::new(client);
    let message = handler.get(1).await.unwrap();

    assert_eq!(message.id, Some(1));
    assert_eq!(message.subject, Some("Important Announcement".to_string()));
    assert_eq!(message.body, Some("Please read carefully...".to_string()));
    assert!(message.comments.is_some());
    let comments = message.comments.unwrap();
    assert_eq!(comments.len(), 1);
    assert_eq!(comments[0].id, Some(5));
}

#[tokio::test]
async fn test_create_message() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/messages"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 42,
            "subject": "New Message",
            "body": "This is a new message",
            "comments": []
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = MessageHandler::new(client);
    let message = handler
        .create(1, "New Message", "This is a new message")
        .await
        .unwrap();

    assert_eq!(message.id, Some(42));
    assert_eq!(message.subject, Some("New Message".to_string()));
    assert_eq!(message.body, Some("This is a new message".to_string()));
}

#[tokio::test]
async fn test_update_message_subject() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/messages/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "subject": "Updated Subject",
            "body": "Original body",
            "comments": []
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = MessageHandler::new(client);
    let message = handler
        .update(1, Some("Updated Subject"), None)
        .await
        .unwrap();

    assert_eq!(message.id, Some(1));
    assert_eq!(message.subject, Some("Updated Subject".to_string()));
}

#[tokio::test]
async fn test_update_message_body() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/messages/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "subject": "Original Subject",
            "body": "Updated body content",
            "comments": []
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = MessageHandler::new(client);
    let message = handler
        .update(1, None, Some("Updated body content"))
        .await
        .unwrap();

    assert_eq!(message.id, Some(1));
    assert_eq!(message.body, Some("Updated body content".to_string()));
}

#[tokio::test]
async fn test_update_message_both() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/messages/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "subject": "Completely Updated",
            "body": "New body too",
            "comments": []
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = MessageHandler::new(client);
    let message = handler
        .update(1, Some("Completely Updated"), Some("New body too"))
        .await
        .unwrap();

    assert_eq!(message.subject, Some("Completely Updated".to_string()));
    assert_eq!(message.body, Some("New body too".to_string()));
}

#[tokio::test]
async fn test_delete_message() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/messages/1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = MessageHandler::new(client);
    handler.delete(1).await.unwrap();
}

#[tokio::test]
async fn test_get_message_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/messages/\d+$"))
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

    let handler = MessageHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_message_bad_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/messages"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "error": "Bad Request - Missing required fields",
            "http-code": 400
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = MessageHandler::new(client);
    let result = handler.create(1, "", "").await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_message_forbidden() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path_regex(r"^/messages/\d+$"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "error": "Forbidden - You don't have permission to delete this message",
            "http-code": 403
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = MessageHandler::new(client);
    let result = handler.delete(1).await;

    assert!(result.is_err());
}
