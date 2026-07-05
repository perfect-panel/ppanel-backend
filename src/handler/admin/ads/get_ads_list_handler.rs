use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::ads::get_ads_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_ads_list(
    State(state): State<AppState>,
    Query(req): Query<GetAdsListRequest>,
) -> HttpResult {
    match get_ads_list_service::get_ads_list(state.repos.ads.as_ref(), req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
