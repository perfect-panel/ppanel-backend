use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::public::portal::get_subscription_service::GetSubscriptionService;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_subscription(
    State(state): State<AppState>,
    Query(req): Query<GetSubscriptionRequest>,
) -> HttpResult {
    let svc = GetSubscriptionService::new(state.repos.clone());
    match svc.get(req.language).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
