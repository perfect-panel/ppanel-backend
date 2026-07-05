use crate::model::dto::server::ResetSortRequest;
use crate::repository::node::NodeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn reset_sort_with_node(
    repo: &dyn NodeRepo,
    req: ResetSortRequest,
) -> Result<(), anyhow::Error> {
    for item in req.sort {
        repo.update_node_sort(item.id, item.sort)
            .await
            .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_UPDATE_ERROR,
                &e.to_string(),
            )))?;
    }
    Ok(())
}
