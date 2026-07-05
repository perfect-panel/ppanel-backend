//! Payment notify middleware — port of `notifyMiddleware.go`.
//!
//! Extracts `platform` and `token` from the URI path, looks up the payment
//! config, and injects it into request extensions.

use std::sync::Arc;
use axum::{
    extract::{Path, Request, State},
    middleware::Next,
    response::{IntoResponse, Response},
};
use result::http_result::build_param_error_result;
use crate::handler::AppState;
use crate::model::entity::payment::Payment;

/// Extension injected by this middleware for downstream handlers.
#[derive(Debug, Clone)]
pub struct PaymentContext {
    pub platform: String,
    pub payment: Payment,
}

pub async fn notify_middleware(
    State(state): State<AppState>,
    Path(token): Path<String>,
    mut req: Request,
    next: Next,
) -> Response {
    match state.repos.payment.find_one_by_token(&token).await {
        Ok(payment) => {
            let ctx = PaymentContext {
                platform: payment.platform.clone(),
                payment,
            };
            req.extensions_mut().insert(Arc::new(ctx));
            next.run(req).await
        }
        Err(e) => {
            let err = anyhow::anyhow!("payment token invalid: {e}");
            build_param_error_result(err.as_ref()).into_response()
        }
    }
}
