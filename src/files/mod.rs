//! File operations module
//!
//! This module contains handlers for file and folder operations including:
//! - File upload, download, and management
//! - Folder listing and manipulation
//! - File actions (copy, move, metadata)
//! - File comments and reactions
//! - File migrations

#[allow(clippy::module_inception)]
pub mod file_actions;
pub mod file_comment_reactions;
pub mod file_comments;
pub mod file_migration_logs;
pub mod file_migrations;
#[allow(clippy::module_inception)]
pub mod files;
pub mod folders;

// Re-export handlers
pub use file_actions::FileActionHandler;
pub use file_comment_reactions::FileCommentReactionHandler;
pub use file_comments::FileCommentHandler;
pub use file_migration_logs::FileMigrationLogHandler;
pub use file_migrations::FileMigrationHandler;
pub use files::FileHandler;
pub use folders::FolderHandler;

// Re-export entities
pub use file_comment_reactions::FileCommentReactionEntity;
pub use file_comments::{
    FileCommentEntity, FileCommentReactionEntity as FileCommentReactionEntity2,
};
pub use file_migration_logs::FileMigrationLogEntity;
pub use file_migrations::FileMigrationEntity;
