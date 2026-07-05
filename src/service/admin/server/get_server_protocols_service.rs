use crate::model::dto::server::{
    GetServerProtocolsRequest, GetServerProtocolsResponse,
};
use crate::repository::node::NodeRepo;
use crate::service::admin::server::constant::SUPPORTED_PROTOCOLS;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_server_protocols(
    repo: &dyn NodeRepo,
    req: GetServerProtocolsRequest,
) -> Result<GetServerProtocolsResponse, anyhow::Error> {
    // TODO: load configured protocols for `req.id` and merge with the static
    // SUPPORTED_PROTOCOLS list. For now, return the configured list as-is.
    let _ = req;
    repo.find_one_server(req.id)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;
    Ok(GetServerProtocolsResponse {
        protocols: Vec::new(),
    })
}

/// Returns the static list of supported protocol names. Exposed for callers
/// (handlers, tests) that want the catalogue without a DB round-trip.
pub fn supported_protocol_names() -> Vec<String> {
    SUPPORTED_PROTOCOLS.iter().map(|s| (*s).to_string()).collect()
}
