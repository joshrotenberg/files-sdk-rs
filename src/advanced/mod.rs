//! Advanced features module
//!
//! This module contains handlers for advanced and miscellaneous features:
//! - Form field sets
//! - Share groups
//! - SIEM HTTP destinations
//! - Child site management
//! - External events
//! - Styles and branding
//! - DNS records
//! - Apps
//! - Email incoming messages
//! - Holiday regions

pub mod apps;
pub mod child_site_management_policies;
pub mod dns_records;
pub mod email_incoming_messages;
pub mod external_events;
pub mod form_field_sets;
pub mod holiday_regions;
pub mod share_groups;
pub mod siem_http_destinations;
pub mod styles;

// Re-export handlers
pub use apps::AppHandler;
pub use child_site_management_policies::ChildSiteManagementPolicyHandler;
pub use dns_records::DnsRecordHandler;
pub use email_incoming_messages::EmailIncomingMessageHandler;
pub use external_events::ExternalEventHandler;
pub use form_field_sets::FormFieldSetHandler;
pub use holiday_regions::HolidayRegionHandler;
pub use share_groups::ShareGroupHandler;
pub use siem_http_destinations::SiemHttpDestinationHandler;
pub use styles::StyleHandler;

// Re-export entities
pub use apps::AppEntity;
pub use child_site_management_policies::ChildSiteManagementPolicyEntity;
pub use dns_records::DnsRecordEntity;
pub use email_incoming_messages::EmailIncomingMessageEntity;
pub use external_events::ExternalEventEntity;
pub use form_field_sets::FormFieldSetEntity;
pub use holiday_regions::HolidayRegionEntity;
pub use share_groups::ShareGroupEntity;
pub use siem_http_destinations::SiemHttpDestinationEntity;
pub use styles::StyleEntity;
