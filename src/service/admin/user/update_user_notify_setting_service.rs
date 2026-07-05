use std::sync::Arc;

use chrono::Utc;

use crate::model::dto::user::UpdateUserNotifySettingRequest;
use crate::model::entity::user::User;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn update_user_notify_setting(
    repos: &Arc<Repositories>,
    req: UpdateUserNotifySettingRequest,
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

    user.enable_balance_notify = req.enable_balance_notify;
    user.enable_login_notify = req.enable_login_notify;
    user.enable_subscribe_notify = req.enable_subscribe_notify;
    user.enable_trade_notify = req.enable_trade_notify;
    user.updated_at = Utc::now().timestamp_millis();

    repos.user.update_user(&user).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        ))
    })
}
