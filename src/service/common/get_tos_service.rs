use crate::model::dto::common::GetTosResponse;
use crate::repository::Repositories;
use anyhow::anyhow;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_tos(repos: &Repositories) -> anyhow::Result<GetTosResponse> {
    let configs = repos
        .system
        .get_tos_config()
        .await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

    let tos_content = configs
        .iter()
        .find(|s| s.key == "TosContent")
        .map(|s| s.value.clone())
        .unwrap_or_default();

    Ok(GetTosResponse { tos_content })
}
