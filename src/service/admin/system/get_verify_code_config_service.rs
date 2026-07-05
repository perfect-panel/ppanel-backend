use result::code_error::CodeError;
use result::error_code;

use crate::model::dto::VerifyCodeConfig;
use crate::repository::Repositories;

/// Read verify-code configuration from the `system` table (category = "verify_code").
pub async fn get_verify_code_config(
    repos: &Repositories,
) -> Result<VerifyCodeConfig, anyhow::Error> {
    let rows = repos
        .system
        .get_verify_code_config()
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string(),
            ))
        })?;
    let mut resp = VerifyCodeConfig {
        verify_code_expire_time: 300,
        verify_code_limit: 15,
        verify_code_interval: 60,
    };
    for row in rows {
        match row.key.as_str() {
            "verify_code_expire_time" | "expire_time" => {
                if let Ok(v) = row.value.parse() {
                    resp.verify_code_expire_time = v;
                }
            }
            "verify_code_limit" | "limit" => {
                if let Ok(v) = row.value.parse() {
                    resp.verify_code_limit = v;
                }
            }
            "verify_code_interval" | "interval" => {
                if let Ok(v) = row.value.parse() {
                    resp.verify_code_interval = v;
                }
            }
            _ => {}
        }
    }
    Ok(resp)
}
