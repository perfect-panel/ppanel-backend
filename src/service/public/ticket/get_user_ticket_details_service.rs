//! `GetUserTicketDetails` — fetch ticket and follows, verify user ownership.

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::ticket::{Follow, GetUserTicketDetailRequest, Ticket};
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct GetUserTicketDetailsService {
    repos: Arc<Repositories>,
}

impl GetUserTicketDetailsService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn get(
        &self,
        user_id: i64,
        req: GetUserTicketDetailRequest,
    ) -> Result<Ticket, anyhow::Error> {
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

        let follows = self
            .repos
            .ticket
            .find_follows_by_ticket(ticket.id)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;

        let follow_dtos: Vec<Follow> = follows
            .into_iter()
            .map(|f| Follow {
                id: f.id,
                ticket_id: f.ticket_id,
                from: f.from,
                type_: f.type_ as u8,
                content: f.content.unwrap_or_default(),
                created_at: f.created_at,
            })
            .collect();

        Ok(Ticket {
            id: ticket.id,
            title: ticket.title,
            description: ticket.description.unwrap_or_default(),
            user_id: ticket.user_id,
            follow: Some(follow_dtos),
            status: ticket.status as u8,
            created_at: ticket.created_at,
            updated_at: ticket.updated_at,
        })
    }
}
