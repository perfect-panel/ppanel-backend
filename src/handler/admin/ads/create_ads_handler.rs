use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::ads::create_ads_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn create_ads(
    State(state): State<AppState>,
    Json(req): Json<CreateAdsRequest>,
) -> HttpResult {
    match create_ads_service::create_ads(state.repos.ads.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
