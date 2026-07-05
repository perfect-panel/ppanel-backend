//! `CreateUserTicket` — insert a new ticket with status=OPEN.

use std::sync::Arc;

use anyhow::anyhow;
use chrono::Utc;

use crate::model::dto::ticket::{CreateUserTicketRequest, Ticket};
use crate::model::entity::ticket::Ticket as TicketEntity;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

use super::constant::TICKET_STATUS_OPEN;

pub struct CreateUserTicketService {
    repos: Arc<Repositories>,
}

impl CreateUserTicketService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn create(
        &self,
        user_id: i64,
        req: CreateUserTicketRequest,
    ) -> Result<Ticket, anyhow::Error> {
        let now = Utc::now().timestamp_millis();
        let entity = TicketEntity {
            id: 0,
            title: req.title,
            description: Some(req.description),
            user_id,
            status: TICKET_STATUS_OPEN,
            created_at: now,
            updated_at: now,
        };

        let result = self
            .repos
            .ticket
            .insert(&entity)
            .await
            .map_err(|e| {
                anyhow!(CodeError::new_err_code_msg(
                    error_code::DATABASE_INSERT_ERROR,
                    &e.to_string(),
                ))
            })?;

        Ok(Ticket {
            id: result.id,
            title: result.title,
            description: result.description.unwrap_or_default(),
            user_id: result.user_id,
            follow: None,
            status: result.status as u8,
            created_at: result.created_at,
            updated_at: result.updated_at,
        })
    }
}
