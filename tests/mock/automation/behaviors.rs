//! Mock tests for BehaviorHandler

use files_sdk::{BehaviorHandler, FilesClient};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, path_regex},
};

#[tokio::test]
async fn test_list_behaviors() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/behaviors"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 1,
                "path": "/uploads",
                "behavior": "webhook",
                "name": "Upload Notification",
                "value": {"urls": ["https://example.com/webhook"]},
                "recursive": true
            },
            {
                "id": 2,
                "path": "/temp",
                "behavior": "file_expiration",
                "name": "Cleanup Old Files",
                "value": {"days": 30},
                "recursive": false
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BehaviorHandler::new(client);
    let (behaviors, _) = handler.list(None, None, None, None, None).await.unwrap();

    assert_eq!(behaviors.len(), 2);
    assert_eq!(behaviors[0].id, Some(1));
    assert_eq!(behaviors[0].behavior, Some("webhook".to_string()));
    assert_eq!(behaviors[0].recursive, Some(true));
}

#[tokio::test]
async fn test_list_behaviors_for_folder() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/behaviors/folders/uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 5,
                "path": "/uploads",
                "behavior": "auto_encrypt",
                "name": "Encrypt Uploads",
                "value": {"algorithm": "PGP/GPG", "gpg_key_ids": [1, 2]},
                "recursive": true
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BehaviorHandler::new(client);
    let (behaviors, _) = handler
        .list_for_folder("uploads", None, None)
        .await
        .unwrap();

    assert_eq!(behaviors.len(), 1);
    assert_eq!(behaviors[0].behavior, Some("auto_encrypt".to_string()));
}

#[tokio::test]
async fn test_get_behavior() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/behaviors/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "path": "/secure",
            "behavior": "webhook",
            "name": "Security Webhook",
            "description": "Send webhook on file operations",
            "value": {
                "urls": ["https://security.example.com/hook"],
                "method": "POST",
                "triggers": ["create", "update", "delete"]
            },
            "recursive": true,
            "disable_parent_folder_behavior": false
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BehaviorHandler::new(client);
    let behavior = handler.get(1).await.unwrap();

    assert_eq!(behavior.id, Some(1));
    assert_eq!(behavior.name, Some("Security Webhook".to_string()));
    assert!(behavior.value.is_some());
}

#[tokio::test]
async fn test_create_behavior_webhook() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/behaviors"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 42,
            "path": "/api-uploads",
            "behavior": "webhook",
            "name": "API Upload Hook",
            "value": {"urls": ["https://api.example.com/webhook"]},
            "recursive": true
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BehaviorHandler::new(client);
    let webhook_config = serde_json::json!({"urls": ["https://api.example.com/webhook"]});
    let behavior = handler
        .create(
            "/api-uploads",
            "webhook",
            Some(webhook_config),
            Some("API Upload Hook"),
            Some(true),
        )
        .await
        .unwrap();

    assert_eq!(behavior.id, Some(42));
    assert_eq!(behavior.behavior, Some("webhook".to_string()));
}

#[tokio::test]
async fn test_create_behavior_file_expiration() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/behaviors"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 50,
            "path": "/logs",
            "behavior": "file_expiration",
            "name": "Log Cleanup",
            "value": {"days": 90},
            "recursive": true
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BehaviorHandler::new(client);
    let expiration_config = serde_json::json!({"days": 90});
    let behavior = handler
        .create(
            "/logs",
            "file_expiration",
            Some(expiration_config),
            Some("Log Cleanup"),
            Some(true),
        )
        .await
        .unwrap();

    assert_eq!(behavior.behavior, Some("file_expiration".to_string()));
}

#[tokio::test]
async fn test_update_behavior() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/behaviors/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "path": "/uploads",
            "behavior": "webhook",
            "name": "Updated Webhook",
            "value": {"urls": ["https://new-url.example.com/webhook"]},
            "recursive": true
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BehaviorHandler::new(client);
    let new_config = serde_json::json!({"urls": ["https://new-url.example.com/webhook"]});
    let behavior = handler
        .update(1, Some(new_config), Some("Updated Webhook"), None)
        .await
        .unwrap();

    assert_eq!(behavior.name, Some("Updated Webhook".to_string()));
}

#[tokio::test]
async fn test_update_behavior_disable_parent() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/behaviors/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "path": "/uploads",
            "behavior": "webhook",
            "disable_parent_folder_behavior": true
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BehaviorHandler::new(client);
    let behavior = handler.update(1, None, None, Some(true)).await.unwrap();

    assert_eq!(behavior.disable_parent_folder_behavior, Some(true));
}

#[tokio::test]
async fn test_delete_behavior() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/behaviors/1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BehaviorHandler::new(client);
    handler.delete(1).await.unwrap();
}

#[tokio::test]
async fn test_webhook_test() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/behaviors/webhook/test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success",
            "response_code": 200,
            "response_body": "OK",
            "message": "Webhook test successful"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BehaviorHandler::new(client);
    let result = handler
        .test_webhook(
            "https://example.com/webhook",
            Some("POST"),
            None,
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(result["status"], "success");
    assert_eq!(result["response_code"], 200);
}

#[tokio::test]
async fn test_webhook_test_with_headers() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/behaviors/webhook/test"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "success",
            "response_code": 200,
            "message": "Webhook with custom headers successful"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BehaviorHandler::new(client);
    let headers = serde_json::json!({"X-Custom-Header": "value"});
    let result = handler
        .test_webhook(
            "https://example.com/webhook",
            Some("POST"),
            None,
            Some(headers),
            None,
        )
        .await
        .unwrap();

    assert_eq!(result["status"], "success");
}

#[tokio::test]
async fn test_get_behavior_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/behaviors/\d+$"))
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

    let handler = BehaviorHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_behavior_bad_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/behaviors"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "error": "Bad Request - Invalid behavior type",
            "http-code": 400
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BehaviorHandler::new(client);
    let result = handler
        .create("/path", "invalid_behavior", None, None, None)
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_behavior_forbidden() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path_regex(r"^/behaviors/\d+$"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "error": "Forbidden - Insufficient permissions",
            "http-code": 403
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = BehaviorHandler::new(client);
    let result = handler.delete(1).await;

    assert!(result.is_err());
}
