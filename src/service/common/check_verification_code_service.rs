use std::sync::Arc;

use anyhow::anyhow;
use crate::cache::Cache;
use crate::config::cache_key::AUTH_CODE_CACHE_KEY;
use crate::config::Config;
use crate::model::dto::auth::{CheckVerificationCodeRequest, CheckVerificationCodeRespone};
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct CheckVerificationCodeService {
    _repos: Arc<Repositories>,
    _config: Arc<Config>,
    cache: Arc<Cache>,
}

impl CheckVerificationCodeService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>, cache: Arc<Cache>) -> Self {
        Self {
            _repos: repos,
            _config: config,
            cache,
        }
    }

    pub async fn check(
        &self,
        req: CheckVerificationCodeRequest,
    ) -> Result<CheckVerificationCodeRespone, anyhow::Error> {
        let cache_key = format!("{}:{}:{}", AUTH_CODE_CACHE_KEY, req.type_, req.account);
        let cached = self.cache.get(&cache_key).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::VERIFY_CODE_ERROR, e.to_string())))?
            .ok_or_else(|| anyhow!(CodeError::new_err_code(error_code::VERIFY_CODE_ERROR)))?;

        let payload: serde_json::Value = serde_json::from_str(&cached)
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::VERIFY_CODE_ERROR)))?;

        let valid = payload["code"].as_str() == Some(&req.code);
        Ok(CheckVerificationCodeRespone { status: valid })
    }
}
