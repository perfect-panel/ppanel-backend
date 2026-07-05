use asynq::error::Result;
use asynq::task::Task;
use std::sync::Arc;

use crate::config::Config;
use crate::queue::service::order::{ActivateOrderLogic, DeferCloseOrderLogic, OrderTaskPayload};
use crate::repository::Repositories;

pub async fn activate_order(
    task: Task,
    repos: Arc<Repositories>,
    config: Arc<Config>,
) -> Result<()> {
    let payload = decode_payload(&task)?;
    ActivateOrderLogic::new(repos, config)
        .execute(payload)
        .await
        .map_err(|err| asynq::error::Error::other(err.to_string()))
}

pub async fn defer_close_order(task: Task, repos: Arc<Repositories>) -> Result<()> {
    let payload = decode_payload(&task)?;
    DeferCloseOrderLogic::new(repos)
        .execute(payload)
        .await
        .map_err(|err| asynq::error::Error::other(err.to_string()))
}

fn decode_payload(task: &Task) -> Result<OrderTaskPayload> {
    task.get_payload_with_json()
}
