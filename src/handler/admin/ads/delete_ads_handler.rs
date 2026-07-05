use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::ads::delete_ads_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn delete_ads(
    State(state): State<AppState>,
    Json(req): Json<DeleteAdsRequest>,
) -> HttpResult {
    match delete_ads_service::delete_ads(state.repos.ads.as_ref(), req.id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
