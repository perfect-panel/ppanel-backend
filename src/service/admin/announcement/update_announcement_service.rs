use chrono::Utc;

use crate::model::dto::UpdateAnnouncementRequest;
use crate::model::entity::announcement::Announcement as AnnouncementEntity;
use crate::repository::announcement::AnnouncementRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn update_announcement(
    repo: &dyn AnnouncementRepo,
    req: UpdateAnnouncementRequest,
) -> Result<(), anyhow::Error> {
    let mut entity: AnnouncementEntity = repo
        .find_one(req.id)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;

    entity.title = req.title;
    entity.content = req.content;
    if let Some(v) = req.show {
        entity.show = Some(v);
    }
    if let Some(v) = req.pinned {
        entity.pinned = Some(v);
    }
    if let Some(v) = req.popup {
        entity.popup = Some(v);
    }
    entity.updated_at = Utc::now().timestamp_millis();

    repo.update(&entity)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        )))?;

    Ok(())
}
