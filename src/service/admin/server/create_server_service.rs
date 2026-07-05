use chrono::Utc;

use crate::model::dto::server::CreateServerRequest;
use crate::model::entity::node::Server;
use crate::repository::node::NodeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn create_server(
    repo: &dyn NodeRepo,
    req: CreateServerRequest,
) -> Result<Server, anyhow::Error> {
    let now = Utc::now().timestamp_millis();
    let protocols = serde_json::to_string(&req.protocols)
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::INVALID_PARAMS,
            &format!("encode protocols: {e}"),
        )))?;
    let entity = Server {
        id: 0,
        name: req.name,
        country: req.country.unwrap_or_default(),
        city: req.city.unwrap_or_default(),
        address: req.address,
        sort: req.sort.unwrap_or(0),
        protocols,
        last_reported_at: None,
        created_at: now,
        updated_at: now,
    };
    repo.insert_server(&entity)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_INSERT_ERROR,
            &e.to_string(),
        )))
}
