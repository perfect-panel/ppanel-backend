use std::sync::Arc;

use anyhow::anyhow;
use uuid::Uuid;

use crate::cache::Cache;
use crate::config::cache_key::SESSION_ID_KEY;
use crate::config::Config;
use crate::model::dto::auth::{LoginResponse, UserLoginRequest};
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;
use result::code_error::CodeError;
use result::error_code;

pub struct UserLoginService {
    repos: Arc<Repositories>,
    config: Arc<Config>,
    cache: Arc<Cache>,
}

impl UserLoginService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>, cache: Arc<Cache>) -> Self {
        Self { repos, config, cache }
    }

    pub async fn login(&self, req: UserLoginRequest) -> Result<LoginResponse, anyhow::Error> {
        let user = self
            .repos.user.find_one_by_email(&req.email).await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?
            .ok_or_else(|| anyhow!(CodeError::new_err_code(error_code::USER_NOT_EXIST)))?;

        if user.deleted_at.is_some() {
            return Err(anyhow!(CodeError::new_err_code(error_code::USER_NOT_EXIST)));
        }

        if !password::multi_password_verify(
            &user.algo,
            user.salt.as_deref().unwrap_or(""),
            &req.password,
            &user.password,
        ) {
            return Err(anyhow!(CodeError::new_err_code(error_code::USER_PASSWORD_ERROR)));
        }

        if !user.enable {
            return Err(anyhow!(CodeError::new_err_code(error_code::USER_DISABLED)));
        }

        if !req.identifier.is_empty() {
            let bind_svc = super::bind_device_service::BindDeviceService::new(
                self.repos.clone(), self.config.clone(), self.cache.clone(),
            );
            let _ = bind_svc.bind_device_to_user(&req.identifier, &req.ip, &req.user_agent, user.id).await;
        }

        let session_id = Uuid::new_v4().to_string();
        let login_type = if req.login_type.is_empty() { "email".to_string() } else { req.login_type };
        let (claims, seconds) = jwt::Claims::new(user.id, session_id.clone(), login_type);

        let token = jwt::generate_token(&claims, &self.config.jwt_auth.access_secret)
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, &e.to_string())))?;

        let session_key = format!("{}:{}", SESSION_ID_KEY, session_id);
        self.cache.set_ex(&session_key, &user.id.to_string(), seconds).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, &e.to_string())))?;

        Telemetry::login(&self.repos, user.id, "email", &req.ip, &req.user_agent, true).await;

        Ok(LoginResponse { token })
    }
}
