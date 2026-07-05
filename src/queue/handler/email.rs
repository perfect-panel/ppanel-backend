use std::sync::Arc;

use asynq::error::Result;
use asynq::task::Task;

use crate::config::Config;
use crate::queue::service::email::{BatchEmailLogic, SendEmailLogic};
use crate::repository::Repositories;

pub async fn send_email(task: Task, repos: Arc<Repositories>, config: Arc<Config>) -> Result<()> {
    SendEmailLogic::new(repos, config)
        .execute(task.get_payload())
        .await
        .map_err(|e| asynq::error::Error::other(e.to_string()))
}

pub async fn batch_email(task: Task, repos: Arc<Repositories>, config: Arc<Config>) -> Result<()> {
    BatchEmailLogic::new(repos, config)
        .execute(task.get_payload())
        .await
        .map_err(|e| asynq::error::Error::other(e.to_string()))
}
