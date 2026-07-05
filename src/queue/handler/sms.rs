use std::sync::Arc;

use asynq::error::Result;
use asynq::task::Task;

use crate::config::Config;
use crate::queue::service::sms::SendSmsLogic;
use crate::repository::Repositories;

pub async fn send_sms(task: Task, repos: Arc<Repositories>, config: Arc<Config>) -> Result<()> {
    SendSmsLogic::new(repos, config)
        .execute(task.get_payload())
        .await
        .map_err(|e| asynq::error::Error::other(e.to_string()))
}
