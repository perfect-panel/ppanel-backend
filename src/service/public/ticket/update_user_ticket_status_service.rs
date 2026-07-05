//! `UpdateUserTicketStatus` — user closes their own ticket.

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::ticket::UpdateUserTicketStatusRequest;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

use super::constant::TICKET_STATUS_CLOSED;

pub struct UpdateUserTicketStatusService {
    repos: Arc<Repositories>,
}

impl UpdateUserTicketStatusService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn update(
        &self,
        user_id: i64,
        req: UpdateUserTicketStatusRequest,
    ) -> Result<(), anyhow::Error> {
        // Verify ownership before allowing any status change.
        let ticket = self
            .repos
            .ticket
            .find_one(req.id)
            .await
            .map_err(|e| match e {
                sqlx::Error::RowNotFound => {
                    anyhow!(CodeError::new_err_code(error_code::USER_NOT_EXIST))
                }
                _ => anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)),
            })?;

        if ticket.user_id != user_id {
            return Err(anyhow!(CodeError::new_err_code(error_code::INVALID_ACCESS)));
        }

        // Users can only close their own tickets.
        let new_status = req.status.map(|s| s as i16).unwrap_or(TICKET_STATUS_CLOSED);

        self.repos
            .ticket
            .update_ticket_status(req.id, user_id, new_status)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_UPDATE_ERROR,
                    &e.to_string(),
                ))
            })?;

        Ok(())
    }
}
