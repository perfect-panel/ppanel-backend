use crate::model::dto::Announcement;
use crate::model::entity::announcement::Announcement as AnnouncementEntity;
use crate::repository::announcement::AnnouncementRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_announcement(
    repo: &dyn AnnouncementRepo,
    id: i64,
) -> Result<Announcement, anyhow::Error> {
    let entity: AnnouncementEntity = repo
        .find_one(id)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;

    Ok(Announcement {
        id: entity.id,
        title: entity.title,
        content: entity.content,
        show: entity.show,
        pinned: entity.pinned,
        popup: entity.popup,
        created_at: entity.created_at,
        updated_at: entity.updated_at,
    })
}
