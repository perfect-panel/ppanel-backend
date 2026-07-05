// Response envelopes and HTTP result construction.
//
// Ported from the Go package `result` (responseBean.go + httpResult.go). The Go
// package wraps responses in `ResponseSuccessBean` / `ResponseErrorBean` and
// writes them through a Hertz `*Context`; in axum the equivalent is producing a
// type that implements `IntoResponse`, so those beans are exposed directly.

use axum::http::StatusCode;
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Serialize;
use std::error::Error as StdError;

use crate::code_error::CodeError;
use crate::error_code;

/// Envelope returned for a successful request.
///
/// Mirrors Go `result.ResponseSuccessBean`. `data` is omitted from the JSON
/// payload when `None` (the `omitempty`-style `skip_serializing_if`).
#[derive(Debug, Serialize)]
pub struct ResponseSuccessBean<T = serde_json::Value> {
    pub code: u32,
    pub msg: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
}

impl<T> ResponseSuccessBean<T> {
    pub fn new(data: Option<T>) -> Self {
        Self {
            code: 200,
            msg: "success".to_string(),
            data,
        }
    }
}

/// Marker for an empty (null) data payload, kept for parity with the Go
/// `NullJson` type.
pub struct NullJson;

/// Envelope returned for a failed request.
///
/// Mirrors Go `result.ResponseErrorBean`.
#[derive(Debug, Serialize)]
pub struct ResponseErrorBean {
    pub code: u32,
    pub msg: String,
}

/// Builds a success envelope wrapping `data`.
///
/// Mirrors Go `result.Success`.
pub fn success<T>(data: T) -> ResponseSuccessBean<T> {
    ResponseSuccessBean::new(Some(data))
}

/// Builds an error envelope for the given code and message.
///
/// Mirrors Go `result.Error`.
pub fn error(err_code: u32, err_msg: impl Into<String>) -> ResponseErrorBean {
    ResponseErrorBean {
        code: err_code,
        msg: err_msg.into(),
    }
}

/// A structured HTTP response, identical in intent to the Go `HTTPResult`:
/// an HTTP status code paired with the serialized body. Axum's `Response`
/// subsumes the original `(StatusCode, body)` pair.
#[derive(Debug)]
pub struct HttpResult {
    pub status_code: StatusCode,
    pub body: Response,
}

/// Constructs an `HttpResult` from a fallible response, normalizing the body
/// into a success or error envelope.
///
/// Mirrors Go `result.BuildHTTPResult`:
/// - On success: HTTP 200 with a `ResponseSuccessBean` body.
/// - On error: HTTP 200 with a `ResponseErrorBean` body, the code/message
///   recovered from any nested `CodeError`, defaulting to `ERROR` /
///   `"Internal Server Error"` for plain errors.
pub fn build_http_result<T>(resp: Option<T>, err: Option<anyhow::Error>) -> HttpResult
where
    T: Serialize,
{
    if let Some(err) = err {
        let (code, msg) = recover_code_and_msg(&err);
        return HttpResult {
            status_code: StatusCode::OK,
            body: error(code, msg).into_response_body(),
        };
    }

    HttpResult {
        status_code: StatusCode::OK,
        body: Json(ResponseSuccessBean::new(resp)).into_response_body(),
    }
}

/// Constructs an `HttpResult` for a parameter-validation failure.
///
/// Mirrors Go `result.BuildParamErrorResult`: HTTP 200 with a `ResponseErrorBean`
/// whose code is `INVALID_PARAMS` and whose message is the raw error text.
pub fn build_param_error_result(err: &dyn StdError) -> HttpResult {
    HttpResult {
        status_code: StatusCode::OK,
        body: error(error_code::INVALID_PARAMS, err.to_string()).into_response_body(),
    }
}

/// Emits an HTTP response built from a fallible result.
///
/// Mirrors Go `result.HttpResult(ctx, resp, err)` (which wrote the body via the
/// Hertz context). Here it simply renders the constructed `HttpResult`.
pub fn http_result<T>(resp: Option<T>, err: Option<anyhow::Error>) -> Response
where
    T: Serialize,
{
    build_http_result(resp, err).into_response()
}

/// Emits an HTTP response for a parameter-validation failure.
///
/// Mirrors Go `result.ParamErrorResult(ctx, err)`. The Go version also logged the
/// error onto the Hertz error chain (`ctx.Error`); in axum that side effect is
/// the caller's responsibility (e.g. a tracing call), so only the response is
/// produced here.
pub fn param_error_result(err: &dyn StdError) -> Response {
    build_param_error_result(err).into_response()
}

impl IntoResponse for HttpResult {
    fn into_response(self) -> Response {
        self.body
    }
}

// ---- helpers --------------------------------------------------------------

/// Re-implementation of Go's `errors.As(errors.Cause(err), &e)` chain walk: the
/// first `CodeError` found while unwrapping `anyhow::Error` wins.
fn recover_code_and_msg(err: &anyhow::Error) -> (u32, String) {
    for cause in err.chain() {
        if let Some(code_err) = cause.downcast_ref::<CodeError>() {
            return (code_err.get_err_code(), code_err.get_err_msg().to_string());
        }
    }
    (error_code::ERROR, "Internal Server Error".to_string())
}

/// Extension so the `(StatusCode, Json<T>)` rendering can be captured as the
/// inner `Response` body of an `HttpResult`.
trait IntoResponseBody {
    fn into_response_body(self) -> Response;
}

impl<T: Serialize> IntoResponseBody for Json<T> {
    fn into_response_body(self) -> Response {
        (StatusCode::OK, self).into_response()
    }
}

impl IntoResponseBody for ResponseErrorBean {
    fn into_response_body(self) -> Response {
        (StatusCode::OK, Json(self)).into_response()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use axum::body::to_bytes;
    use crate::code_error::CodeError;
    use crate::error_code;

    #[tokio::test]
    async fn build_http_result_success() {
        let result = build_http_result(Some("ok"), None);
        assert_eq!(result.status_code, StatusCode::OK);
        let bytes = to_bytes(result.body.into_body(), usize::MAX).await.unwrap();
        let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(v["code"], 200);
        assert_eq!(v["msg"], "success");
        assert_eq!(v["data"], "ok");
    }

    #[tokio::test]
    async fn build_http_result_code_error() {
        let err = anyhow::Error::new(CodeError::new_err_code(error_code::INVALID_PARAMS));
        let result = build_http_result::<()>(None, Some(err));
        let bytes = to_bytes(result.body.into_body(), usize::MAX).await.unwrap();
        let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(v["code"], 400);
        assert_eq!(v["msg"], "Param Error");
        assert!(v.get("data").is_none());
    }

    #[tokio::test]
    async fn build_http_result_generic_error() {
        let err = anyhow::Error::msg("boom");
        let result = build_http_result::<()>(None, Some(err));
        let bytes = to_bytes(result.body.into_body(), usize::MAX).await.unwrap();
        let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(v["code"], 500);
        assert_eq!(v["msg"], "Internal Server Error");
    }

    #[tokio::test]
    async fn build_param_error_result_works() {
        let err = anyhow::Error::msg("bad param");
        let result = build_param_error_result(err.as_ref());
        let bytes = to_bytes(result.body.into_body(), usize::MAX).await.unwrap();
        let v: serde_json::Value = serde_json::from_slice(&bytes).unwrap();
        assert_eq!(v["code"], 400);
        assert_eq!(v["msg"], "bad param");
    }

    #[test]
    fn success_envelope_shape() {
        let bean = success(serde_json::json!({"x": 1}));
        assert_eq!(bean.code, 200);
        assert_eq!(bean.msg, "success");
        assert_eq!(bean.data.as_ref().unwrap()["x"], 1);
    }

    #[test]
    fn error_envelope_shape() {
        let bean = error(401, "Too Many Requests");
        assert_eq!(bean.code, 401);
        assert_eq!(bean.msg, "Too Many Requests");
    }
}
