use crate::repository::node::NodeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn toggle_node_status(
    repo: &dyn NodeRepo,
    id: i64,
) -> Result<bool, anyhow::Error> {
    let mut node = repo
        .find_one_node(id)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;
    let next = !node.enabled.unwrap_or(false);
    node.enabled = Some(next);
    node.updated_at = chrono::Utc::now().timestamp_millis();
    let _updated = repo
        .update_node(&node)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        )))?;
    Ok(next)
}
