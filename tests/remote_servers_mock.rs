//! Mock tests for RemoteServerHandler

use files_sdk::{FilesClient, RemoteServerHandler};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, path_regex},
};

#[tokio::test]
async fn test_list_remote_servers() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/remote_servers"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 1,
                "name": "Production S3",
                "server_type": "s3",
                "s3_bucket": "prod-bucket",
                "s3_region": "us-east-1",
                "aws_access_key": "AKIA...",
                "disabled": false
            },
            {
                "id": 2,
                "name": "SFTP Server",
                "server_type": "sftp",
                "hostname": "sftp.example.com",
                "port": 22,
                "username": "sftpuser",
                "disabled": false
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RemoteServerHandler::new(client);
    let (servers, _) = handler.list(None, None).await.unwrap();

    assert_eq!(servers.len(), 2);
    assert_eq!(servers[0].server_type, Some("s3".to_string()));
    assert_eq!(servers[1].server_type, Some("sftp".to_string()));
}

#[tokio::test]
async fn test_get_remote_server() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/remote_servers/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "name": "Azure Blob Storage",
            "server_type": "azure",
            "azure_blob_storage_account": "myaccount",
            "azure_blob_storage_container": "mycontainer",
            "azure_blob_storage_dns_suffix": "core.windows.net",
            "supports_versioning": true,
            "disabled": false
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RemoteServerHandler::new(client);
    let server = handler.get(1).await.unwrap();

    assert_eq!(server.id, Some(1));
    assert_eq!(server.name, Some("Azure Blob Storage".to_string()));
    assert_eq!(server.server_type, Some("azure".to_string()));
    assert_eq!(server.supports_versioning, Some(true));
}

#[tokio::test]
async fn test_get_configuration_file() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/remote_servers/1/configuration_file"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "config": "base64encodedconfig...",
            "filename": "files_agent_config.json"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RemoteServerHandler::new(client);
    let config = handler.get_configuration_file(1).await.unwrap();

    assert_eq!(config["filename"], "files_agent_config.json");
}

#[tokio::test]
async fn test_create_s3_remote_server() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/remote_servers"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 42,
            "name": "New S3 Bucket",
            "server_type": "s3",
            "s3_bucket": "new-bucket",
            "s3_region": "us-west-2",
            "disabled": false
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RemoteServerHandler::new(client);
    let server = handler
        .create(
            "New S3 Bucket",
            "s3",
            None,
            None,
            None,
            Some("new-bucket"),
            Some("us-west-2"),
        )
        .await
        .unwrap();

    assert_eq!(server.id, Some(42));
    assert_eq!(server.server_type, Some("s3".to_string()));
    assert_eq!(server.s3_bucket, Some("new-bucket".to_string()));
}

#[tokio::test]
async fn test_create_sftp_remote_server() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/remote_servers"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 50,
            "name": "Partner SFTP",
            "server_type": "sftp",
            "hostname": "partner.example.com",
            "port": 2222,
            "username": "partner-user",
            "ssl": "if_available",
            "disabled": false
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RemoteServerHandler::new(client);
    let server = handler
        .create(
            "Partner SFTP",
            "sftp",
            Some("partner.example.com"),
            Some("partner-user"),
            Some(2222),
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(server.server_type, Some("sftp".to_string()));
    assert_eq!(server.hostname, Some("partner.example.com".to_string()));
    assert_eq!(server.port, Some(2222));
}

#[tokio::test]
async fn test_update_remote_server() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/remote_servers/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "name": "Updated Server Name",
            "server_type": "sftp",
            "hostname": "new-hostname.example.com",
            "port": 2222,
            "disabled": false
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RemoteServerHandler::new(client);
    let server = handler
        .update(
            1,
            Some("Updated Server Name"),
            Some("new-hostname.example.com"),
            Some(2222),
            None,
        )
        .await
        .unwrap();

    assert_eq!(server.name, Some("Updated Server Name".to_string()));
    assert_eq!(
        server.hostname,
        Some("new-hostname.example.com".to_string())
    );
}

#[tokio::test]
async fn test_update_remote_server_disable() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/remote_servers/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "name": "Server",
            "server_type": "s3",
            "disabled": true
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RemoteServerHandler::new(client);
    let server = handler
        .update(1, None, None, None, Some(true))
        .await
        .unwrap();

    assert_eq!(server.disabled, Some(true));
}

#[tokio::test]
async fn test_delete_remote_server() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/remote_servers/1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RemoteServerHandler::new(client);
    handler.delete(1).await.unwrap();
}

#[tokio::test]
async fn test_wasabi_remote_server() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/remote_servers/5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 5,
            "name": "Wasabi Storage",
            "server_type": "wasabi",
            "wasabi_bucket": "my-wasabi-bucket",
            "wasabi_region": "us-east-1",
            "wasabi_access_key": "WASABI_KEY_...",
            "disabled": false
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RemoteServerHandler::new(client);
    let server = handler.get(5).await.unwrap();

    assert_eq!(server.server_type, Some("wasabi".to_string()));
    assert_eq!(server.wasabi_bucket, Some("my-wasabi-bucket".to_string()));
}

#[tokio::test]
async fn test_google_cloud_storage() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/remote_servers/6"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 6,
            "name": "GCS Bucket",
            "server_type": "google_cloud_storage",
            "google_cloud_storage_bucket": "my-gcs-bucket",
            "google_cloud_storage_project_id": "my-project-123",
            "disabled": false
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RemoteServerHandler::new(client);
    let server = handler.get(6).await.unwrap();

    assert_eq!(server.server_type, Some("google_cloud_storage".to_string()));
    assert_eq!(
        server.google_cloud_storage_project_id,
        Some("my-project-123".to_string())
    );
}

#[tokio::test]
async fn test_get_remote_server_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/remote_servers/\d+$"))
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

    let handler = RemoteServerHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_remote_server_bad_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/remote_servers"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "error": "Bad Request - Invalid server type",
            "http-code": 400
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = RemoteServerHandler::new(client);
    let result = handler
        .create("Test", "invalid_type", None, None, None, None, None)
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_remote_server_forbidden() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path_regex(r"^/remote_servers/\d+$"))
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

    let handler = RemoteServerHandler::new(client);
    let result = handler.delete(1).await;

    assert!(result.is_err());
}
