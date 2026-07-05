use std::sync::Arc;

use asynq::error::Result;
use asynq::task::Task;

use crate::config::Config;
use crate::queue::service::subscription::CheckSubscriptionLogic;
use crate::repository::Repositories;

pub async fn check_subscription(
    task: Task,
    repos: Arc<Repositories>,
    config: Arc<Config>,
) -> Result<()> {
    let _ = task;
    CheckSubscriptionLogic::new(repos, config)
        .execute()
        .await
        .map_err(|e| asynq::error::Error::other(e.to_string()))
}
