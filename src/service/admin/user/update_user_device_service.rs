use std::sync::Arc;

use chrono::Utc;

use crate::model::entity::user::Device;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub async fn update_user_device(
    repos: &Arc<Repositories>,
    mut device: Device,
) -> Result<Device, anyhow::Error> {
    device.updated_at = Utc::now().timestamp_millis();
    repos.user.update_device(&device).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        ))
    })
}
