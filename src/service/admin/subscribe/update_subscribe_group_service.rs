use chrono::Utc;

use crate::model::dto::subscribe::UpdateSubscribeGroupRequest;
use crate::model::entity::subscribe::Group;
use crate::repository::subscribe::SubscribeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn update_subscribe_group(
    repo: &dyn SubscribeRepo,
    req: UpdateSubscribeGroupRequest,
) -> Result<Group, anyhow::Error> {
    let existing = repo
        .query_group_list()
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?
        .1
        .into_iter()
        .find(|g| g.id == req.id)
        .ok_or_else(|| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::ERROR,
                "subscribe group not found",
            ))
        })?;
    let updated = Group {
        id: existing.id,
        name: req.name,
        description: req.description.or(existing.description),
        created_at: existing.created_at,
        updated_at: Utc::now().timestamp_millis(),
    };
    repo.update_group(&updated)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        )))
}
