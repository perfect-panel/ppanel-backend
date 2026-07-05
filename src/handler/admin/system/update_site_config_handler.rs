use axum::extract::State;
use axum::Json;

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::system::update_site_config_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_site_config(
    State(state): State<AppState>,
    Json(req): Json<SiteConfig>,
) -> HttpResult {
    match update_site_config_service::update_site_config(&state.repos, req).await {
        Ok(_) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
