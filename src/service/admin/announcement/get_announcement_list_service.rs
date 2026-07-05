use crate::model::dto::{Announcement, GetAnnouncementListRequest, GetAnnouncementListResponse};
use crate::repository::announcement::AnnouncementRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_announcement_list(
    repo: &dyn AnnouncementRepo,
    req: GetAnnouncementListRequest,
) -> Result<GetAnnouncementListResponse, anyhow::Error> {
    let (total, items) = repo
        .get_list_by_page(
            req.page,
            req.size,
            req.show,
            req.pinned,
            req.popup,
            req.search.as_deref(),
        )
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;

    let list: Vec<Announcement> = items
        .into_iter()
        .map(|e| Announcement {
            id: e.id,
            title: e.title,
            content: e.content,
            show: e.show,
            pinned: e.pinned,
            popup: e.popup,
            created_at: e.created_at,
            updated_at: e.updated_at,
        })
        .collect();

    Ok(GetAnnouncementListResponse { total, list })
}
