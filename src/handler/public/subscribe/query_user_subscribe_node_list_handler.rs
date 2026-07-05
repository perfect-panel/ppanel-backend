use axum::extract::State;
use axum::Extension;

use crate::handler::AppState;
use crate::middleware::auth_middleware::AuthContext;
use crate::service::public::subscribe::query_user_subscribe_node_list_service::QueryUserSubscribeNodeListService;
use result::http_result::{build_http_result, HttpResult};

pub async fn query_user_subscribe_node_list(
    State(state): State<AppState>,
    Extension(auth): Extension<AuthContext>,
) -> HttpResult {
    let svc = QueryUserSubscribeNodeListService::new(state.repos.clone());
    match svc.query_nodes(auth.user_id).await {
        Ok(data) => build_http_result(Some(data), None),
        Err(e) => build_http_result::<()>(None, Some(e)),
    }
}
