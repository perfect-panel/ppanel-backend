use crate::service::admin::tool::get_system_log_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_system_log() -> HttpResult {
    match get_system_log_service::get_system_log().await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
