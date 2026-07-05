use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::ads::update_ads_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_ads(
    State(state): State<AppState>,
    Json(req): Json<UpdateAdsRequest>,
) -> HttpResult {
    match update_ads_service::update_ads(state.repos.ads.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
