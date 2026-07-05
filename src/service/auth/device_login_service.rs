use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;
use uuid::Uuid;

use crate::cache::Cache;
use crate::config::cache_key::SESSION_ID_KEY;
use crate::config::Config;
use crate::model::dto::auth::{DeviceLoginRequest, LoginResponse};
use crate::model::entity::user::{AuthMethods, Device, User, UserSubscribe};
use crate::repository::Repositories;
use crate::service::telemetry::Telemetry;
use result::code_error::CodeError;
use result::error_code;

pub struct DeviceLoginService {
    repos: Arc<Repositories>,
    config: Arc<Config>,
    cache: Arc<Cache>,
}

impl DeviceLoginService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>, cache: Arc<Cache>) -> Self {
        Self { repos, config, cache }
    }

    pub async fn login(&self, req: DeviceLoginRequest) -> Result<LoginResponse, anyhow::Error> {
        if !self.config.device.enable {
            return Err(anyhow!(CodeError::new_err_msg("Device login is disabled")));
        }

        let user = match self.repos.user.find_one_device_by_identifier(&req.identifier).await {
            Ok(None)         => self.register_user_and_device(&req).await?,
            Ok(Some(device)) => self.repos.user.find_one_user(device.user_id).await
                .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?,
            Err(e)           => return Err(anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string()))),
        };

        let session_id = Uuid::new_v4().to_string();
        let (claims, seconds) = jwt::Claims::new(user.id, session_id.clone(), "device".to_string());

        let token = jwt::generate_token(&claims, &self.config.jwt_auth.access_secret)
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, &e.to_string())))?;

        let session_key = format!("{}:{}", SESSION_ID_KEY, session_id);
        self.cache.set_ex(&session_key, &user.id.to_string(), seconds).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, &e.to_string())))?;

        Telemetry::login(&self.repos, user.id, "device", &req.ip, &req.user_agent, true).await;

        Ok(LoginResponse { token })
    }

    async fn register_user_and_device(&self, req: &DeviceLoginRequest) -> Result<User, anyhow::Error> {
        let now = Utc::now().timestamp_millis();

        let mut user = User {
            id: 0, password: String::new(), algo: String::new(), salt: None,
            avatar: String::new(), balance: 0, refer_code: String::new(),
            referer_id: 0, commission: 0,
            referral_percentage: self.config.invite.referral_percentage as i16,
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

        let _ = self.repos.user.insert_auth_method(&AuthMethods {
            id: 0, user_id: user.id, auth_type: "device".to_string(),
            auth_identifier: req.identifier.clone(), verified: true,
            created_at: now, updated_at: now,
        }).await.map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_INSERT_ERROR, e.to_string())))?;

        let _ = self.repos.user.insert_device(&Device {
            id: 0, ip: req.ip.clone(), user_id: user.id,
            user_agent: Some(req.user_agent.clone()),
            identifier: req.identifier.clone(), online: false, enabled: true,
            created_at: now, updated_at: now,
        }).await.map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_INSERT_ERROR, e.to_string())))?;

        let trial_sub = if self.config.register.enable_trial {
            Some(self.activate_trial(user.id).await?)
        } else {
            None
        };

        if let Some(ref sub) = trial_sub {
            super::trial_cache::clear_trial_subscribe_cache(&self.cache, sub);
        }

        Telemetry::register(&self.repos, user.id, "device", &req.identifier, &req.ip, &req.user_agent).await;

        Ok(user)
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
