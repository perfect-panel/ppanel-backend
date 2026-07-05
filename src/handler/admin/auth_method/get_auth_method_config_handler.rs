use axum::extract::{Query, State};
use crate::handler::AppState;
use crate::model::dto::auth::GetAuthMethodConfigRequest;
use crate::service::admin::auth_method::get_auth_method_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_auth_method_config(
    State(state): State<AppState>,
    Query(req): Query<GetAuthMethodConfigRequest>,
) -> HttpResult {
    match get_auth_method_list_service::get_auth_method_config(state.repos.auth.as_ref(), req).await {
        Ok(d) => build_http_result(Some(d), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
