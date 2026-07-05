use axum::extract::State;
use crate::handler::AppState;
use crate::service::admin::payment::get_payment_method_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_payment_platform(
    State(_state): State<AppState>,
) -> HttpResult {
    match get_payment_method_list_service::get_payment_platform().await {
        Ok(d) => build_http_result(Some(d), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
