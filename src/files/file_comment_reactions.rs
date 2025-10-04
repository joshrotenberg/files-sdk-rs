use crate::{Result, client::FilesClient};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FileCommentReactionEntity {
    #[serde(flatten)]
    pub data: serde_json::Map<String, serde_json::Value>,
}

pub struct FileCommentReactionHandler {
    client: FilesClient,
}

impl FileCommentReactionHandler {
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    pub async fn create(&self, params: serde_json::Value) -> Result<FileCommentReactionEntity> {
        let response = self
            .client
            .post_raw("/file_comment_reactions", params)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    pub async fn delete(&self, id: i64) -> Result<()> {
        let endpoint = format!("/file_comment_reactions/{}", id);
        self.client.delete_raw(&endpoint).await?;
        Ok(())
    }
}
