//! Ticket status & follow-type constants shared by admin ticket services.

/// Ticket lifecycle states (mirrors `model::entity::ticket::*`).
pub const TICKET_STATUS_PENDING: i16 = 1;
pub const TICKET_STATUS_WAITING: i16 = 2;
pub const TICKET_STATUS_PROCESSED: i16 = 3;
pub const TICKET_STATUS_CLOSED: i16 = 4;

/// Follow message sender origin.
pub const FOLLOW_FROM_USER: &str = "user";
pub const FOLLOW_FROM_ADMIN: &str = "admin";

/// Follow entry type.
pub const FOLLOW_TYPE_USER: i16 = 1;
pub const FOLLOW_TYPE_ADMIN: i16 = 2;
