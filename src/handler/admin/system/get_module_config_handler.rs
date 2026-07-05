use crate::service::admin::system::get_module_config_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_module_config() -> HttpResult {
    match get_module_config_service::get_module_config().await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
