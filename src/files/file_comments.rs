//! File comment operations
//!
//! This module provides operations for commenting on files:
//! - List comments on a file
//! - Create new comments
//! - Update existing comments
//! - Delete comments
//! - React to comments

use crate::{FilesClient, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Represents a comment on a file
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileCommentEntity {
    /// File comment ID
    pub id: Option<i64>,

    /// Comment body text
    pub body: Option<String>,

    /// Reactions to this comment
    pub reactions: Option<Vec<FileCommentReactionEntity>>,
}

/// Represents a reaction to a file comment
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileCommentReactionEntity {
    /// Reaction ID
    pub id: Option<i64>,

    /// Emoji used for the reaction
    pub emoji: Option<String>,
}

/// Handler for file comment operations
#[derive(Debug, Clone)]
pub struct FileCommentHandler {
    client: FilesClient,
}

impl FileCommentHandler {
    /// Creates a new FileCommentHandler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List comments for a file
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, FileCommentHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = FileCommentHandler::new(client);
    ///
    /// let comments = handler.list("/path/to/file.txt").await?;
    /// for comment in comments {
    ///     println!("{}", comment.body.unwrap_or_default());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(&self, path: &str) -> Result<Vec<FileCommentEntity>> {
        let endpoint = format!("/file_comments/files{}", path);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Create a new file comment
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file
    /// * `body` - Comment text
    pub async fn create(&self, path: &str, body: &str) -> Result<FileCommentEntity> {
        let body_json = json!({
            "body": body,
            "path": path,
        });

        let response = self.client.post_raw("/file_comments", body_json).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Update a file comment
    ///
    /// # Arguments
    ///
    /// * `id` - Comment ID
    /// * `body` - New comment text
    pub async fn update(&self, id: i64, body: &str) -> Result<FileCommentEntity> {
        let body_json = json!({
            "body": body,
        });

        let endpoint = format!("/file_comments/{}", id);
        let response = self.client.patch_raw(&endpoint, body_json).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete a file comment
    ///
    /// # Arguments
    ///
    /// * `id` - Comment ID
    pub async fn delete(&self, id: i64) -> Result<()> {
        let endpoint = format!("/file_comments/{}", id);
        self.client.delete_raw(&endpoint).await?;
        Ok(())
    }

    /// Add a reaction to a file comment
    ///
    /// # Arguments
    ///
    /// * `file_comment_id` - ID of the comment to react to
    /// * `emoji` - Emoji for the reaction
    pub async fn add_reaction(
        &self,
        file_comment_id: i64,
        emoji: &str,
    ) -> Result<FileCommentReactionEntity> {
        let body = json!({
            "file_comment_id": file_comment_id,
            "emoji": emoji,
        });

        let response = self
            .client
            .post_raw("/file_comment_reactions", body)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete a reaction from a file comment
    ///
    /// # Arguments
    ///
    /// * `id` - Reaction ID
    pub async fn delete_reaction(&self, id: i64) -> Result<()> {
        let endpoint = format!("/file_comment_reactions/{}", id);
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
        let _handler = FileCommentHandler::new(client);
    }
}
