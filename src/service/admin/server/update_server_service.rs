use chrono::Utc;

use crate::model::dto::server::UpdateServerRequest;
use crate::model::entity::node::Server;
use crate::repository::node::NodeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn update_server(
    repo: &dyn NodeRepo,
    req: UpdateServerRequest,
) -> Result<Server, anyhow::Error> {
    let existing = repo
        .find_one_server(req.id)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;
    let protocols = serde_json::to_string(&req.protocols)
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::INVALID_PARAMS,
            &format!("encode protocols: {e}"),
        )))?;
    let updated = Server {
        id: existing.id,
        name: req.name,
        country: req.country.unwrap_or(existing.country),
        city: req.city.unwrap_or(existing.city),
        address: req.address,
        sort: req.sort.unwrap_or(existing.sort),
        protocols,
        last_reported_at: existing.last_reported_at,
        created_at: existing.created_at,
        updated_at: Utc::now().timestamp_millis(),
    };
    repo.update_server(&updated)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        )))
}
