use chrono::Utc;

use crate::model::dto::common::HeartbeatResponse;

pub async fn heartbeat() -> anyhow::Result<HeartbeatResponse> {
    Ok(HeartbeatResponse {
        status: true,
        message: Some("service is alive".to_string()),
        timestamp: Some(Utc::now().timestamp()),
    })
}
