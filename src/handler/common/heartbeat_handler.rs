use crate::model::dto::common::HeartbeatResponse;
use crate::service::common::heartbeat_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn heartbeat() -> HttpResult {
    match heartbeat_service::heartbeat().await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result::<HeartbeatResponse>(None, Some(err)),
    }
}
