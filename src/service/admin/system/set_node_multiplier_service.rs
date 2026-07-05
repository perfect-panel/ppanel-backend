use result::code_error::CodeError;
use result::error_code;
use serde_json;

use crate::model::dto::SetNodeMultiplierRequest;
use crate::repository::Repositories;

/// Persist a new node-multiplier schedule.
///
/// Stored as a JSON-encoded `Vec<TimePeriod>` under `(server, NodeMultiplierConfig)`.
pub async fn set_node_multiplier(
    repos: &Repositories,
    req: SetNodeMultiplierRequest,
) -> Result<(), anyhow::Error> {
    let payload = serde_json::to_string(&req.periods).map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            e.to_string(),
        ))
    })?;
    repos
        .system
        .update_node_multiplier_config(&payload)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_UPDATE_ERROR,
                e.to_string(),
            ))
        })?;
    Ok(())
}
