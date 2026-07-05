use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::user::{GetDeviceListResponse, UserDevice};
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct GetDeviceListService {
    repos: Arc<Repositories>,
}

impl GetDeviceListService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn get_device_list(
        &self,
        user_id: i64,
    ) -> Result<GetDeviceListResponse, anyhow::Error> {
        let (devices, total) = self
            .repos
            .user
            .query_device_list(user_id)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                ))
            })?;

        let list = devices
            .into_iter()
            .map(|d| UserDevice {
                id: d.id,
                ip: d.ip,
                identifier: d.identifier,
                user_agent: d.user_agent.unwrap_or_default(),
                online: d.online,
                enabled: d.enabled,
                created_at: d.created_at,
                updated_at: d.updated_at,
            })
            .collect();

        Ok(GetDeviceListResponse { list, total })
    }
}
