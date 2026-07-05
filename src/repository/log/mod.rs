use crate::model::entity::log::SystemLog;

#[async_trait::async_trait]
pub trait LogRepo: Send + Sync {
    async fn insert(&self, data: &SystemLog) -> Result<SystemLog, sqlx::Error>;
    async fn find_one(&self, id: i64) -> Result<SystemLog, sqlx::Error>;
    async fn filter_logs(
        &self,
        page: i64,
        size: i64,
        type_: Option<i16>,
        date: Option<&str>,
        object_id: Option<i64>,
        search: Option<&str>,
    ) -> Result<(Vec<SystemLog>, i64), sqlx::Error>;
    async fn find_first_by_date_type(&self, date: &str, type_: i16) -> Result<Option<SystemLog>, sqlx::Error>;
    async fn find_by_dates_type(&self, dates: &[String], type_: i16) -> Result<Vec<SystemLog>, sqlx::Error>;
}

pub mod pg;
pub mod mysql;
