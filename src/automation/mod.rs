//! Automation and integration module
//!
//! This module contains handlers for automating file operations and integrations:
//! - Automations and automation runs
//! - Behaviors (webhooks, auto-encrypt, etc.)
//! - Remote servers and syncs
//! - Remote mount backends

pub mod automation_runs;
pub mod automations;
pub mod behaviors;
pub mod remote_mount_backends;
pub mod remote_servers;
pub mod sync_runs;
pub mod syncs;

// Re-export handlers
pub use automation_runs::AutomationRunHandler;
pub use automations::AutomationHandler;
pub use behaviors::BehaviorHandler;
pub use remote_mount_backends::RemoteMountBackendHandler;
pub use remote_servers::RemoteServerHandler;
pub use sync_runs::SyncRunHandler;
pub use syncs::SyncHandler;

// Re-export entities
pub use automation_runs::AutomationRunEntity;
pub use automations::{AutomationEntity, AutomationTrigger, AutomationType};
pub use behaviors::BehaviorEntity;
pub use remote_mount_backends::RemoteMountBackendEntity;
pub use remote_servers::RemoteServerEntity;
pub use sync_runs::SyncRunEntity;
pub use syncs::SyncEntity;
