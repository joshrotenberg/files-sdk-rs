//! Remote Server operations
//!
//! Remote Servers represent connections to external storage providers like S3, Azure,
//! FTP/SFTP servers, and more for syncing or mounting.

use crate::{FilesClient, PaginationInfo, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// A Remote Server entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RemoteServerEntity {
    /// Remote server ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<i64>,

    /// Server type (e.g., s3, azure, ftp, sftp)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_type: Option<String>,

    /// Server name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Authentication method
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authentication_method: Option<String>,

    /// Hostname (for FTP/SFTP)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hostname: Option<String>,

    /// Port (for FTP/SFTP)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub port: Option<i64>,

    /// Username
    #[serde(skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,

    /// Remote home path
    #[serde(skip_serializing_if = "Option::is_none")]
    pub remote_home_path: Option<String>,

    /// Use SSL
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl: Option<String>,

    /// Max connections
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_connections: Option<i64>,

    /// Pin to site region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin_to_site_region: Option<bool>,

    /// Pinned region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pinned_region: Option<String>,

    /// S3 bucket name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_bucket: Option<String>,

    /// S3 region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_region: Option<String>,

    /// AWS access key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aws_access_key: Option<String>,

    /// Server host key (SSH)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_host_key: Option<String>,

    /// Server certificate
    #[serde(skip_serializing_if = "Option::is_none")]
    pub server_certificate: Option<String>,

    /// Azure Blob storage account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_blob_storage_account: Option<String>,

    /// Azure Blob storage container
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_blob_storage_container: Option<String>,

    /// Azure Blob storage DNS suffix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_blob_storage_dns_suffix: Option<String>,

    /// Azure Blob hierarchical namespace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_blob_storage_hierarchical_namespace: Option<bool>,

    /// Azure Files storage account
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_files_storage_account: Option<String>,

    /// Azure Files storage share name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_files_storage_share_name: Option<String>,

    /// Azure Files storage DNS suffix
    #[serde(skip_serializing_if = "Option::is_none")]
    pub azure_files_storage_dns_suffix: Option<String>,

    /// Backblaze B2 bucket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backblaze_b2_bucket: Option<String>,

    /// Backblaze B2 S3 endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub backblaze_b2_s3_endpoint: Option<String>,

    /// Wasabi bucket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wasabi_bucket: Option<String>,

    /// Wasabi region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wasabi_region: Option<String>,

    /// Wasabi access key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wasabi_access_key: Option<String>,

    /// Google Cloud Storage bucket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_cloud_storage_bucket: Option<String>,

    /// Google Cloud Storage project ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_cloud_storage_project_id: Option<String>,

    /// S3-compatible access key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_compatible_access_key: Option<String>,

    /// S3-compatible bucket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_compatible_bucket: Option<String>,

    /// S3-compatible endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_compatible_endpoint: Option<String>,

    /// S3-compatible region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub s3_compatible_region: Option<String>,

    /// Files Agent API token
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_agent_api_token: Option<String>,

    /// Files Agent root
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_agent_root: Option<String>,

    /// Files Agent permission set
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_agent_permission_set: Option<String>,

    /// Files Agent version
    #[serde(skip_serializing_if = "Option::is_none")]
    pub files_agent_version: Option<String>,

    /// Filebase bucket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filebase_bucket: Option<String>,

    /// Filebase access key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filebase_access_key: Option<String>,

    /// Cloudflare bucket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudflare_bucket: Option<String>,

    /// Cloudflare access key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudflare_access_key: Option<String>,

    /// Cloudflare endpoint
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cloudflare_endpoint: Option<String>,

    /// Dropbox teams
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dropbox_teams: Option<bool>,

    /// Linode bucket
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linode_bucket: Option<String>,

    /// Linode access key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linode_access_key: Option<String>,

    /// Linode region
    #[serde(skip_serializing_if = "Option::is_none")]
    pub linode_region: Option<String>,

    /// OneDrive account type
    #[serde(skip_serializing_if = "Option::is_none")]
    pub one_drive_account_type: Option<String>,

    /// Server disabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled: Option<bool>,

    /// Supports versioning
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supports_versioning: Option<bool>,

    /// Enable dedicated IPs
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enable_dedicated_ips: Option<bool>,

    /// Auth account name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_account_name: Option<String>,

    /// Auth status
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auth_status: Option<String>,

    /// Google Cloud Storage S3-compatible access key
    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_cloud_storage_s3_compatible_access_key: Option<String>,
}

/// Handler for remote server operations
pub struct RemoteServerHandler {
    client: FilesClient,
}

impl RemoteServerHandler {
    /// Create a new remote server handler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// List remote servers
    ///
    /// # Arguments
    /// * `cursor` - Pagination cursor
    /// * `per_page` - Results per page
    ///
    /// # Returns
    /// Tuple of (remote_servers, pagination_info)
    ///
    /// # Example
    /// ```no_run
    /// use files_sdk::{FilesClient, RemoteServerHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = RemoteServerHandler::new(client);
    /// let (servers, _) = handler.list(None, None).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn list(
        &self,
        cursor: Option<&str>,
        per_page: Option<i64>,
    ) -> Result<(Vec<RemoteServerEntity>, PaginationInfo)> {
        let mut params = vec![];
        if let Some(c) = cursor {
            params.push(("cursor", c.to_string()));
        }
        if let Some(pp) = per_page {
            params.push(("per_page", pp.to_string()));
        }

        let query = if params.is_empty() {
            String::new()
        } else {
            format!(
                "?{}",
                params
                    .iter()
                    .map(|(k, v)| format!("{}={}", k, v))
                    .collect::<Vec<_>>()
                    .join("&")
            )
        };

        let response = self
            .client
            .get_raw(&format!("/remote_servers{}", query))
            .await?;
        let servers: Vec<RemoteServerEntity> = serde_json::from_value(response)?;

        let pagination = PaginationInfo {
            cursor_next: None,
            cursor_prev: None,
        };

        Ok((servers, pagination))
    }

    /// Get a specific remote server
    ///
    /// # Arguments
    /// * `id` - Remote server ID
    pub async fn get(&self, id: i64) -> Result<RemoteServerEntity> {
        let response = self
            .client
            .get_raw(&format!("/remote_servers/{}", id))
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Get remote server configuration file
    ///
    /// # Arguments
    /// * `id` - Remote server ID
    pub async fn get_configuration_file(&self, id: i64) -> Result<serde_json::Value> {
        let response = self
            .client
            .get_raw(&format!("/remote_servers/{}/configuration_file", id))
            .await?;
        Ok(response)
    }

    /// Create a new remote server
    ///
    /// # Arguments
    /// * `name` - Server name (required)
    /// * `server_type` - Server type (required)
    /// * `hostname` - Hostname (for FTP/SFTP)
    /// * `username` - Username
    /// * `port` - Port
    /// * `s3_bucket` - S3 bucket name
    /// * `s3_region` - S3 region
    ///
    /// # Returns
    /// The created remote server
    #[allow(clippy::too_many_arguments)]
    pub async fn create(
        &self,
        name: &str,
        server_type: &str,
        hostname: Option<&str>,
        username: Option<&str>,
        port: Option<i64>,
        s3_bucket: Option<&str>,
        s3_region: Option<&str>,
    ) -> Result<RemoteServerEntity> {
        let mut request_body = json!({
            "name": name,
            "server_type": server_type,
        });

        if let Some(h) = hostname {
            request_body["hostname"] = json!(h);
        }
        if let Some(u) = username {
            request_body["username"] = json!(u);
        }
        if let Some(p) = port {
            request_body["port"] = json!(p);
        }
        if let Some(b) = s3_bucket {
            request_body["s3_bucket"] = json!(b);
        }
        if let Some(r) = s3_region {
            request_body["s3_region"] = json!(r);
        }

        let response = self
            .client
            .post_raw("/remote_servers", request_body)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Update a remote server
    ///
    /// # Arguments
    /// * `id` - Remote server ID
    /// * `name` - New server name
    /// * `hostname` - New hostname
    /// * `port` - New port
    /// * `disabled` - Disable the server
    ///
    /// # Returns
    /// The updated remote server
    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        &self,
        id: i64,
        name: Option<&str>,
        hostname: Option<&str>,
        port: Option<i64>,
        disabled: Option<bool>,
    ) -> Result<RemoteServerEntity> {
        let mut request_body = json!({});

        if let Some(n) = name {
            request_body["name"] = json!(n);
        }
        if let Some(h) = hostname {
            request_body["hostname"] = json!(h);
        }
        if let Some(p) = port {
            request_body["port"] = json!(p);
        }
        if let Some(d) = disabled {
            request_body["disabled"] = json!(d);
        }

        let response = self
            .client
            .patch_raw(&format!("/remote_servers/{}", id), request_body)
            .await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Delete a remote server
    ///
    /// # Arguments
    /// * `id` - Remote server ID
    pub async fn delete(&self, id: i64) -> Result<()> {
        self.client
            .delete_raw(&format!("/remote_servers/{}", id))
            .await?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = RemoteServerHandler::new(client);
    }
}
