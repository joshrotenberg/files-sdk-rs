//! Wiremock HTTP Testing Example
//!
//! Demonstrates using wiremock to create HTTP mock servers for testing.
//!
//! This approach is useful when you want to:
//! - Test actual HTTP interactions without hitting the real API
//! - Verify request headers, body, and parameters
//! - Simulate various API responses (success, errors, edge cases)
//! - Test the full request/response cycle
//!
//! Run tests with:
//! ```bash
//! cargo test --example wiremock_example
//! ```

#[cfg(test)]
mod tests {
    use files_sdk::FilesClient;
    use wiremock::matchers::{header, header_exists, method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[tokio::test]
    async fn test_file_list_with_mock_server() {
        // Arrange: Start a mock HTTP server
        let mock_server = MockServer::start().await;

        // Configure mock response for folder listing
        let response_body = serde_json::json!([
            {
                "path": "/test/file1.txt",
                "display_name": "file1.txt",
                "type": "file",
                "size": 1024,
                "mtime": "2025-01-01T00:00:00Z"
            },
            {
                "path": "/test/file2.txt",
                "display_name": "file2.txt",
                "type": "file",
                "size": 2048,
                "mtime": "2025-01-01T00:00:00Z"
            }
        ]);

        Mock::given(method("GET"))
            .and(path("/folders/test"))
            .and(header("X-FilesAPI-Key", "test-api-key"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&response_body))
            .mount(&mock_server)
            .await;

        // Act: Create client pointing to mock server
        let client = FilesClient::builder()
            .api_key("test-api-key")
            .base_url(mock_server.uri())
            .build()
            .unwrap();

        let result = client.get_raw("/folders/test").await;

        // Assert
        assert!(result.is_ok());
        let value = result.unwrap();
        assert!(value.is_array());
        let files = value.as_array().unwrap();
        assert_eq!(files.len(), 2);
        assert_eq!(files[0]["display_name"], "file1.txt");
        assert_eq!(files[1]["display_name"], "file2.txt");
    }

    #[tokio::test]
    async fn test_file_upload_with_mock_server() {
        // Arrange
        let mock_server = MockServer::start().await;

        // Mock the begin_upload endpoint
        let upload_response = serde_json::json!({
            "upload_uri": format!("{}/upload-endpoint", mock_server.uri()),
            "http_method": "PUT",
            "ref": "test-ref-123"
        });

        Mock::given(method("POST"))
            .and(path("/file_actions/begin_upload/test/upload.txt"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&upload_response))
            .mount(&mock_server)
            .await;

        // Mock the actual upload endpoint
        Mock::given(method("PUT"))
            .and(path("/upload-endpoint"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&mock_server)
            .await;

        // Mock the finalize endpoint
        let finalize_response = serde_json::json!({
            "path": "/test/upload.txt",
            "type": "file",
            "size": 13
        });

        Mock::given(method("POST"))
            .and(path("/files/test/upload.txt"))
            .respond_with(ResponseTemplate::new(200).set_body_json(&finalize_response))
            .mount(&mock_server)
            .await;

        // Act
        let client = FilesClient::builder()
            .api_key("test-api-key")
            .base_url(mock_server.uri())
            .build()
            .unwrap();

        let result = client
            .post_raw(
                "/file_actions/begin_upload/test/upload.txt",
                serde_json::json!({}),
            )
            .await;

        // Assert
        assert!(result.is_ok());
        let value = result.unwrap();
        assert_eq!(value["ref"], "test-ref-123");
    }

    #[tokio::test]
    async fn test_handles_api_error() {
        // Arrange
        let mock_server = MockServer::start().await;

        // Mock a 404 Not Found response
        let error_response = serde_json::json!({
            "error": "Not Found",
            "message": "File not found"
        });

        Mock::given(method("GET"))
            .and(path("/files/nonexistent.txt"))
            .respond_with(ResponseTemplate::new(404).set_body_json(&error_response))
            .mount(&mock_server)
            .await;

        // Act
        let client = FilesClient::builder()
            .api_key("test-api-key")
            .base_url(mock_server.uri())
            .build()
            .unwrap();

        let result = client.get_raw("/files/nonexistent.txt").await;

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Not Found"));
    }

    #[tokio::test]
    async fn test_handles_rate_limiting() {
        // Arrange
        let mock_server = MockServer::start().await;

        // First request: Return 429 Too Many Requests
        Mock::given(method("GET"))
            .and(path("/files/test.txt"))
            .respond_with(
                ResponseTemplate::new(429)
                    .set_body_json(serde_json::json!({
                        "error": "Too Many Requests",
                        "message": "Rate limit exceeded"
                    }))
                    .insert_header("Retry-After", "1"),
            )
            .expect(1..)
            .mount(&mock_server)
            .await;

        // Act
        let client = FilesClient::builder()
            .api_key("test-api-key")
            .base_url(mock_server.uri())
            .max_retries(0) // Disable retries for this test
            .build()
            .unwrap();

        let result = client.get_raw("/files/test.txt").await;

        // Assert
        assert!(result.is_err());
        let error = result.unwrap_err();
        assert!(error.to_string().contains("Rate"));
    }

    #[tokio::test]
    async fn test_validates_request_headers() {
        // Arrange
        let mock_server = MockServer::start().await;

        // Verify the client sends correct headers
        Mock::given(method("GET"))
            .and(path("/files/test.txt"))
            .and(header("X-FilesAPI-Key", "test-api-key"))
            .and(header_exists("User-Agent"))
            .respond_with(ResponseTemplate::new(200).set_body_json(serde_json::json!({})))
            .expect(1)
            .mount(&mock_server)
            .await;

        // Act
        let client = FilesClient::builder()
            .api_key("test-api-key")
            .base_url(mock_server.uri())
            .build()
            .unwrap();

        let result = client.get_raw("/files/test.txt").await;

        // Assert
        assert!(result.is_ok());
        // Mock will verify headers were sent correctly
    }
}

fn main() {
    println!("Wiremock HTTP Testing Example");
    println!("==============================\n");
    println!("This example demonstrates HTTP mocking with wiremock.");
    println!("\nRun the tests with:");
    println!("  cargo test --example wiremock_example");
    println!("\nKey concepts:");
    println!("- Start a MockServer for each test");
    println!("- Configure mock endpoints with Mock::given()");
    println!("- Point your client to mock_server.uri()");
    println!("- Verify request headers, methods, and paths");
    println!("- Simulate various API responses and errors");
}
