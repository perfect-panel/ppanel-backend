use crate::service::admin::tool::restart_system_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn restart_system() -> HttpResult {
    match restart_system_service::restart_system().await {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
