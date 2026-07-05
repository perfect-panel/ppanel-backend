use axum::extract::State;
use axum::Json;
use crate::handler::AppState;
use crate::model::dto::payment::CreatePaymentMethodRequest;
use crate::service::admin::payment::get_payment_method_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn create_payment_method(
    State(state): State<AppState>,
    Json(req): Json<CreatePaymentMethodRequest>,
) -> HttpResult {
    match get_payment_method_list_service::create_payment_method(state.repos.payment.as_ref(), req).await {
        Ok(_) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
