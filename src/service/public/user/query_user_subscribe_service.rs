use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::subscribe::UserSubscribe as UserSubscribeDto;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct QueryUserSubscribeService {
    repos: Arc<Repositories>,
}

impl QueryUserSubscribeService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn query_user_subscribe(
        &self,
        user_id: i64,
    ) -> Result<Vec<UserSubscribeDto>, anyhow::Error> {
        let statuses: Vec<i64> = vec![1, 2, 3];
        let rows = self
            .repos
            .user
            .query_user_subscribe(user_id, &statuses)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                ))
            })?;

        let list = rows
            .into_iter()
            .map(|r| UserSubscribeDto {
                id: r.id,
                user_id: r.user_id,
                order_id: r.order_id,
                subscribe_id: r.subscribe_id,
                subscribe: crate::model::dto::subscribe::Subscribe {
                    id: r.subscribe_id,
                    name: r.subscribe_name.unwrap_or_default(),
                    language: None,
                    description: None,
                    unit_price: 0,
                    unit_time: String::new(),
                    discount: Vec::new(),
                    replacement: 0,
                    inventory: 0,
                    traffic: r.traffic,
                    speed_limit: 0,
                    device_limit: 0,
                    quota: 0,
                    nodes: crate::model::dto::misc::StringInt64Slice::default(),
                    node_tags: Vec::new(),
                    show: false,
                    sell: false,
                    sort: 0,
                    deduction_ratio: 0,
                    allow_deduction: false,
                    reset_cycle: 0,
                    renewal_reset: false,
                    show_original_price: false,
                    created_at: 0,
                    updated_at: 0,
                },
                start_time: r.start_time,
                expire_time: r.expire_time,
                finished_at: r.finished_at.unwrap_or(0),
                reset_time: 0,
                traffic: r.traffic,
                download: r.download,
                upload: r.upload,
                token: r.token,
                status: r.status as u8,
                short: r.uuid,
                created_at: r.created_at,
                updated_at: r.updated_at,
            })
            .collect();

        Ok(list)
    }
}
