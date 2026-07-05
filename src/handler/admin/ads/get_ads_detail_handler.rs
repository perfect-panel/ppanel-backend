use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::ads::get_ads_detail_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_ads_detail(
    State(state): State<AppState>,
    Query(req): Query<GetAdsDetailRequest>,
) -> HttpResult {
    match get_ads_detail_service::get_ads_detail(state.repos.ads.as_ref(), req.id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
