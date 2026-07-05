use axum::extract::State;
use axum::Json;
use crate::handler::AppState;
use crate::model::dto::auth::UpdateAuthMethodConfigRequest;
use crate::service::admin::auth_method::get_auth_method_list_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn update_auth_method_config(
    State(state): State<AppState>,
    Json(req): Json<UpdateAuthMethodConfigRequest>,
) -> HttpResult {
    match get_auth_method_list_service::update_auth_method_config(state.repos.auth.as_ref(), req).await {
        Ok(_) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
