use axum::extract::{Query, State};

use crate::handler::AppState;
use crate::model::dto::*;
use crate::service::admin::application::preview_subscribe_template_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn preview_subscribe_template(
    State(state): State<AppState>,
    Query(req): Query<PreviewSubscribeTemplateRequest>,
) -> HttpResult {
    match preview_subscribe_template_service::preview_subscribe_template(
        state.repos.client.as_ref(),
        req,
    )
    .await
    {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
