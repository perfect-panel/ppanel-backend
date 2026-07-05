use std::sync::Arc;

use asynq::error::Result;
use asynq::task::Task;

use crate::config::Config;
use crate::queue::service::task::QuotaTaskLogic;
use crate::repository::Repositories;

pub async fn quota_task(task: Task, repos: Arc<Repositories>, config: Arc<Config>) -> Result<()> {
    QuotaTaskLogic::new(repos, config)
        .execute(task.get_payload())
        .await
        .map_err(|e| asynq::error::Error::other(e.to_string()))
}
