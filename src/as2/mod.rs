//! AS2 protocol module
//!
//! This module contains handlers for AS2 (Applicability Statement 2) protocol:
//! - AS2 partners and stations
//! - Incoming and outgoing AS2 messages

pub mod as2_incoming_messages;
pub mod as2_outgoing_messages;
pub mod as2_partners;
pub mod as2_stations;

// Re-export handlers
pub use as2_incoming_messages::As2IncomingMessageHandler;
pub use as2_outgoing_messages::As2OutgoingMessageHandler;
pub use as2_partners::As2PartnerHandler;
pub use as2_stations::As2StationHandler;

// Re-export entities
pub use as2_incoming_messages::As2IncomingMessageEntity;
pub use as2_outgoing_messages::As2OutgoingMessageEntity;
pub use as2_partners::As2PartnerEntity;
pub use as2_stations::As2StationEntity;
