use crate::model::entity::ticket::{Follow, Ticket};

#[derive(Debug, Clone, sqlx::FromRow)]
pub struct TicketDetails {
    pub id: i64,
    pub title: String,
    pub description: Option<String>,
    pub user_id: i64,
    pub status: i16,
    pub created_at: i64,
    pub updated_at: i64,
}

#[async_trait::async_trait]
pub trait TicketRepo: Send + Sync {
    async fn insert(&self, data: &Ticket) -> Result<Ticket, sqlx::Error>;
    async fn find_one(&self, id: i64) -> Result<Ticket, sqlx::Error>;
    async fn update(&self, data: &Ticket) -> Result<Ticket, sqlx::Error>;
    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn insert_follow(&self, data: &Follow) -> Result<Follow, sqlx::Error>;
    async fn find_follows_by_ticket(&self, ticket_id: i64) -> Result<Vec<Follow>, sqlx::Error>;
    async fn query_ticket_detail(&self, id: i64) -> Result<TicketDetails, sqlx::Error>;
    async fn query_ticket_list(
        &self,
        page: i64,
        size: i64,
        user_id: i64,
        status: Option<i16>,
        search: Option<&str>,
    ) -> Result<(i64, Vec<Ticket>), sqlx::Error>;
    async fn update_ticket_status(
        &self,
        id: i64,
        user_id: i64,
        status: i16,
    ) -> Result<u64, sqlx::Error>;
    async fn query_wait_reply_total(&self) -> Result<i64, sqlx::Error>;
}

pub mod pg;
pub mod mysql;
