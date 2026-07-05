//! `GetUserTicketList` — paginated ticket list for the current user.

use std::sync::Arc;

use anyhow::anyhow;

use crate::model::dto::ticket::{GetUserTicketListRequest, GetUserTicketListResponse, Ticket};
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct GetUserTicketListService {
    repos: Arc<Repositories>,
}

impl GetUserTicketListService {
    pub fn new(repos: Arc<Repositories>) -> Self {
        Self { repos }
    }

    pub async fn list(
        &self,
        user_id: i64,
        req: GetUserTicketListRequest,
    ) -> Result<GetUserTicketListResponse, anyhow::Error> {
        let page = i64::from(req.page.max(1));
        let size = i64::from(req.size.max(1));
        let status = req.status.map(|s| s as i16);
        let search = req.search.as_deref();

        let (total, rows) = self
            .repos
            .ticket
            .query_ticket_list(page, size, user_id, status, search)
            .await
            .map_err(|_| anyhow!(CodeError::new_err_code(error_code::DATABASE_QUERY_ERROR)))?;

        let list = rows
            .into_iter()
            .map(|t| Ticket {
                id: t.id,
                title: t.title,
                description: t.description.unwrap_or_default(),
                user_id: t.user_id,
                follow: None,
                status: t.status as u8,
                created_at: t.created_at,
                updated_at: t.updated_at,
            })
            .collect();

        Ok(GetUserTicketListResponse { total, list })
    }
}
