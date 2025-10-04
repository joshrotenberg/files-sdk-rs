use files_sdk::{FilesClient, PublicKeyHandler};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

async fn setup() -> (MockServer, PublicKeyHandler) {
    let mock_server = MockServer::start().await;
    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();
    let handler = PublicKeyHandler::new(client);
    (mock_server, handler)
}

#[tokio::test]
async fn test_list_public_keys() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("GET"))
        .and(path("/public_keys"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 1,
                "title": "Work Laptop",
                "user_id": 123,
                "username": "john@example.com",
                "fingerprint": "ab:cd:ef:12:34:56",
                "fingerprint_sha256": "SHA256:abcdef123456",
                "created_at": "2024-01-01T00:00:00Z",
                "last_login_at": "2024-01-10T00:00:00Z"
            },
            {
                "id": 2,
                "title": "Home Desktop",
                "user_id": 123,
                "username": "john@example.com",
                "fingerprint": "12:34:56:ab:cd:ef",
                "fingerprint_sha256": "SHA256:123456abcdef",
                "created_at": "2024-01-02T00:00:00Z",
                "last_login_at": null
            }
        ])))
        .mount(&mock_server)
        .await;

    let (keys, _) = handler.list(None, None, None).await.unwrap();
    assert_eq!(keys.len(), 2);
    assert_eq!(keys[0].title, Some("Work Laptop".to_string()));
    assert_eq!(keys[1].title, Some("Home Desktop".to_string()));
}

#[tokio::test]
async fn test_list_public_keys_for_user() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("GET"))
        .and(path("/public_keys"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 1,
                "title": "User Key",
                "user_id": 456,
                "username": "jane@example.com",
                "fingerprint": "aa:bb:cc:dd:ee:ff",
                "created_at": "2024-01-01T00:00:00Z"
            }
        ])))
        .mount(&mock_server)
        .await;

    let (keys, _) = handler.list(Some(456), None, None).await.unwrap();
    assert_eq!(keys.len(), 1);
    assert_eq!(keys[0].user_id, Some(456));
}

#[tokio::test]
async fn test_get_public_key() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("GET"))
        .and(path("/public_keys/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "title": "My SSH Key",
            "user_id": 123,
            "username": "user@example.com",
            "fingerprint": "ab:cd:ef:12:34:56",
            "fingerprint_sha256": "SHA256:abcdef123456",
            "created_at": "2024-01-01T00:00:00Z",
            "last_login_at": "2024-01-10T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let key = handler.get(1).await.unwrap();
    assert_eq!(key.id, Some(1));
    assert_eq!(key.title, Some("My SSH Key".to_string()));
    assert_eq!(key.fingerprint, Some("ab:cd:ef:12:34:56".to_string()));
}

#[tokio::test]
async fn test_create_public_key() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("POST"))
        .and(path("/public_keys"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 10,
            "title": "New Key",
            "user_id": 123,
            "username": "user@example.com",
            "fingerprint": "11:22:33:44:55:66",
            "fingerprint_sha256": "SHA256:112233445566",
            "created_at": "2024-01-15T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let key = handler
        .create(123, "New Key", "ssh-rsa AAAAB3NzaC1yc2EA...")
        .await
        .unwrap();

    assert_eq!(key.id, Some(10));
    assert_eq!(key.title, Some("New Key".to_string()));
}

#[tokio::test]
async fn test_generate_public_key() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("POST"))
        .and(path("/public_keys"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 11,
            "title": "Generated Key",
            "user_id": 123,
            "username": "user@example.com",
            "status": "complete",
            "generated_public_key": "ssh-ed25519 AAAAC3NzaC1lZDI1NTE5...",
            "generated_private_key": "-----BEGIN OPENSSH PRIVATE KEY-----\n...",
            "fingerprint": "99:88:77:66:55:44",
            "fingerprint_sha256": "SHA256:998877665544",
            "created_at": "2024-01-15T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let key = handler
        .generate(123, "Generated Key", "ed25519", None, None)
        .await
        .unwrap();

    assert_eq!(key.id, Some(11));
    assert_eq!(key.status, Some("complete".to_string()));
    assert!(key.generated_public_key.is_some());
    assert!(key.generated_private_key.is_some());
}

#[tokio::test]
async fn test_generate_rsa_key_with_password() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("POST"))
        .and(path("/public_keys"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 12,
            "title": "RSA Key",
            "user_id": 123,
            "status": "complete",
            "generated_public_key": "ssh-rsa AAAAB3NzaC1yc2EA...",
            "generated_private_key": "-----BEGIN RSA PRIVATE KEY-----\n...",
            "fingerprint": "aa:bb:cc:dd:ee:ff",
            "created_at": "2024-01-15T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let key = handler
        .generate(123, "RSA Key", "rsa", Some(2048), Some("my-password"))
        .await
        .unwrap();

    assert_eq!(key.title, Some("RSA Key".to_string()));
}

#[tokio::test]
async fn test_update_public_key() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("PATCH"))
        .and(path("/public_keys/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "title": "Updated Title",
            "user_id": 123,
            "username": "user@example.com",
            "fingerprint": "ab:cd:ef:12:34:56",
            "created_at": "2024-01-01T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let key = handler.update(1, "Updated Title").await.unwrap();
    assert_eq!(key.title, Some("Updated Title".to_string()));
}

#[tokio::test]
async fn test_delete_public_key() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("DELETE"))
        .and(path("/public_keys/1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let result = handler.delete(1).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_public_key_not_found() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("GET"))
        .and(path("/public_keys/999"))
        .respond_with(
            ResponseTemplate::new(404).set_body_json(serde_json::json!({"error": "Not Found"})),
        )
        .mount(&mock_server)
        .await;

    let result = handler.get(999).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_public_key_bad_request() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("POST"))
        .and(path("/public_keys"))
        .respond_with(
            ResponseTemplate::new(400)
                .set_body_json(serde_json::json!({"error": "Invalid public key format"})),
        )
        .mount(&mock_server)
        .await;

    let result = handler.create(123, "Bad Key", "invalid-key-data").await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_public_keys_with_pagination() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("GET"))
        .and(path("/public_keys"))
        .respond_with(
            ResponseTemplate::new(200)
                .set_body_json(serde_json::json!([
                    {"id": 1, "title": "Key 1", "user_id": 123},
                    {"id": 2, "title": "Key 2", "user_id": 123}
                ]))
                .insert_header("X-Files-Cursor-Next", "next_cursor_value"),
        )
        .mount(&mock_server)
        .await;

    let (keys, pagination) = handler.list(Some(123), None, Some(2)).await.unwrap();
    assert_eq!(keys.len(), 2);
    assert!(pagination.has_next());
    assert_eq!(
        pagination.cursor_next,
        Some("next_cursor_value".to_string())
    );
}
