use std::sync::Arc;

use chrono::Utc;

use crate::model::dto::user::CreateUserRequest;
use crate::model::entity::user::User;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn create_user(
    repos: &Arc<Repositories>,
    req: CreateUserRequest,
) -> Result<User, anyhow::Error> {
    let now = Utc::now().timestamp_millis();
    let hash = password::encode_password(&req.password)
        .map_err(|e| anyhow::Error::new(CodeError::new_err_msg(&e.to_string())))?;

    let entity = User {
        id: 0,
        password: hash,
        algo: String::new(),
        salt: None,
        avatar: String::new(),
        balance: req.balance,
        refer_code: req.refer_code,
        referer_id: 0,
        commission: req.commission,
        referral_percentage: req.referral_percentage as i16,
        only_first_purchase: req.only_first_purchase,
        gift_amount: req.gift_amount,
        enable: true,
        is_admin: req.is_admin,
        enable_balance_notify: false,
        enable_login_notify: false,
        enable_subscribe_notify: false,
        enable_trade_notify: false,
        rules: None,
        created_at: now,
        updated_at: now,
        deleted_at: None,
    };

    repos.user.insert_user(&entity).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_INSERT_ERROR,
            &e.to_string(),
        ))
    })
}
