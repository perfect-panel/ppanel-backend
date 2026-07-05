use crate::repository::node::NodeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn query_node_tag(repo: &dyn NodeRepo) -> Result<Vec<String>, anyhow::Error> {
    repo.query_node_tags()
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))
}
