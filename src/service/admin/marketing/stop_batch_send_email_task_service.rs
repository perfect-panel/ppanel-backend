use crate::model::dto::marketing::StopBatchSendEmailTaskRequest;
use crate::model::entity::task::TaskType;
use crate::repository::task::TaskRepo;
use result::code_error::CodeError;
use result::error_code;

const STATUS_STOPPED: i16 = 3;

pub async fn stop_batch_send_email_task(
    repo: &dyn TaskRepo,
    req: StopBatchSendEmailTaskRequest,
) -> Result<(), anyhow::Error> {
    repo.find_one_by_type(req.id, TaskType::EMAIL.0 as i16)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string(),
            ))
        })?;

    repo.update_status(req.id, STATUS_STOPPED)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_UPDATE_ERROR,
                e.to_string(),
            ))
        })?;
    Ok(())
}
