//! Site operations
//!
//! Site represents site-wide settings and configuration for your Files.com account.

use crate::{FilesClient, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;

/// A Site entity (site-wide settings)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteEntity {
    /// Site name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Admin user ID
    #[serde(skip_serializing_if = "Option::is_none")]
    pub admin_user_id: Option<i64>,

    /// Allowed IP addresses (whitelist)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_ips: Option<String>,

    /// Allowed countries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_countries: Option<String>,

    /// Disallowed countries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disallowed_countries: Option<String>,

    /// Default time zone
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_time_zone: Option<String>,

    /// Domain (custom domain)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain: Option<String>,

    /// Email (contact email)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Session expiry (minutes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_expiry: Option<f64>,

    /// SSL required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ssl_required: Option<bool>,

    /// Subdomain
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subdomain: Option<String>,

    /// Welcome email enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub welcome_email_enabled: Option<bool>,

    /// User lockout enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_lockout: Option<bool>,

    /// User lockout tries
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_lockout_tries: Option<i64>,

    /// User lockout within (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_lockout_within: Option<i64>,

    /// User lockout lock period (seconds)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_lockout_lock_period: Option<i64>,

    /// Require 2FA
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_2fa: Option<bool>,

    /// Allowed 2FA methods
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_2fa_method_sms: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_2fa_method_totp: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_2fa_method_webauthn: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_2fa_method_yubi: Option<bool>,

    /// Site currency
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<String>,

    /// Session pinned by IP
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session_pinned_by_ip: Option<bool>,

    /// Bundle expiration (days)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bundle_expiration: Option<i64>,

    /// Days to retain backups
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_to_retain_backups: Option<i64>,

    /// Max prior passwords
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_prior_passwords: Option<i64>,

    /// Password validity days
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_validity_days: Option<i64>,

    /// Password min length
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_min_length: Option<i64>,

    /// Password require letter
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_require_letter: Option<bool>,

    /// Password require mixed case
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_require_mixed: Option<bool>,

    /// Password require number
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_require_number: Option<bool>,

    /// Password require special character
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_require_special: Option<bool>,

    /// Password require unbreached
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_require_unbreached: Option<bool>,

    /// SFTP user root enabled
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sftp_user_root_enabled: Option<bool>,

    /// Disable password reset
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disable_password_reset: Option<bool>,

    /// Site created at
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<String>,

    /// Custom namespace
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_namespace: Option<bool>,
}

/// Site usage statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SiteUsageEntity {
    /// Current storage usage (bytes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_storage: Option<i64>,

    /// Deleted files count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_files_counted_in_minimum: Option<i64>,

    /// Deleted files storage (bytes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted_files_storage: Option<i64>,

    /// Root storage (bytes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub root_storage: Option<i64>,

    /// Total billable transfer (bytes)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_billable_transfer_used: Option<i64>,

    /// Usage by top-level directory
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage_by_top_level_dir: Option<serde_json::Value>,
}

/// Handler for site operations
pub struct SiteHandler {
    client: FilesClient,
}

impl SiteHandler {
    /// Create a new site handler
    pub fn new(client: FilesClient) -> Self {
        Self { client }
    }

    /// Get site settings
    ///
    /// # Example
    /// ```no_run
    /// use files_sdk::{FilesClient, SiteHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = SiteHandler::new(client);
    /// let site = handler.get().await?;
    /// println!("Site: {}", site.name.unwrap_or_default());
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&self) -> Result<SiteEntity> {
        let response = self.client.get_raw("/site").await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Get site usage statistics
    ///
    /// # Example
    /// ```no_run
    /// use files_sdk::{FilesClient, SiteHandler};
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let client = FilesClient::builder().api_key("key").build()?;
    /// let handler = SiteHandler::new(client);
    /// let usage = handler.get_usage().await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get_usage(&self) -> Result<SiteUsageEntity> {
        let response = self.client.get_raw("/site/usage").await?;
        Ok(serde_json::from_value(response)?)
    }

    /// Update site settings
    ///
    /// # Arguments
    /// * `name` - Site name
    /// * `subdomain` - Site subdomain
    /// * `domain` - Custom domain
    /// * `email` - Contact email
    /// * `allowed_ips` - Allowed IP addresses (comma-separated)
    /// * `require_2fa` - Require 2FA for all users
    ///
    /// # Returns
    /// The updated site entity
    #[allow(clippy::too_many_arguments)]
    pub async fn update(
        &self,
        name: Option<&str>,
        subdomain: Option<&str>,
        domain: Option<&str>,
        email: Option<&str>,
        allowed_ips: Option<&str>,
        require_2fa: Option<bool>,
    ) -> Result<SiteEntity> {
        let mut request_body = json!({});

        if let Some(n) = name {
            request_body["name"] = json!(n);
        }
        if let Some(s) = subdomain {
            request_body["subdomain"] = json!(s);
        }
        if let Some(d) = domain {
            request_body["domain"] = json!(d);
        }
        if let Some(e) = email {
            request_body["email"] = json!(e);
        }
        if let Some(ips) = allowed_ips {
            request_body["allowed_ips"] = json!(ips);
        }
        if let Some(r2fa) = require_2fa {
            request_body["require_2fa"] = json!(r2fa);
        }

        let response = self.client.patch_raw("/site", request_body).await?;
        Ok(serde_json::from_value(response)?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_handler_creation() {
        let client = FilesClient::builder().api_key("test-key").build().unwrap();
        let _handler = SiteHandler::new(client);
    }
}
