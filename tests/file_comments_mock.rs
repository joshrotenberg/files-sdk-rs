use files_sdk::{FileCommentHandler, FilesClient};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path},
};

async fn setup() -> (MockServer, FileCommentHandler) {
    let mock_server = MockServer::start().await;
    let client = FilesClient::builder()
        .api_key("test-key")
        .base_url(mock_server.uri())
        .build()
        .unwrap();
    let handler = FileCommentHandler::new(client);
    (mock_server, handler)
}

#[tokio::test]
async fn test_list_file_comments() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("GET"))
        .and(path("/file_comments/files/test.txt"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!([
            {
                "id": 1,
                "body": "Great work on this file!",
                "reactions": [
                    {"id": 10, "emoji": "👍"},
                    {"id": 11, "emoji": "🎉"}
                ]
            },
            {
                "id": 2,
                "body": "Needs some updates",
                "reactions": []
            }
        ])))
        .mount(&mock_server)
        .await;

    let comments = handler.list("/test.txt").await.unwrap();
    assert_eq!(comments.len(), 2);
    assert_eq!(
        comments[0].body,
        Some("Great work on this file!".to_string())
    );
    assert_eq!(comments[0].reactions.as_ref().unwrap().len(), 2);
}

#[tokio::test]
async fn test_create_file_comment() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("POST"))
        .and(path("/file_comments"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 5,
            "body": "New comment",
            "reactions": []
        })))
        .mount(&mock_server)
        .await;

    let comment = handler.create("/test.txt", "New comment").await.unwrap();
    assert_eq!(comment.id, Some(5));
    assert_eq!(comment.body, Some("New comment".to_string()));
}

#[tokio::test]
async fn test_update_file_comment() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("PATCH"))
        .and(path("/file_comments/1"))
        .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({
            "id": 1,
            "body": "Updated comment text",
            "reactions": []
        })))
        .mount(&mock_server)
        .await;

    let comment = handler.update(1, "Updated comment text").await.unwrap();
    assert_eq!(comment.body, Some("Updated comment text".to_string()));
}

#[tokio::test]
async fn test_delete_file_comment() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("DELETE"))
        .and(path("/file_comments/1"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let result = handler.delete(1).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_add_reaction() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("POST"))
        .and(path("/file_comment_reactions"))
        .respond_with(ResponseTemplate::new(201).set_body_json(serde_json::json!({
            "id": 20,
            "emoji": "❤️"
        })))
        .mount(&mock_server)
        .await;

    let reaction = handler.add_reaction(1, "❤️").await.unwrap();
    assert_eq!(reaction.id, Some(20));
    assert_eq!(reaction.emoji, Some("❤️".to_string()));
}

#[tokio::test]
async fn test_delete_reaction() {
    let (mock_server, handler) = setup().await;

    Mock::given(method("DELETE"))
        .and(path("/file_comment_reactions/20"))
        .respond_with(ResponseTemplate::new(204))
        .mount(&mock_server)
        .await;

    let result = handler.delete_reaction(20).await;
    assert!(result.is_ok());
}
