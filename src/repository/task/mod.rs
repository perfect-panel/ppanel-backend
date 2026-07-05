use crate::model::entity::task::Task;

#[derive(Debug, Default)]
pub struct TaskFilter {
    pub type_: i16,
    pub page: i64,
    pub size: i64,
    pub status: Option<i16>,
    pub scope: Option<i16>,
}

#[async_trait::async_trait]
pub trait TaskRepo: Send + Sync {
    async fn insert(&self, data: &Task) -> Result<Task, sqlx::Error>;
    async fn find_one(&self, id: i64) -> Result<Task, sqlx::Error>;
    async fn find_one_by_type(&self, id: i64, type_: i16) -> Result<Task, sqlx::Error>;
    async fn update(&self, data: &Task) -> Result<Task, sqlx::Error>;
    async fn update_status(&self, id: i64, status: i16) -> Result<u64, sqlx::Error>;
    async fn delete(&self, id: i64) -> Result<u64, sqlx::Error>;
    async fn query_task_list(
        &self,
        filter: &TaskFilter,
    ) -> Result<(i64, Vec<Task>), sqlx::Error>;
}

pub mod pg;
pub mod mysql;
