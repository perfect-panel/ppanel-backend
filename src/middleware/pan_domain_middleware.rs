use axum::{
    extract::{Request, State},
    http::StatusCode,
    middleware::Next,
    response::{IntoResponse, Response},
};
use crate::handler::AppState;

pub async fn pan_domain_middleware(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    // Only intercept when pan_domain is enabled and path is "/"
    if !state.config.subscribe.pan_domain || req.uri().path() != "/" {
        return next.run(req).await;
    }

    let host = req
        .headers()
        .get("host")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    let ua = req
        .headers()
        .get("user-agent")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("")
        .to_string();

    // Extract first subdomain label as token
    let parts: Vec<&str> = host.split('.').collect();
    if parts.len() < 2 {
        return (StatusCode::FORBIDDEN, "Access denied").into_response();
    }
    let token = parts[0].to_string();

    // User-agent limit check — mirrors IsUserAgentAllowed in userAgent.go
    if state.config.subscribe.user_agent_limit {
        let allowed = is_user_agent_allowed(&state, &ua).await;
        if !allowed {
            tracing::debug!("pan_domain: UA blocked by allowlist, UA={ua}");
            return (StatusCode::FORBIDDEN, "Access denied").into_response();
        }
    }

    // Delegate to subscribe service
    let svc = crate::service::subscribe::subscribe_service::SubscribeService::new(
        state.repos.clone(),
        state.config.clone(),
    );
    match svc.handle_subscribe(&ua, &token, &host, std::collections::HashMap::new()).await {
        Ok(result) => {
            use axum::response::Response as AxumResponse;
            use axum::http::header;
            let mut builder = AxumResponse::builder()
                .status(StatusCode::OK)
                .header("subscription-userinfo", &result.userinfo);
            builder = builder.header(header::CONTENT_TYPE, &result.content_type);
            if !result.disposition.is_empty() {
                builder = builder.header("content-disposition", &result.disposition);
            }
            builder
                .body(axum::body::Body::from(result.content))
                .unwrap_or_else(|_| (StatusCode::INTERNAL_SERVER_ERROR, "error").into_response())
        }
        Err(e) => {
            tracing::error!("pan_domain subscribe error: {e}");
            (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error").into_response()
        }
    }
}

async fn is_user_agent_allowed(state: &AppState, ua: &str) -> bool {
    if ua.is_empty() {
        return false;
    }

    let ua_lower = ua.to_lowercase();

    let mut keywords: Vec<String> = state
        .config
        .subscribe
        .user_agent_list
        .split('\n')
        .map(|s| s.trim().to_lowercase())
        .filter(|s| !s.is_empty())
        .collect();

    match state.repos.client.list().await {
        Ok(clients) => {
            for c in &clients {
                let needle = c.user_agent.trim().to_lowercase();
                if !needle.is_empty() {
                    keywords.push(needle);
                }
            }
        }
        Err(e) => {
            tracing::error!("pan_domain: failed to load client list: {e}");
        }
    }

    if keywords.is_empty() {
        return true;
    }

    keywords.iter().any(|k| ua_lower.contains(k.as_str()))
}
