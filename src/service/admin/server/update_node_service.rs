use chrono::Utc;

use crate::model::dto::node::UpdateNodeRequest;
use crate::model::entity::node::Node;
use crate::repository::node::NodeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn update_node(
    repo: &dyn NodeRepo,
    req: UpdateNodeRequest,
) -> Result<Node, anyhow::Error> {
    let existing = repo
        .find_one_node(req.id)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;
    let tags_csv = req
        .tags
        .clone()
        .unwrap_or_else(|| {
            existing
                .tags
                .split(',')
                .filter(|s| !s.is_empty())
                .map(|s| s.trim().to_string())
                .collect()
        })
        .join(",");
    let updated = Node {
        id: existing.id,
        name: req.name,
        tags: tags_csv,
        port: req.port as i32,
        address: req.address,
        server_id: req.server_id,
        protocol: req.protocol,
        enabled: req.enabled.or(existing.enabled),
        sort: existing.sort,
        created_at: existing.created_at,
        updated_at: Utc::now().timestamp_millis(),
    };
    repo.update_node(&updated)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        )))
}
