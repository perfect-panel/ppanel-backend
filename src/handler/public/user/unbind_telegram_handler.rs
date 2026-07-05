use axum::extract::State;
use axum::Extension;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::service::public::user::unbind_telegram_service::UnbindTelegramService;
use result::http_result::{build_http_result, HttpResult};

pub async fn unbind_telegram(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
) -> HttpResult {
    let svc = UnbindTelegramService::new(state.repos.clone());
    match svc.unbind_telegram(auth.user_id).await {
        Ok(()) => build_http_result(Some(()), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
