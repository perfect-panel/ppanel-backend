//! Ticket-domain status constants.

/// Ticket status: open, awaiting admin reply.
pub const TICKET_STATUS_OPEN: i16 = 1;

/// Ticket status: admin replied, awaiting user response.
pub const TICKET_STATUS_PENDING_ADMIN: i16 = 2;

/// Ticket status: closed.
pub const TICKET_STATUS_CLOSED: i16 = 4;
