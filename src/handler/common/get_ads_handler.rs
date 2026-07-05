use axum::extract::{Query, State};
use result::http_result::{build_http_result, HttpResult};

use crate::handler::AppState;
use crate::model::dto::ads::GetAdsRequest;
use crate::service::common::get_ads_service;

pub async fn get_ads(
    State(state): State<AppState>,
    Query(_req): Query<GetAdsRequest>,
) -> HttpResult {
    match get_ads_service::get_ads(&state.repos).await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result(None::<()>, Some(err)),
    }
}
