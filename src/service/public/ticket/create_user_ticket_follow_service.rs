//! `CreateUserTicketFollow` — insert follow, update ticket status.

use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::ticket::{CreateUserTicketFollowRequest, Follow};
use crate::model::entity::ticket::Follow as FollowEntity;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

use super::constant::TICKET_STATUS_PENDING_ADMIN;

pub struct CreateUserTicketFollowService {
    repos: Arc<Repositories>,
}

impl CreateUserTicketFollowService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn create(
        &self,
        user_id: i64,
        req: CreateUserTicketFollowRequest,
    ) -> Result<Follow, anyhow::Error> {
        // Verify ownership.
        let ticket = self
            .repos
            .ticket
            .find_one(req.ticket_id)
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

        let now = Utc::now().timestamp_millis();
        let entity = FollowEntity {
            id: 0,
            ticket_id: req.ticket_id,
            from: req.from.clone(),
            type_: req.type_ as i16,
            content: Some(req.content.clone()),
            created_at: now,
        };

        let result = self
            .repos
            .ticket
            .insert_follow(&entity)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_INSERT_ERROR,
                    &e.to_string(),
                ))
            })?;

        // Update ticket status to PENDING_ADMIN (user replied, waiting admin).
        let mut updated = ticket.clone();
        updated.status = TICKET_STATUS_PENDING_ADMIN;
        updated.updated_at = now;
        self.repos.ticket.update(&updated).await.map_err(|e| {
            anyhow!(CodeError::new_err_code_msg(
                error_code::DATABASE_UPDATE_ERROR,
                &e.to_string(),
            ))
        })?;

        Ok(Follow {
            id: result.id,
            ticket_id: result.ticket_id,
            from: result.from,
            type_: result.type_ as u8,
            content: result.content.unwrap_or_default(),
            created_at: result.created_at,
        })
    }
}
