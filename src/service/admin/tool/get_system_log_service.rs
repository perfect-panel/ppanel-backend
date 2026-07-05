use crate::model::dto::log::LogResponse;
use serde_json::Value;

pub async fn get_system_log() -> anyhow::Result<LogResponse> {
    // System log reading requires file-system access to the log path.
    // Return empty list as safe default — full file reading is a future enhancement.
    Ok(LogResponse {
        list: Value::Array(vec![]),
    })
}
