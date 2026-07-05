use std::sync::Arc;

use crate::model::dto::user::{GetUserListRequest, GetUserListResponse};
use crate::repository::user::UserFilter;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_user_list(
    repos: &Arc<Repositories>,
    req: GetUserListRequest,
) -> Result<GetUserListResponse, anyhow::Error> {
    let mut filter = UserFilter::default();
    filter.search = req.search;
    filter.user_id = req.user_id;
    filter.subscribe_id = req.subscribe_id;
    filter.user_subscribe_id = req.user_subscribe_id;
    filter.unscoped = req.unscoped;

    let (total, list) = repos
        .user
        .query_page_list(req.page as i64, req.size as i64, &filter)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })?;

    let dto_list = list
        .into_iter()
        .map(|u| crate::model::dto::user::User {
            id: u.id,
            avatar: u.avatar,
            balance: u.balance,
            commission: u.commission,
            referral_percentage: u.referral_percentage as u8,
            only_first_purchase: u.only_first_purchase,
            gift_amount: u.gift_amount,
            telegram: 0,
            refer_code: u.refer_code,
            referer_id: u.referer_id,
            enable: u.enable,
            is_admin: Some(u.is_admin),
            enable_balance_notify: u.enable_balance_notify,
            enable_login_notify: u.enable_login_notify,
            enable_subscribe_notify: u.enable_subscribe_notify,
            enable_trade_notify: u.enable_trade_notify,
            auth_methods: Vec::new(),
            user_devices: Vec::new(),
            rules: Vec::new(),
            created_at: u.created_at,
            updated_at: u.updated_at,
            deleted_at: u.deleted_at,
        })
        .collect();

    Ok(GetUserListResponse { total, list: dto_list })
}
