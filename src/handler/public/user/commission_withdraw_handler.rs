use axum::extract::State;
use axum::Extension;
use axum::Json;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::model::dto::*;
use crate::service::public::user::commission_withdraw_service::CommissionWithdrawService;
use result::http_result::{build_http_result, HttpResult};

pub async fn commission_withdraw(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
    Json(req): Json<CommissionWithdrawRequest>,
) -> HttpResult {
    let svc = CommissionWithdrawService::new(state.repos.clone());
    match svc.commission_withdraw(auth.user_id, req).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
