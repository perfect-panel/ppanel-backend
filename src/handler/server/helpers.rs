/// Server middleware and utility functions for server-facing handlers.
///
/// Re-exports `check_node_auth` so other handler modules can import it
/// from a single place.
pub use super::get_server_config_handler::check_node_auth;

/// Placeholder — may be extended with axum middleware later.
pub fn server_middleware() {}
