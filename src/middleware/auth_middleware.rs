use axum::{
    extract::{Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};

use crate::config::cache_key::SESSION_ID_KEY;
use crate::handler::AppState;
use result::code_error::CodeError;
use result::error_code;
use result::http_result::build_http_result;

#[derive(Debug, Clone)]
pub struct AuthContext {
    pub user_id: i64,
    pub login_type: String,
    pub session_id: String,
    pub is_admin: bool,
}

pub async fn auth_middleware(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Response {
    let auth_header = request
        .headers()
        .get("Authorization")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("");

    let token = auth_header.strip_prefix("Bearer ").unwrap_or(auth_header);

    if token.is_empty() {
        return err_response(error_code::ERROR_TOKEN_EMPTY);
    }

    let claims = match jwt::validate_token(token, &state.config.jwt_auth.access_secret) {
        Ok(c) => c,
        Err(_) => return err_response(error_code::ERROR_TOKEN_INVALID),
    };

    let session_key = format!("{}:{}", SESSION_ID_KEY, claims.session_id);
    let cached_user_id: i64 = match state.cache.get(&session_key).await {
        Ok(Some(id)) => id.parse().unwrap_or(0),
        _ => return err_response(error_code::INVALID_ACCESS),
    };

    if cached_user_id != claims.user_id {
        return err_response(error_code::INVALID_ACCESS);
    }

    let user = match state.repos.user.find_one_user(claims.user_id).await {
        Ok(u) => u,
        Err(_) => return err_response(error_code::USER_NOT_EXIST),
    };

    if !user.enable {
        return err_response(error_code::USER_DISABLED);
    }

    let path = request.uri().path();
    if path.contains("/admin") && !user.is_admin {
        return err_response(error_code::INVALID_ACCESS);
    }

    let auth_ctx = AuthContext {
        user_id: claims.user_id,
        login_type: claims.login_type,
        session_id: claims.session_id,
        is_admin: user.is_admin,
    };
    request.extensions_mut().insert(auth_ctx);

    next.run(request).await
}

fn err_response(code: u32) -> Response {
    build_http_result::<()>(None, Some(anyhow::Error::new(CodeError::new_err_code(code))))
        .into_response()
}
