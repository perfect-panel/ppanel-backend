use crate::model::dto::server::GetServerConfigRequest;
use crate::model::entity::node::ServerConfigOverride;
use crate::repository::node::NodeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_server_node_config(
    repo: &dyn NodeRepo,
    req: GetServerConfigRequest,
) -> Result<ServerConfigOverride, anyhow::Error> {
    let node_id = req.common.server_id;
    repo.find_one_override(node_id)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))
}
