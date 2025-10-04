//! Messaging and notifications module
//!
//! This module contains handlers for messages, comments, and notifications:
//! - Messages and message comments
//! - Message reactions
//! - Notifications

#[allow(clippy::module_inception)]
pub mod message_comment_reactions;
pub mod message_comments;
pub mod message_reactions;
#[allow(clippy::module_inception)]
pub mod messages;
pub mod notifications;

// Re-export handlers
pub use message_comment_reactions::MessageCommentReactionHandler;
pub use message_comments::MessageCommentHandler;
pub use message_reactions::MessageReactionHandler;
pub use messages::MessageHandler;
pub use notifications::NotificationHandler;

// Re-export entities
pub use message_comment_reactions::MessageCommentReactionEntity;
pub use message_comments::MessageCommentEntity;
pub use message_reactions::MessageReactionEntity;
pub use messages::{MessageCommentEntity as MessageCommentEntity2, MessageEntity};
pub use notifications::{NotificationEntity, SendInterval, UnsubscribedReason};
