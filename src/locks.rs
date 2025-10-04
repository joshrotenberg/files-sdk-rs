//! File locking operations

use crate::{FilesClient, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Represents a file lock
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct LockEntity {
    pub path: Option<String>,
    pub timeout: Option<i64>,
    pub depth: Option<String>,
    pub owner: Option<String>,
    pub scope: Option<String>,
    pub token: Option<String>,
    #[serde(rename = "type")]
    pub lock_type: Option<String>,
    pub user_id: Option<i64>,
    pub username: Option<String>,
}

#[derive(Debug, Clone)]
pub struct LockHandler {
    client: FilesClient,
}

impl LockHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn list_for_path(
        &self,
        path: &str,
        include_children: bool,
    ) -> Result<Vec<LockEntity>> {
        let endpoint = format!("/locks{}?include_children={}", path, include_children);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn create(&self, path: &str, timeout: Option<i64>) -> Result<LockEntity> {
        let mut body = json!({"path": path});
        if let Some(t) = timeout {
            body["timeout"] = json!(t);
        }

        let endpoint = format!("/locks{}", path);
        let response = self.client.post_raw(&endpoint, body).await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn delete(&self, path: &str, token: &str) -> Result<()> {
        let endpoint = format!("/locks{}?token={}", path, token);
        self.client.delete_raw(&endpoint).await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = LockHandler::new(client);
    }
}
