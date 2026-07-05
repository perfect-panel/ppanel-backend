use std::sync::Arc;

use crate::model::dto::user::{GetDeviceListResponse, UserDevice};
use crate::model::entity::user::Device;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_user_subscribe_devices(
    repos: &Arc<Repositories>,
    user_id: i64,
) -> Result<GetDeviceListResponse, anyhow::Error> {
    let (list, total) = repos
        .user
        .query_device_list(user_id)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })?;

    let devices: Vec<UserDevice> = list
        .into_iter()
        .map(device_to_dto)
        .collect();

    Ok(GetDeviceListResponse { list: devices, total })
}

fn device_to_dto(d: Device) -> UserDevice {
    UserDevice {
        id: d.id,
        ip: d.ip,
        identifier: d.identifier,
        user_agent: d.user_agent.unwrap_or_default(),
        online: d.online,
        enabled: d.enabled,
        created_at: d.created_at,
        updated_at: d.updated_at,
    }
}
