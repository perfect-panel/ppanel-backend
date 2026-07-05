use chrono::Utc;

use crate::model::dto::subscribe::CreateSubscribeGroupRequest;
use crate::model::entity::subscribe::Group;
use crate::repository::subscribe::SubscribeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn create_subscribe_group(
    repo: &dyn SubscribeRepo,
    req: CreateSubscribeGroupRequest,
) -> Result<Group, anyhow::Error> {
    let now = Utc::now().timestamp_millis();
    let entity = Group {
        id: 0,
        name: req.name,
        description: req.description,
        created_at: now,
        updated_at: now,
    };
    repo.create_group(&entity)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_INSERT_ERROR,
            &e.to_string(),
        )))
}
