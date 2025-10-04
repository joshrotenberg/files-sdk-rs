//! Storage management module
//!
//! This module contains handlers for storage and resource management:
//! - Projects and priorities
//! - Locks and restores
//! - Snapshots (bandwidth, usage, daily)

pub mod bandwidth_snapshots;
pub mod locks;
pub mod priorities;
pub mod projects;
pub mod remote_bandwidth_snapshots;
pub mod restores;
pub mod snapshots;
pub mod usage_daily_snapshots;
pub mod usage_snapshots;

// Re-export handlers
pub use bandwidth_snapshots::BandwidthSnapshotHandler;
pub use locks::LockHandler;
pub use priorities::PriorityHandler;
pub use projects::ProjectHandler;
pub use remote_bandwidth_snapshots::RemoteBandwidthSnapshotHandler;
pub use restores::RestoreHandler;
pub use snapshots::SnapshotHandler;
pub use usage_daily_snapshots::UsageDailySnapshotHandler;
pub use usage_snapshots::UsageSnapshotHandler;

// Re-export entities
pub use bandwidth_snapshots::BandwidthSnapshotEntity;
pub use locks::LockEntity;
pub use priorities::PriorityEntity;
pub use projects::ProjectEntity;
pub use remote_bandwidth_snapshots::RemoteBandwidthSnapshotEntity;
pub use restores::RestoreEntity;
pub use snapshots::SnapshotEntity;
pub use usage_daily_snapshots::UsageDailySnapshotEntity;
pub use usage_snapshots::UsageSnapshotEntity;
