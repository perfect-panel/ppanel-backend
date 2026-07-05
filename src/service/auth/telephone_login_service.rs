use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;
use uuid::Uuid;

use crate::cache::Cache;
use crate::config::cache_key::SESSION_ID_KEY;
use crate::config::Config;
use crate::model::dto::auth::{LoginResponse, TelephoneLoginRequest};
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;
use result::code_error::CodeError;
use result::error_code;

pub struct TelephoneLoginService {
    repos: Arc<Repositories>,
    config: Arc<Config>,
    cache: Arc<Cache>,
}

impl TelephoneLoginService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>, cache: Arc<Cache>) -> Self {
        Self { repos, config, cache }
    }

    pub async fn login(&self, req: TelephoneLoginRequest) -> Result<LoginResponse, anyhow::Error> {
        let phone = format!("+{}{}", req.telephone_area_code, req.telephone);

        let auth_method = self.repos.user.find_auth_method_by_open_id("mobile", &phone).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?
            .ok_or_else(|| anyhow!(CodeError::new_err_code(error_code::USER_NOT_EXIST)))?;

        let user = self.repos.user.find_one_user(auth_method.user_id).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

        if !user.enable {
            return Err(anyhow!(CodeError::new_err_code(error_code::USER_DISABLED)));
        }

        if !req.password.is_empty() {
            if !password::multi_password_verify(
                &user.algo, user.salt.as_deref().unwrap_or(""), &req.password, &user.password,
            ) {
                return Err(anyhow!(CodeError::new_err_code(error_code::USER_PASSWORD_ERROR)));
            }
        } else if !req.telephone_code.is_empty() {
            let cache_key = format!("{}:{}", crate::config::cache_key::AUTH_CODE_TELEPHONE_CACHE_KEY, phone);
            let cached = self.cache.get(&cache_key).await
                .map_err(|_| anyhow!(CodeError::new_err_code(error_code::VERIFY_CODE_ERROR)))?
                .ok_or_else(|| anyhow!(CodeError::new_err_code(error_code::VERIFY_CODE_ERROR)))?;

            let payload: serde_json::Value = serde_json::from_str(&cached)
                .map_err(|_| anyhow!(CodeError::new_err_code(error_code::VERIFY_CODE_ERROR)))?;

            if payload["code"].as_str() != Some(&req.telephone_code) {
                return Err(anyhow!(CodeError::new_err_code(error_code::VERIFY_CODE_ERROR)));
            }

            let _ = self.cache.del(&cache_key).await;
        } else {
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::PASSWORD_OR_VERIFICATION_CODE_REQUIRED,
                "password or verification code required",
            )));
        }

        if !req.identifier.is_empty() {
            let bind_svc = super::bind_device_service::BindDeviceService::new(
                self.repos.clone(), self.config.clone(), self.cache.clone(),
            );
            let _ = bind_svc.bind_device_to_user(&req.identifier, &req.ip, &req.user_agent, user.id).await;
        }

        let login_type = if req.login_type.is_empty() { "mobile".to_string() } else { req.login_type };
        let session_id = Uuid::new_v4().to_string();
        let (claims, seconds) = jwt::Claims::new(user.id, session_id.clone(), login_type);

        let token = jwt::generate_token(&claims, &self.config.jwt_auth.access_secret)
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, &e.to_string())))?;

        let session_key = format!("{}:{}", SESSION_ID_KEY, session_id);
        self.cache.set_ex(&session_key, &user.id.to_string(), seconds).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, &e.to_string())))?;

        Telemetry::login(&self.repos, user.id, "mobile", &req.ip, &req.user_agent, true).await;

        Ok(LoginResponse { token })
    }
}
