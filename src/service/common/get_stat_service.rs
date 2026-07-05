use crate::model::dto::common::GetStatResponse;
use crate::repository::Repositories;
use anyhow::anyhow;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_stat(repos: &Repositories) -> anyhow::Result<GetStatResponse> {
    let mut user_count = repos
        .user
        .count_enabled_users()
        .await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

    if user_count > 100 {
        user_count -= user_count % 100;
    } else if user_count > 10 {
        user_count -= user_count % 10;
    } else {
        user_count = 1;
    }

    let node_count = repos
        .node
        .count_enabled_nodes()
        .await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

    let raw_protocols = repos
        .node
        .query_enabled_node_protocols()
        .await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

    let mut seen = std::collections::HashSet::new();
    let protocol: Vec<String> = raw_protocols
        .into_iter()
        .filter(|p| !p.is_empty() && seen.insert(p.clone()))
        .collect();

    Ok(GetStatResponse {
        user: user_count,
        node: node_count,
        country: 0,
        protocol,
    })
}
