use axum::extract::State;

use crate::handler::AppState;
use crate::model::dto::system::PrivacyPolicyConfig;
use crate::service::common::get_privacy_policy_service;
use result::http_result::{build_http_result, HttpResult};

pub async fn get_privacy_policy(State(state): State<AppState>) -> HttpResult {
    match get_privacy_policy_service::get_privacy_policy(&state.repos).await {
        Ok(resp) => build_http_result(Some(resp), None),
        Err(err) => build_http_result::<PrivacyPolicyConfig>(None, Some(err)),
    }
}
