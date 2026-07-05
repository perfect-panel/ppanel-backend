use std::sync::Arc;

use chrono::Utc;

use crate::model::dto::user::UpdateUserBasiceInfoRequest;
use crate::model::entity::user::User;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn update_user_basic_info(
    repos: &Arc<Repositories>,
    req: UpdateUserBasiceInfoRequest,
) -> Result<User, anyhow::Error> {
    let mut user = repos
        .user
        .find_one_user(req.user_id)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })?;

    if let Some(avatar) = req.avatar {
        user.avatar = avatar;
    }
    if let Some(refer_code) = req.refer_code {
        user.refer_code = refer_code;
    }
    if let Some(pwd) = req.password {
        if !pwd.is_empty() {
            let hash = password::encode_password(&pwd)
                .map_err(|e| anyhow::Error::new(CodeError::new_err_msg(&e.to_string())))?;
            user.password = hash;
        }
    }
    user.balance = req.balance;
    user.commission = req.commission;
    user.referral_percentage = req.referral_percentage as i16;
    user.only_first_purchase = req.only_first_purchase;
    user.gift_amount = req.gift_amount;
    user.referer_id = req.referer_id;
    user.enable = req.enable;
    user.is_admin = req.is_admin;
    user.updated_at = Utc::now().timestamp_millis();

    repos.user.update_user(&user).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        ))
    })
}
