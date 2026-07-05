use axum::extract::State;
use axum::Extension;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::service::public::user::bind_telegram_service::BindTelegramService;
use result::http_result::{build_http_result, HttpResult};

pub async fn bind_telegram(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
) -> HttpResult {
    let svc = BindTelegramService::new(state.repos.clone());
    match svc.bind_telegram(auth.user_id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
