use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::user::User;
use crate::model::entity::user::User as UserEntity;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct QueryUserInfoService {
    repos: Arc<Repositories>,
}

impl QueryUserInfoService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn query_user_info(&self, user_id: i64) -> Result<User, anyhow::Error> {
        let u = self
            .repos
            .user
            .find_one_user(user_id)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                ))
            })?;

        let methods = self
            .repos
            .user
            .find_user_auth_methods(user_id)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                ))
            })?;

        Ok(to_dto(&u, &methods))
    }
}

fn to_dto(u: &UserEntity, methods: &[crate::model::entity::user::AuthMethods]) -> User {
    let auth_methods = methods
        .iter()
        .map(|m| crate::model::dto::user::UserAuthMethod {
            auth_type: m.auth_type.clone(),
            auth_identifier: m.auth_identifier.clone(),
            verified: m.verified,
        })
        .collect();

    let rules: Vec<String> = u
        .rules
        .as_deref()
        .map(|r| {
            r.split(|c: char| c == ',' || c == '\n')
                .filter(|s| !s.is_empty())
                .map(|s| s.to_string())
                .collect()
        })
        .unwrap_or_default();

    User {
        id: u.id,
        avatar: u.avatar.clone(),
        balance: u.balance,
        commission: u.commission,
        referral_percentage: u.referral_percentage as u8,
        only_first_purchase: u.only_first_purchase,
        gift_amount: u.gift_amount,
        telegram: 0,
        refer_code: u.refer_code.clone(),
        referer_id: u.referer_id,
        enable: u.enable,
        is_admin: Some(u.is_admin),
        enable_balance_notify: u.enable_balance_notify,
        enable_login_notify: u.enable_login_notify,
        enable_subscribe_notify: u.enable_subscribe_notify,
        enable_trade_notify: u.enable_trade_notify,
        auth_methods,
        user_devices: Vec::new(),
        rules,
        created_at: u.created_at,
        updated_at: u.updated_at,
        deleted_at: u.deleted_at,
    }
}

#[allow(dead_code)]
fn _now_ms() -> i64 {
    Utc::now().timestamp_millis()
}
