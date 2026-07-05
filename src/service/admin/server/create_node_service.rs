use chrono::Utc;

use crate::model::dto::node::CreateNodeRequest;
use crate::model::entity::node::Node;
use crate::repository::node::NodeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn create_node(
    repo: &dyn NodeRepo,
    req: CreateNodeRequest,
) -> Result<Node, anyhow::Error> {
    let now = Utc::now().timestamp_millis();
    let tags_csv = req.tags.clone().unwrap_or_default().join(",");
    let entity = Node {
        id: 0,
        name: req.name,
        tags: tags_csv,
        port: req.port as i32,
        address: req.address,
        server_id: req.server_id,
        protocol: req.protocol,
        enabled: req.enabled,
        sort: 0,
        created_at: now,
        updated_at: now,
    };
    repo.insert_node(&entity)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_INSERT_ERROR,
            &e.to_string(),
        )))
}
