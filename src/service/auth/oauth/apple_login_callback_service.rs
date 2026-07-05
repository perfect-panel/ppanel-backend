//! Apple OAuth callback — stores state in Redis and redirects the browser.
//!
//! Apple's form_post callback POSTs `code` + `state` to our server.
//! We look up the original redirect URL from Redis and redirect the client.

use std::sync::Arc;

use anyhow::anyhow;

use crate::cache::Cache;
use crate::config::Config;
use crate::model::dto::auth::AppleLoginCallbackRequest;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct AppleLoginCallbackService {
    _repos: Arc<Repositories>,
    _config: Arc<Config>,
    cache: Arc<Cache>,
}

impl AppleLoginCallbackService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>, cache: Arc<Cache>) -> Self {
        Self { _repos: repos, _config: config, cache }
    }

    /// Returns the redirect URL the browser should be sent to.
    ///
    /// Go behaviour: look up "telegram:<state>" in Redis to find the original
    /// redirect URL, then redirect to `{redirect}?code={code}&state={state}`.
    pub async fn callback(
        &self,
        req: AppleLoginCallbackRequest,
    ) -> Result<String, anyhow::Error> {
        let redis_key = format!("telegram:{}", req.state);

        let redirect = self.cache.get(&redis_key).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?
            .ok_or_else(|| anyhow!(CodeError::new_err_code_msg(
                error_code::ERROR,
                "invalid or expired apple oauth state",
            )))?;

        // Delete state key after use (one-time)
        let _ = self.cache.del(&redis_key).await;

        let url = format!(
            "{}?code={}&state={}",
            redirect,
            urlencoding::encode(&req.code),
            urlencoding::encode(&req.state),
        );
        Ok(url)
    }
}
