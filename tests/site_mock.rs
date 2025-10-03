//! Mock tests for SiteHandler

use files_sdk::{FilesClient, SiteHandler};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

#[tokio::test]
async fn test_get_site() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/site"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": "My Company Files",
            "subdomain": "mycompany",
            "domain": "files.mycompany.com",
            "email": "admin@mycompany.com",
            "admin_user_id": 1,
            "allowed_ips": "192.168.1.0/24,10.0.0.0/8",
            "allowed_countries": "US,CA,GB",
            "default_time_zone": "America/New_York",
            "ssl_required": true,
            "require_2fa": true,
            "allowed_2fa_method_totp": true,
            "allowed_2fa_method_webauthn": true,
            "session_expiry": 120.0,
            "user_lockout": true,
            "user_lockout_tries": 5,
            "user_lockout_within": 300,
            "user_lockout_lock_period": 900,
            "created_at": "2024-01-01T00:00:00Z"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = SiteHandler::new(client);
    let site = handler.get().await.unwrap();

    assert_eq!(site.name, Some("My Company Files".to_string()));
    assert_eq!(site.subdomain, Some("mycompany".to_string()));
    assert_eq!(site.ssl_required, Some(true));
    assert_eq!(site.require_2fa, Some(true));
    assert_eq!(site.user_lockout_tries, Some(5));
}

#[tokio::test]
async fn test_get_site_usage() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/site/usage"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "current_storage": 1073741824i64,
            "deleted_files_counted_in_minimum": 100,
            "deleted_files_storage": 52428800,
            "root_storage": 1021313024,
            "total_billable_transfer_used": 5368709120i64,
            "usage_by_top_level_dir": {
                "/uploads": 536870912,
                "/documents": 268435456,
                "/media": 268435456
            }
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = SiteHandler::new(client);
    let usage = handler.get_usage().await.unwrap();

    assert_eq!(usage.current_storage, Some(1073741824));
    assert_eq!(usage.deleted_files_counted_in_minimum, Some(100));
    assert!(usage.usage_by_top_level_dir.is_some());
}

#[tokio::test]
async fn test_update_site_name() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/site"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": "Updated Company Name",
            "subdomain": "mycompany",
            "email": "admin@mycompany.com"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = SiteHandler::new(client);
    let site = handler
        .update(Some("Updated Company Name"), None, None, None, None, None)
        .await
        .unwrap();

    assert_eq!(site.name, Some("Updated Company Name".to_string()));
}

#[tokio::test]
async fn test_update_site_security() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/site"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": "My Company",
            "allowed_ips": "192.168.1.0/24",
            "require_2fa": true
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = SiteHandler::new(client);
    let site = handler
        .update(None, None, None, None, Some("192.168.1.0/24"), Some(true))
        .await
        .unwrap();

    assert_eq!(site.allowed_ips, Some("192.168.1.0/24".to_string()));
    assert_eq!(site.require_2fa, Some(true));
}

#[tokio::test]
async fn test_update_site_domain() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/site"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": "My Company",
            "subdomain": "newsubdomain",
            "domain": "files.newdomain.com"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = SiteHandler::new(client);
    let site = handler
        .update(
            None,
            Some("newsubdomain"),
            Some("files.newdomain.com"),
            None,
            None,
            None,
        )
        .await
        .unwrap();

    assert_eq!(site.subdomain, Some("newsubdomain".to_string()));
    assert_eq!(site.domain, Some("files.newdomain.com".to_string()));
}

#[tokio::test]
async fn test_update_site_email() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/site"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": "My Company",
            "email": "newemail@company.com"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = SiteHandler::new(client);
    let site = handler
        .update(None, None, None, Some("newemail@company.com"), None, None)
        .await
        .unwrap();

    assert_eq!(site.email, Some("newemail@company.com".to_string()));
}

#[tokio::test]
async fn test_get_site_with_password_requirements() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/site"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": "Secure Company",
            "password_min_length": 12,
            "password_require_letter": true,
            "password_require_mixed": true,
            "password_require_number": true,
            "password_require_special": true,
            "password_require_unbreached": true,
            "password_validity_days": 90,
            "max_prior_passwords": 5
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = SiteHandler::new(client);
    let site = handler.get().await.unwrap();

    assert_eq!(site.password_min_length, Some(12));
    assert_eq!(site.password_require_special, Some(true));
    assert_eq!(site.password_validity_days, Some(90));
}

#[tokio::test]
async fn test_get_site_with_bundle_settings() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/site"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "name": "Bundle Site",
            "bundle_expiration": 7,
            "days_to_retain_backups": 30
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = SiteHandler::new(client);
    let site = handler.get().await.unwrap();

    assert_eq!(site.bundle_expiration, Some(7));
    assert_eq!(site.days_to_retain_backups, Some(30));
}

#[tokio::test]
async fn test_get_site_authentication_failed() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/site"))
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

    let handler = SiteHandler::new(client);
    let result = handler.get().await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_update_site_forbidden() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/site"))
        .respond_with(ResponseTemplate::new(403).set_body_json(serde_json::json!({
            "error": "Forbidden - Insufficient permissions to modify site settings",
            "http-code": 403
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("limited-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = SiteHandler::new(client);
    let result = handler
        .update(Some("New Name"), None, None, None, None, None)
        .await;

    assert!(result.is_err());
}
