use crate::model::entity::announcement::Announcement;

#[async_trait::async_trait]
pub trait AnnouncementRepo: Send + Sync {
    async fn insert(&self, data: &Announcement) -> Result<Announcement, sqlx::Error>;
    async fn find_one(&self, id: i64) -> Result<Announcement, sqlx::Error>;
    async fn update(&self, data: &Announcement) -> Result<Announcement, sqlx::Error>;
    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn get_list_by_page(
        &self,
        page: i64,
        size: i64,
        show: Option<bool>,
        pinned: Option<bool>,
        popup: Option<bool>,
        search: Option<&str>,
    ) -> Result<(i64, Vec<Announcement>), sqlx::Error>;
}

pub mod pg;
pub mod mysql;
