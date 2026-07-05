use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::user::{PreUnsubscribeRequest, PreUnsubscribeResponse};
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct PreUnsubscribeService {
    repos: Arc<Repositories>,
}

impl PreUnsubscribeService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn pre_unsubscribe(
        &self,
        _user_id: i64,
        req: PreUnsubscribeRequest,
    ) -> Result<PreUnsubscribeResponse, anyhow::Error> {
        let _ = self
            .repos
            .user
            .find_one_subscribe(req.id)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    e.to_string()
                ))
            })?;

        let now = Utc::now().timestamp_millis();
        let _ = now;

        Ok(PreUnsubscribeResponse { deduction_amount: 0 })
    }
}
