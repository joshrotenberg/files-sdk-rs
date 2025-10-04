//! Mock tests for AutomationHandler

use files_sdk::{AutomationHandler, FilesClient};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path, path_regex},
};

#[tokio::test]
async fn test_list_automations() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/automations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 1,
                "automation": "copy_file",
                "name": "Daily Backup",
                "description": "Copy files to backup folder daily",
                "source": "/source/*.txt",
                "destinations": ["/backup/"],
                "interval": "day",
                "trigger": "daily",
                "disabled": false,
                "path": "/source"
            },
            {
                "id": 2,
                "automation": "move_file",
                "name": "Archive Old Files",
                "source": "/temp/*.log",
                "destinations": ["/archive/"],
                "trigger": "action",
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

    let handler = AutomationHandler::new(client);
    let (automations, _) = handler.list(None, None, None).await.unwrap();

    assert_eq!(automations.len(), 2);
    assert_eq!(automations[0].id, Some(1));
    assert_eq!(automations[0].automation, Some("copy_file".to_string()));
    assert_eq!(automations[0].name, Some("Daily Backup".to_string()));
    assert_eq!(automations[1].id, Some(2));
}

#[tokio::test]
async fn test_list_automations_with_filter() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/automations"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 3,
                "automation": "delete_file",
                "name": "Cleanup Old Logs",
                "source": "/logs/*.old",
                "trigger": "daily",
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

    let handler = AutomationHandler::new(client);
    let (automations, _) = handler.list(None, None, Some("delete_file")).await.unwrap();

    assert_eq!(automations.len(), 1);
    assert_eq!(automations[0].automation, Some("delete_file".to_string()));
}

#[tokio::test]
async fn test_get_automation() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/automations/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "automation": "copy_file",
            "name": "Important Backup",
            "description": "Backup critical files every hour",
            "source": "/critical/*.dat",
            "destinations": ["/backup/critical/", "/offsite/backup/"],
            "interval": "day",
            "trigger": "daily",
            "disabled": false,
            "path": "/critical",
            "overwrite_files": true,
            "flatten_destination_structure": false,
            "schedule_times_of_day": ["02:00", "14:00"],
            "schedule_time_zone": "America/New_York"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AutomationHandler::new(client);
    let automation = handler.get(1).await.unwrap();

    assert_eq!(automation.id, Some(1));
    assert_eq!(automation.name, Some("Important Backup".to_string()));
    assert!(automation.overwrite_files.unwrap());
    assert_eq!(automation.destinations.as_ref().unwrap().len(), 2);
}

#[tokio::test]
async fn test_create_automation() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/automations"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 42,
            "automation": "copy_file",
            "source": "/uploads/*.pdf",
            "destinations": ["/processed/"],
            "interval": "day",
            "path": "/uploads",
            "trigger": "action",
            "disabled": false
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AutomationHandler::new(client);
    let automation = handler
        .create(
            "copy_file",
            Some("/uploads/*.pdf"),
            None,
            Some(vec!["/processed/".to_string()]),
            Some("day"),
            Some("/uploads"),
            Some("action"),
        )
        .await
        .unwrap();

    assert_eq!(automation.id, Some(42));
    assert_eq!(automation.automation, Some("copy_file".to_string()));
}

#[tokio::test]
async fn test_create_automation_move_file() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/automations"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 50,
            "automation": "move_file",
            "source": "/inbox/*.zip",
            "destinations": ["/archive/"],
            "trigger": "webhook",
            "webhook_url": "https://example.com/webhook",
            "disabled": false
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AutomationHandler::new(client);
    let automation = handler
        .create(
            "move_file",
            Some("/inbox/*.zip"),
            None,
            Some(vec!["/archive/".to_string()]),
            None,
            None,
            Some("webhook"),
        )
        .await
        .unwrap();

    assert_eq!(automation.automation, Some("move_file".to_string()));
    assert_eq!(automation.trigger, Some("webhook".to_string()));
}

#[tokio::test]
async fn test_update_automation() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/automations/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "automation": "copy_file",
            "source": "/new-source/*.txt",
            "destinations": ["/new-dest/"],
            "interval": "week",
            "disabled": false
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AutomationHandler::new(client);
    let automation = handler
        .update(
            1,
            Some("/new-source/*.txt"),
            Some("/new-dest/"),
            Some("week"),
            None,
        )
        .await
        .unwrap();

    assert_eq!(automation.source, Some("/new-source/*.txt".to_string()));
    assert_eq!(automation.interval, Some("week".to_string()));
}

#[tokio::test]
async fn test_update_automation_disable() {
    let mock_server = MockServer::start().await;

    Mock::given(method("PATCH"))
        .and(path("/automations/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "automation": "copy_file",
            "disabled": true
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AutomationHandler::new(client);
    let automation = handler
        .update(1, None, None, None, Some(true))
        .await
        .unwrap();

    assert_eq!(automation.disabled, Some(true));
}

#[tokio::test]
async fn test_delete_automation() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/automations/1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AutomationHandler::new(client);
    handler.delete(1).await.unwrap();
}

#[tokio::test]
async fn test_manual_run_automation() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/automations/1/manual_run"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "status": "running",
            "automation_run_id": 123,
            "message": "Automation run started successfully"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AutomationHandler::new(client);
    let result = handler.manual_run(1).await.unwrap();

    assert_eq!(result["status"], "running");
    assert_eq!(result["automation_run_id"], 123);
}

#[tokio::test]
async fn test_automation_with_schedule() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path("/automations/5"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 5,
            "automation": "run_sync",
            "name": "Scheduled Sync",
            "trigger": "custom",
            "schedule_days_of_week": [1, 3, 5],
            "schedule_times_of_day": ["09:00", "17:00"],
            "schedule_time_zone": "UTC",
            "human_readable_schedule": "Monday, Wednesday, Friday at 9:00 AM and 5:00 PM UTC"
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AutomationHandler::new(client);
    let automation = handler.get(5).await.unwrap();

    assert_eq!(automation.schedule_days_of_week.as_ref().unwrap().len(), 3);
    assert_eq!(automation.schedule_times_of_day.as_ref().unwrap().len(), 2);
}

#[tokio::test]
async fn test_get_automation_not_found() {
    let mock_server = MockServer::start().await;

    Mock::given(method("GET"))
        .and(path_regex(r"^/automations/\d+$"))
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

    let handler = AutomationHandler::new(client);
    let result = handler.get(999).await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_create_automation_bad_request() {
    let mock_server = MockServer::start().await;

    Mock::given(method("POST"))
        .and(path("/automations"))
        .respond_with(ResponseTemplate::new(400).set_body_json(serde_json::json!({
            "error": "Bad Request - Invalid automation type",
            "http-code": 400
        })))
        .mount(&mock_server)
        .await;

    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();

    let handler = AutomationHandler::new(client);
    let result = handler
        .create("invalid_type", None, None, None, None, None, None)
        .await;

    assert!(result.is_err());
}

#[tokio::test]
async fn test_delete_automation_forbidden() {
    let mock_server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path_regex(r"^/automations/\d+$"))
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

    let handler = AutomationHandler::new(client);
    let result = handler.delete(1).await;

    assert!(result.is_err());
}
