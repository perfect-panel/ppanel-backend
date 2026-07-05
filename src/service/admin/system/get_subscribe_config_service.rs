use result::code_error::CodeError;
use result::error_code;

use crate::model::dto::SubscribeConfig;
use crate::repository::Repositories;

/// Read subscribe configuration from the `system` table (category = "subscribe").
pub async fn get_subscribe_config(
    repos: &Repositories,
) -> Result<SubscribeConfig, anyhow::Error> {
    let rows = repos
        .system
        .get_subscribe_config()
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string(),
            ))
        })?;
    let mut resp = SubscribeConfig {
        single_model: false,
        subscribe_path: String::new(),
        subscribe_domain: String::new(),
        pan_domain: false,
        user_agent_limit: false,
        user_agent_list: String::new(),
        show_tutorial: true,
    };
    for row in rows {
        match row.key.as_str() {
            "single_model" => resp.single_model = row.value == "true",
            "subscribe_path" => resp.subscribe_path = row.value,
            "subscribe_domain" => resp.subscribe_domain = row.value,
            "pan_domain" => resp.pan_domain = row.value == "true",
            "user_agent_limit" => resp.user_agent_limit = row.value == "true",
            "user_agent_list" => resp.user_agent_list = row.value,
            "show_tutorial" => resp.show_tutorial = row.value == "true",
            _ => {}
        }
    }
    Ok(resp)
}
