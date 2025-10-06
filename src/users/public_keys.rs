//! Public Key operations for SSH key management
//!
//! This module provides operations for managing SSH public keys:
//! - List public keys for a user
//! - Create new public keys (upload or generate)
//! - Update public key metadata
//! - Delete public keys

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// Represents a public SSH key in Files.com
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PublicKeyEntity {
    /// Public key ID
    pub id: Option<i64>,

    /// Public key title
    pub title: Option<String>,

    /// User ID this public key is associated with
    pub user_id: Option<i64>,

    /// Username of the user this public key is associated with
    pub username: Option<String>,

    /// Public key fingerprint (MD5)
    pub fingerprint: Option<String>,

    /// Public key fingerprint (SHA256)
    pub fingerprint_sha256: Option<String>,

    /// Public key created at date/time
    pub created_at: Option<String>,

    /// Key's most recent login time via SFTP
    pub last_login_at: Option<String>,

    /// Only returned when generating keys. Can be invalid, not_generated, generating, complete
    pub status: Option<String>,

    /// Only returned when generating keys. Private key generated for the user
    pub generated_private_key: Option<String>,

    /// Only returned when generating keys. Public key generated for the user
    pub generated_public_key: Option<String>,
}

/// Handler for public key operations
#[derive(Debug, Clone)]
pub struct PublicKeyHandler {
    client: FilesClient,
}

impl PublicKeyHandler {
    /// Creates a new PublicKeyHandler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List public keys
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID (use 0 for current user, None for all users if admin)
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Number of records per page
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// # use files_sdk::{FilesClient, PublicKeyHandler};
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = PublicKeyHandler::new(client);
    ///
    /// // List keys for current user
    /// let (keys, pagination) = handler.list(Some(0), None, None).await?;
    /// for key in keys {
    ///     println!("{}: {}", key.title.unwrap_or_default(), key.fingerprint.unwrap_or_default());
    /// }
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        user_id: Option<i64>,
        cursor: Option<String>,
        per_page: Option<i64>,
    ) -> Result<(Vec<PublicKeyEntity>, PaginationInfo)> {
        let mut endpoint = "/public_keys".to_string();
        let mut query_params = Vec::new();

        if let Some(user_id) = user_id {
            query_params.push(format!("user_id={}", user_id));
        }

        if let Some(cursor) = cursor {
            query_params.push(format!("cursor={}", cursor));
        }

        if let Some(per_page) = per_page {
            query_params.push(format!("per_page={}", per_page));
        }

        if !query_params.is_empty() {
            endpoint.push('?');
            endpoint.push_str(&query_params.join("&"));
        }

        let url = format!("{}{}", self.client.inner.base_url, endpoint);
        let response = reqwest::Client::new()
            .get(&url)
            .header("X-FilesAPI-Key", &self.client.inner.api_key)
            .send()
            .await?;

        let headers = response.headers().clone();
        let pagination = PaginationInfo::from_headers(&headers);

        let status = response.status();
        if !status.is_success() {
            return Err(crate::FilesError::ApiError { endpoint: None,
                code: status.as_u16(),
                message: response.text().await.unwrap_or_default(),
            });
        }

        let keys: Vec<PublicKeyEntity> = response.json().await?;
        Ok((keys, pagination))
    }

    /// Get a specific public key by ID
    ///
    /// # Arguments
    ///
    /// * `id` - Public key ID
    pub async fn get(&self, id: i64) -> Result<PublicKeyEntity> {
        let endpoint = format!("/public_keys/{}", id);
        let response = self.client.get_raw(&endpoint).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Create a new public key by uploading an existing key
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID (use 0 for current user)
    /// * `title` - Internal reference for key
    /// * `public_key` - Actual contents of SSH key
    pub async fn create(
        &self,
        user_id: i64,
        title: &str,
        public_key: &str,
    ) -> Result<PublicKeyEntity> {
        let body = json!({
            "user_id": user_id,
            "title": title,
            "public_key": public_key,
        });

        let response = self.client.post_raw("/public_keys", body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Generate a new SSH key pair
    ///
    /// # Arguments
    ///
    /// * `user_id` - User ID (use 0 for current user)
    /// * `title` - Internal reference for key
    /// * `algorithm` - Type of key (rsa, dsa, ecdsa, ed25519)
    /// * `length` - Length of key (or signature size for ecdsa)
    /// * `password` - Password for the private key (optional)
    pub async fn generate(
        &self,
        user_id: i64,
        title: &str,
        algorithm: &str,
        length: Option<i64>,
        password: Option<&str>,
    ) -> Result<PublicKeyEntity> {
        let mut body = json!({
            "user_id": user_id,
            "title": title,
            "generate_keypair": true,
            "generate_algorithm": algorithm,
        });

        if let Some(length) = length {
            body["generate_length"] = json!(length);
        }

        if let Some(password) = password {
            body["generate_private_key_password"] = json!(password);
        }

        let response = self.client.post_raw("/public_keys", body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Update a public key's title
    ///
    /// # Arguments
    ///
    /// * `id` - Public key ID
    /// * `title` - New title for the key
    pub async fn update(&self, id: i64, title: &str) -> Result<PublicKeyEntity> {
        let body = json!({
            "title": title,
        });

        let endpoint = format!("/public_keys/{}", id);
        let response = self.client.patch_raw(&endpoint, body).await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete a public key
    ///
    /// # Arguments
    ///
    /// * `id` - Public key ID
    pub async fn delete(&self, id: i64) -> Result<()> {
        let endpoint = format!("/public_keys/{}", id);
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
        let _handler = PublicKeyHandler::new(client);
    }
}
