use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::user::UnbindDeviceRequest;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct UnbindDeviceService {
    repos: Arc<Repositories>,
}

impl UnbindDeviceService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn unbind_device(
        &self,
        _user_id: i64,
        req: UnbindDeviceRequest,
    ) -> Result<(), anyhow::Error> {
        self.repos
            .user
            .delete_device(req.id)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_DELETED_ERROR,
                    e.to_string()
                ))
            })?;

        Ok(())
    }
}
