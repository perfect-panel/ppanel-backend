use crate::model::entity::document::Document;

#[async_trait::async_trait]
pub trait DocumentRepo: Send + Sync {
    async fn insert(&self, data: &Document) -> Result<Document, sqlx::Error>;
    async fn find_one(&self, id: i64) -> Result<Document, sqlx::Error>;
    async fn update(&self, data: &Document) -> Result<Document, sqlx::Error>;
    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn query_detail(&self, id: i64) -> Result<Option<Document>, sqlx::Error>;
    async fn query_list(
        &self,
        page: i64,
        size: i64,
        tag: Option<&str>,
        search: Option<&str>,
    ) -> Result<(i64, Vec<Document>), sqlx::Error>;
    async fn get_all_visible(&self) -> Result<Vec<Document>, sqlx::Error>;
}

pub mod pg;
pub mod mysql;
