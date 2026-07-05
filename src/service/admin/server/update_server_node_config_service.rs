use chrono::Utc;

use crate::model::dto::server::GetServerConfigRequest;
use crate::model::entity::node::ServerConfigOverride;
use crate::repository::node::NodeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn update_server_node_config(
    repo: &dyn NodeRepo,
    req: GetServerConfigRequest,
    body: ServerConfigOverride,
) -> Result<ServerConfigOverride, anyhow::Error> {
    let now = Utc::now().timestamp_millis();
    let entity = ServerConfigOverride {
        id: body.id,
        server_id: req.common.server_id,
        ip_strategy: body.ip_strategy,
        dns: body.dns,
        block: body.block,
        outbound: body.outbound,
        created_at: body.created_at,
        updated_at: now,
    };
    repo.update_override(&entity)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        )))
}
