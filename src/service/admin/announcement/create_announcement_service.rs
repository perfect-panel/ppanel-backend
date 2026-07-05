use chrono::Utc;

use crate::model::dto::{Announcement, CreateAnnouncementRequest};
use crate::model::entity::announcement::Announcement as AnnouncementEntity;
use crate::repository::announcement::AnnouncementRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn create_announcement(
    repo: &dyn AnnouncementRepo,
    req: CreateAnnouncementRequest,
) -> Result<Announcement, anyhow::Error> {
    let now = Utc::now().timestamp_millis();
    let entity = AnnouncementEntity {
        id: 0,
        title: req.title,
        content: req.content,
        show: None,
        pinned: None,
        popup: None,
        created_at: now,
        updated_at: now,
    };
    let result = repo
        .insert(&entity)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_INSERT_ERROR,
            &e.to_string(),
        )))?;

    Ok(Announcement {
        id: result.id,
        title: result.title,
        content: result.content,
        show: result.show,
        pinned: result.pinned,
        popup: result.popup,
        created_at: result.created_at,
        updated_at: result.updated_at,
    })
}
