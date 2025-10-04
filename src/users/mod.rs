//! User authentication and access control module
//!
//! This module contains handlers for user management, authentication, and access control:
//! - User CRUD operations
//! - API key management
//! - Session management
//! - Groups and permissions
//! - Public key management
//! - SSO strategies
//! - User lifecycle rules
//! - User activity tracking

#[allow(clippy::module_inception)]
pub mod api_key;
pub mod api_keys;
pub mod group_users;
pub mod groups;
pub mod permissions;
pub mod public_keys;
pub mod sessions;
pub mod sso_strategies;
pub mod user;
pub mod user_cipher_uses;
pub mod user_lifecycle_rules;
pub mod user_requests;
pub mod user_sftp_client_uses;
#[allow(clippy::module_inception)]
pub mod users;

// Re-export handlers
pub use api_key::ApiKeyCurrentHandler;
pub use api_keys::ApiKeyHandler;
pub use group_users::GroupUserHandler;
pub use groups::GroupHandler;
pub use permissions::PermissionHandler;
pub use public_keys::PublicKeyHandler;
pub use sessions::SessionHandler;
pub use sso_strategies::SsoStrategyHandler;
pub use user::CurrentUserHandler;
pub use user_cipher_uses::UserCipherUseHandler;
pub use user_lifecycle_rules::UserLifecycleRuleHandler;
pub use user_requests::UserRequestHandler;
pub use user_sftp_client_uses::UserSftpClientUseHandler;
pub use users::UserHandler;

// Re-export entities
pub use api_key::ApiKeyCurrentEntity;
pub use api_keys::ApiKeyEntity;
pub use group_users::GroupUserEntity;
pub use groups::GroupEntity;
pub use permissions::{PermissionEntity, PermissionType};
pub use public_keys::PublicKeyEntity;
pub use sessions::SessionEntity;
pub use sso_strategies::SsoStrategyEntity;
pub use user::UserEntity as CurrentUserEntity;
pub use user_cipher_uses::UserCipherUseEntity;
pub use user_lifecycle_rules::UserLifecycleRuleEntity;
pub use user_requests::UserRequestEntity;
pub use user_sftp_client_uses::UserSftpClientUseEntity;
pub use users::UserEntity;
