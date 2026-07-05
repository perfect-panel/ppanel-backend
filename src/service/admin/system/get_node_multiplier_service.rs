use result::code_error::CodeError;
use result::error_code;
use serde_json;

use crate::model::dto::{GetNodeMultiplierResponse, TimePeriod};
use crate::repository::Repositories;

/// Read the node-multiplier schedule from the `system` table.
///
/// Mirrors Go `getNodeMultiplierLogic` — stored as a JSON array of
/// `TimePeriod` under `(server, NodeMultiplierConfig)`.
pub async fn get_node_multiplier(
    repos: &Repositories,
) -> Result<GetNodeMultiplierResponse, anyhow::Error> {
    let row = repos
        .system
        .find_node_multiplier_config()
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string(),
            ))
        })?;
    let periods: Vec<TimePeriod> = if row.value.is_empty() {
        Vec::new()
    } else {
        serde_json::from_str(&row.value).map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_UPDATE_ERROR,
                e.to_string(),
            ))
        })?
    };
    Ok(GetNodeMultiplierResponse { periods })
}
