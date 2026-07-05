use result::code_error::CodeError;
use result::error_code;

use crate::model::dto::RegisterConfig;
use crate::repository::Repositories;

/// Read registration configuration from the `system` table (category = "register").
pub async fn get_register_config(
    repos: &Repositories,
) -> Result<RegisterConfig, anyhow::Error> {
    let rows = repos
        .system
        .get_register_config()
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string(),
            ))
        })?;
    let mut resp = RegisterConfig {
        stop_register: false,
        enable_trial: false,
        trial_subscribe: 0,
        trial_time: 0,
        trial_time_unit: String::new(),
        enable_ip_register_limit: false,
        ip_register_limit: 0,
        ip_register_limit_duration: 0,
    };
    for row in rows {
        match row.key.as_str() {
            "stop_register" => resp.stop_register = row.value == "true",
            "enable_trial" => resp.enable_trial = row.value == "true",
            "trial_subscribe" => {
                if let Ok(v) = row.value.parse() {
                    resp.trial_subscribe = v;
                }
            }
            "trial_time" => {
                if let Ok(v) = row.value.parse() {
                    resp.trial_time = v;
                }
            }
            "trial_time_unit" => resp.trial_time_unit = row.value,
            "enable_ip_register_limit" => {
                resp.enable_ip_register_limit = row.value == "true"
            }
            "ip_register_limit" => {
                if let Ok(v) = row.value.parse() {
                    resp.ip_register_limit = v;
                }
            }
            "ip_register_limit_duration" => {
                if let Ok(v) = row.value.parse() {
                    resp.ip_register_limit_duration = v;
                }
            }
            _ => {}
        }
    }
    Ok(resp)
}
