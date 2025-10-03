//! Mock tests for InboxUploadHandler

use files_sdk::{FilesClient, InboxUploadHandler};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn test_list_inbox_uploads() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/inbox_uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "inbox_registration": {
                    "code": "abc123",
                    "name": "John Doe",
                    "company": "Acme Corp",
                    "email": "john@acme.com",
                    "ip": "192.168.1.1",
                    "inbox_id": 1,
                    "inbox_title": "Job Applications",
                    "created_at": "2024-01-15T10:30:00Z"
                },
                "path": "uploads/resume.pdf",
                "created_at": "2024-01-15T10:32:00Z"
            },
            {
                "inbox_registration": {
                    "code": "def456",
                    "name": "Jane Smith",
                    "email": "jane@example.com",
                    "ip": "192.168.1.2",
                    "inbox_id": 1,
                    "created_at": "2024-01-15T11:00:00Z"
                },
                "path": "uploads/portfolio.zip",
                "created_at": "2024-01-15T11:05:00Z"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = InboxUploadHandler::new(client);
    let (uploads, _) = handler
        .list(None, None, None, None, None, None)
        .await
        .unwrap();

    assert_eq!(uploads.len(), 2);
    assert_eq!(uploads[0].path, Some("uploads/resume.pdf".to_string()));

    let reg1 = uploads[0].inbox_registration.as_ref().unwrap();
    assert_eq!(reg1.code, Some("abc123".to_string()));
    assert_eq!(reg1.name, Some("John Doe".to_string()));
    assert_eq!(reg1.email, Some("john@acme.com".to_string()));

    assert_eq!(uploads[1].path, Some("uploads/portfolio.zip".to_string()));
}

#[tokio::test]
async fn test_list_inbox_uploads_with_pagination() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/inbox_uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "inbox_registration": {
                    "code": "page2-001",
                    "name": "Page Two User",
                    "email": "user@page2.com",
                    "ip": "10.0.0.1",
                    "created_at": "2024-01-16T09:00:00Z"
                },
                "path": "uploads/page2/file.txt",
                "created_at": "2024-01-16T09:15:00Z"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = InboxUploadHandler::new(client);
    let (uploads, _) = handler
        .list(Some("cursor123"), Some(10), None, None, None, None)
        .await
        .unwrap();

    assert_eq!(uploads.len(), 1);
    assert_eq!(uploads[0].path, Some("uploads/page2/file.txt".to_string()));
}

#[tokio::test]
async fn test_list_inbox_uploads_with_sort() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/inbox_uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "inbox_registration": {
                    "code": "sorted-001",
                    "name": "Most Recent",
                    "email": "recent@example.com",
                    "created_at": "2024-01-20T15:00:00Z"
                },
                "path": "uploads/newest.txt",
                "created_at": "2024-01-20T15:30:00Z"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = InboxUploadHandler::new(client);
    let sort_by = serde_json::json!({"created_at": "desc"});
    let (uploads, _) = handler
        .list(None, None, Some(sort_by), None, None, None)
        .await
        .unwrap();

    assert_eq!(uploads.len(), 1);
}

#[tokio::test]
async fn test_list_inbox_uploads_with_filter() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/inbox_uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "inbox_registration": {
                    "code": "filtered-001",
                    "name": "Filtered User",
                    "email": "filtered@example.com",
                    "inbox_id": 123,
                    "created_at": "2024-01-18T12:00:00Z"
                },
                "path": "uploads/filtered/data.csv",
                "created_at": "2024-01-18T12:30:00Z"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = InboxUploadHandler::new(client);
    let filter = serde_json::json!({"folder_behavior_id": 123});
    let (uploads, _) = handler
        .list(None, None, None, Some(filter), None, None)
        .await
        .unwrap();

    assert_eq!(uploads.len(), 1);
}

#[tokio::test]
async fn test_list_inbox_uploads_by_registration_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/inbox_uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "inbox_registration": {
                    "code": "reg-specific",
                    "name": "Registration Specific",
                    "email": "specific@example.com",
                    "inbox_registration_id": 789,
                    "created_at": "2024-01-19T14:00:00Z"
                },
                "path": "uploads/reg789/file1.pdf",
                "created_at": "2024-01-19T14:15:00Z"
            },
            {
                "inbox_registration": {
                    "code": "reg-specific",
                    "name": "Registration Specific",
                    "email": "specific@example.com",
                    "inbox_registration_id": 789,
                    "created_at": "2024-01-19T14:00:00Z"
                },
                "path": "uploads/reg789/file2.pdf",
                "created_at": "2024-01-19T14:20:00Z"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = InboxUploadHandler::new(client);
    let (uploads, _) = handler
        .list(None, None, None, None, Some(789), None)
        .await
        .unwrap();

    assert_eq!(uploads.len(), 2);
    assert_eq!(
        uploads[0].path,
        Some("uploads/reg789/file1.pdf".to_string())
    );
    assert_eq!(
        uploads[1].path,
        Some("uploads/reg789/file2.pdf".to_string())
    );
}

#[tokio::test]
async fn test_list_inbox_uploads_by_inbox_id() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/inbox_uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "inbox_registration": {
                    "code": "inbox-42",
                    "name": "Inbox User 1",
                    "email": "user1@inbox42.com",
                    "inbox_id": 42,
                    "inbox_title": "Customer Uploads",
                    "created_at": "2024-01-17T08:00:00Z"
                },
                "path": "inbox42/file1.txt",
                "created_at": "2024-01-17T08:30:00Z"
            },
            {
                "inbox_registration": {
                    "code": "inbox-42-b",
                    "name": "Inbox User 2",
                    "email": "user2@inbox42.com",
                    "inbox_id": 42,
                    "inbox_title": "Customer Uploads",
                    "created_at": "2024-01-17T09:00:00Z"
                },
                "path": "inbox42/file2.txt",
                "created_at": "2024-01-17T09:15:00Z"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = InboxUploadHandler::new(client);
    let (uploads, _) = handler
        .list(None, None, None, None, None, Some(42))
        .await
        .unwrap();

    assert_eq!(uploads.len(), 2);
    let reg1 = uploads[0].inbox_registration.as_ref().unwrap();
    assert_eq!(reg1.inbox_id, Some(42));
    assert_eq!(reg1.inbox_title, Some("Customer Uploads".to_string()));
}

#[tokio::test]
async fn test_list_inbox_uploads_with_form_data() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/inbox_uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "inbox_registration": {
                    "code": "form-data-001",
                    "name": "Form Submitter",
                    "email": "form@example.com",
                    "form_field_set_id": 5,
                    "form_field_data": {
                        "department": "Engineering",
                        "project": "Alpha",
                        "priority": "high"
                    },
                    "created_at": "2024-01-21T10:00:00Z"
                },
                "path": "forms/submission-001.pdf",
                "created_at": "2024-01-21T10:05:00Z"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = InboxUploadHandler::new(client);
    let (uploads, _) = handler
        .list(None, None, None, None, None, None)
        .await
        .unwrap();

    assert_eq!(uploads.len(), 1);
    let reg = uploads[0].inbox_registration.as_ref().unwrap();
    assert_eq!(reg.form_field_set_id, Some(5));
    assert!(reg.form_field_data.is_some());

    let form_data = reg.form_field_data.as_ref().unwrap();
    assert_eq!(form_data["department"], "Engineering");
    assert_eq!(form_data["priority"], "high");
}

#[tokio::test]
async fn test_list_inbox_uploads_with_clickwrap() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/inbox_uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "inbox_registration": {
                    "code": "clickwrap-001",
                    "name": "Agreement Signer",
                    "email": "signer@example.com",
                    "clickwrap_body": "I agree to the terms and conditions...",
                    "created_at": "2024-01-22T13:00:00Z"
                },
                "path": "legal/signed-doc.pdf",
                "created_at": "2024-01-22T13:10:00Z"
            }
        ])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = InboxUploadHandler::new(client);
    let (uploads, _) = handler
        .list(None, None, None, None, None, None)
        .await
        .unwrap();

    assert_eq!(uploads.len(), 1);
    let reg = uploads[0].inbox_registration.as_ref().unwrap();
    assert!(reg.clickwrap_body.is_some());
    assert!(
        reg.clickwrap_body
            .as_ref()
            .unwrap()
            .contains("terms and conditions")
    );
}

#[tokio::test]
async fn test_list_inbox_uploads_empty() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/inbox_uploads"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([])))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = InboxUploadHandler::new(client);
    let (uploads, _) = handler
        .list(None, None, None, None, None, None)
        .await
        .unwrap();

    assert_eq!(uploads.len(), 0);
}

#[tokio::test]
async fn test_list_inbox_uploads_authentication_failed() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/inbox_uploads"))
        .respond_with(ResponseTemplate::new(401).set_body_json(serde_json::json!({
            "error": "Authentication failed - Invalid API key",
            "http-code": 401
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("invalid-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = InboxUploadHandler::new(client);
    let result = handler.list(None, None, None, None, None, None).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_list_inbox_uploads_forbidden() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/inbox_uploads"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "error": "Forbidden - Insufficient permissions to view inbox uploads",
            "http-code": 403
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("limited-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = InboxUploadHandler::new(client);
    let result = handler.list(None, None, None, None, None, None).await;

    assert!(result.is_err());
}
