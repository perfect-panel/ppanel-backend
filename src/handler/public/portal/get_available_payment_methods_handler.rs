use axum::extract::State;

use crate::handler::AppState;
use crate::service::public::portal::get_available_payment_methods_service::GetAvailablePaymentMethodsService;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_available_payment_methods(
    State(state): State<AppState>,
) -> HttpResult {
    let svc = GetAvailablePaymentMethodsService::new(state.repos.clone());
    match svc.get().await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
