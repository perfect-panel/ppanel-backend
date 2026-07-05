use axum::extract::{Query, State};
use crate::handler::AppState;
use crate::model::dto::payment::GetPaymentMethodListRequest;
use crate::service::admin::payment::get_payment_method_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_payment_method_list(
    State(state): State<AppState>,
    Query(req): Query<GetPaymentMethodListRequest>,
) -> HttpResult {
    match get_payment_method_list_service::get_payment_method_list(state.repos.payment.as_ref(), req).await {
        Ok(d) => build_http_result(Some(d), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
