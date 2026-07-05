use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;
use uuid::Uuid;

use crate::cache::Cache;
use crate::config::cache_key::SESSION_ID_KEY;
use crate::config::Config;
use crate::model::dto::auth::{LoginResponse, TelephoneRegisterRequest};
use crate::model::entity::user::{AuthMethods, User, UserSubscribe};
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;
use result::code_error::CodeError;
use result::error_code;

pub struct TelephoneUserRegisterService {
    repos: Arc<Repositories>,
    config: Arc<Config>,
    cache: Arc<Cache>,
}

impl TelephoneUserRegisterService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>, cache: Arc<Cache>) -> Self {
        Self { repos, config, cache }
    }

    pub async fn register(&self, req: TelephoneRegisterRequest) -> Result<LoginResponse, anyhow::Error> {
        if self.config.register.stop_register {
            return Err(anyhow!(CodeError::new_err_code(error_code::STOP_REGISTER)));
        }

        if req.invite.is_empty() && self.config.invite.forced_invite {
            return Err(anyhow!(CodeError::new_err_code(error_code::INVITE_CODE_ERROR)));
        }

        let referer = if !req.invite.is_empty() {
            Some(
                self.repos.user.find_one_by_refer_code(&req.invite).await
                    .map_err(|_| anyhow!(CodeError::new_err_code(error_code::INVITE_CODE_ERROR)))?
                    .ok_or_else(|| anyhow!(CodeError::new_err_code(error_code::INVITE_CODE_ERROR)))?,
            )
        } else {
            None
        };

        let phone = format!("+{}{}", req.telephone_area_code, req.telephone);

        let cache_key = format!("{}:{}", crate::config::cache_key::AUTH_CODE_TELEPHONE_CACHE_KEY, phone);
        let cached = self.cache.get(&cache_key).await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::VERIFY_CODE_ERROR)))?
            .ok_or_else(|| anyhow!(CodeError::new_err_code(error_code::VERIFY_CODE_ERROR)))?;

        let payload: serde_json::Value = serde_json::from_str(&cached)
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::VERIFY_CODE_ERROR)))?;

        if payload["code"].as_str() != Some(&req.code) {
            return Err(anyhow!(CodeError::new_err_code(error_code::VERIFY_CODE_ERROR)));
        }
        let _ = self.cache.del(&cache_key).await;

        if let Some(u) = self.repos.user.find_auth_method_by_open_id("mobile", &phone).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?
        {
            let db_user = self.repos.user.find_one_user(u.user_id).await
                .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;
            if db_user.deleted_at.is_some() {
                return Err(anyhow!(CodeError::new_err_code(error_code::USER_DISABLED)));
            }
            return Err(anyhow!(CodeError::new_err_code(error_code::USER_EXIST)));
        }

        let now = Utc::now().timestamp_millis();
        let pwd = password::encode_password(&req.password)
            .map_err(|e| anyhow!(CodeError::new_err_msg(e.to_string())))?;

        let mut user = User {
            id: 0, password: pwd, algo: "default".to_string(), salt: None,
            avatar: String::new(), balance: 0, refer_code: String::new(),
            referer_id: referer.as_ref().map(|r| r.id).unwrap_or(0),
            commission: 0, referral_percentage: self.config.invite.referral_percentage as i16,
            only_first_purchase: self.config.invite.only_first_purchase,
            gift_amount: 0, enable: true, is_admin: false,
            enable_balance_notify: false, enable_login_notify: false,
            enable_subscribe_notify: false, enable_trade_notify: false,
            rules: None, created_at: now, updated_at: now, deleted_at: None,
        };

        user = self.repos.user.insert_user(&user).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_INSERT_ERROR, e.to_string())))?;

        let refer_code = format!("U{:X}", user.id);
        let mut update_user = user.clone();
        update_user.refer_code = refer_code;
        self.repos.user.update_user(&update_user).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_UPDATE_ERROR, e.to_string())))?;

        self.repos.user.insert_auth_method(&AuthMethods {
            id: 0, user_id: user.id, auth_type: "mobile".to_string(),
            auth_identifier: phone.clone(), verified: true,
            created_at: now, updated_at: now,
        }).await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_INSERT_ERROR, e.to_string())))?;

        let trial_sub = if self.config.register.enable_trial {
            Some(self.activate_trial(user.id).await?)
        } else {
            None
        };

        if let Some(ref sub) = trial_sub {
            super::trial_cache::clear_trial_subscribe_cache(&self.cache, sub);
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
        Telemetry::register(&self.repos, user.id, "mobile", &phone, &req.ip, &req.user_agent).await;

        Ok(LoginResponse { token })
    }

    async fn activate_trial(&self, user_id: i64) -> Result<UserSubscribe, anyhow::Error> {
        let sub = self.repos.subscribe.find_one(self.config.register.trial_subscribe).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

        let now = Utc::now();
        let expire_time = add_time(&self.config.register.trial_time_unit, self.config.register.trial_time, now);
        let token = format!("Trial-{}-{}", user_id, Uuid::new_v4());

        let user_sub = UserSubscribe {
            id: 0, user_id, order_id: 0, subscribe_id: sub.id,
            start_time: now.timestamp_millis(), expire_time: expire_time.timestamp_millis(),
            finished_at: None, traffic: sub.traffic, download: 0, upload: 0,
            token, uuid: Uuid::new_v4().to_string(), status: 1, note: String::new(),
            created_at: now.timestamp_millis(), updated_at: now.timestamp_millis(),
        };

        self.repos.user.insert_subscribe(&user_sub).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_INSERT_ERROR, e.to_string())))?;

        Ok(user_sub)
    }
}

fn add_time(unit: &str, amount: i64, from: chrono::DateTime<Utc>) -> chrono::DateTime<Utc> {
    match unit {
        "hour"  => from + chrono::Duration::hours(amount),
        "day"   => from + chrono::Duration::days(amount),
        "week"  => from + chrono::Duration::weeks(amount),
        "month" => from.checked_add_months(chrono::Months::new(amount as u32)).unwrap_or(from),
        "year"  => from.checked_add_months(chrono::Months::new((amount * 12) as u32)).unwrap_or(from),
        _       => from + chrono::Duration::days(amount),
    }
}
