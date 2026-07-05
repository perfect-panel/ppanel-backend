use result::code_error::CodeError;
use result::error_code;

use crate::model::dto::TosConfig;
use crate::repository::Repositories;

/// Read Terms-of-Service configuration from the `system` table (category = "tos").
pub async fn get_tos_config(
    repos: &Repositories,
) -> Result<TosConfig, anyhow::Error> {
    let rows = repos
        .system
        .get_tos_config()
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string(),
            ))
        })?;
    let mut resp = TosConfig {
        tos_content: String::new(),
    };
    for row in rows {
        if row.key == "tos_content" {
            resp.tos_content = row.value;
        }
    }
    Ok(resp)
}
