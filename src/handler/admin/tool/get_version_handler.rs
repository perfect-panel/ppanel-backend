use crate::service::admin::tool::get_version_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_version() -> HttpResult {
    match get_version_service::get_version().await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
