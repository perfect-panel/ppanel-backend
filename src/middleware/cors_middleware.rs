//! CORS middleware — port of `corsMiddleware.go`.
use axum::{
    extract::Request,
    http::{header, Method, StatusCode},
    middleware::Next,
    response::{IntoResponse, Response},
};

pub async fn cors_middleware(req: Request, next: Next) -> Response {
    let origin = req
        .headers()
        .get(header::ORIGIN)
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

    if req.method() == Method::OPTIONS {
        let mut resp = StatusCode::NO_CONTENT.into_response();
        add_cors_headers(resp.headers_mut(), origin.as_deref());
        return resp;
    }

    let mut resp = next.run(req).await;
    add_cors_headers(resp.headers_mut(), origin.as_deref());
    resp
}

fn add_cors_headers(headers: &mut axum::http::HeaderMap, origin: Option<&str>) {
    let origin_val = origin.unwrap_or("*");
    let _ = headers.insert(
        header::ACCESS_CONTROL_ALLOW_ORIGIN,
        origin_val.parse().unwrap_or(header::HeaderValue::from_static("*")),
    );
    let _ = headers.insert(
        header::ACCESS_CONTROL_ALLOW_METHODS,
        "POST, GET, OPTIONS, PUT, DELETE, UPDATE".parse().unwrap(),
    );
    let _ = headers.insert(
        header::ACCESS_CONTROL_ALLOW_HEADERS,
        "Content-Type, Origin, X-CSRF-Token, Authorization, AccessToken, Token, Range"
            .parse().unwrap(),
    );
    let _ = headers.insert(
        header::ACCESS_CONTROL_EXPOSE_HEADERS,
        "Content-Length, Access-Control-Allow-Origin, Access-Control-Allow-Headers"
            .parse().unwrap(),
    );
    let _ = headers.insert(
        header::ACCESS_CONTROL_ALLOW_CREDENTIALS,
        "true".parse().unwrap(),
    );
    let _ = headers.insert(
        header::ACCESS_CONTROL_MAX_AGE,
        "172800".parse().unwrap(),
    );
}
