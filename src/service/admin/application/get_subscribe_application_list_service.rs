use crate::model::dto::{DownloadLink, GetSubscribeApplicationListRequest, GetSubscribeApplicationListResponse, SubscribeApplication};
use crate::repository::client::ClientRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_subscribe_application_list(
    repo: &dyn ClientRepo,
    _req: GetSubscribeApplicationListRequest,
) -> Result<GetSubscribeApplicationListResponse, anyhow::Error> {
    let items = repo
        .list()
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;

    let list: Vec<SubscribeApplication> = items
        .into_iter()
        .map(|e| {
            let dl: Option<DownloadLink> = if e.download_link.is_empty() {
                None
            } else {
                serde_json::from_str(&e.download_link).ok()
            };
            SubscribeApplication {
                id: e.id,
                name: e.name,
                description: e.description,
                icon: e.icon,
                scheme: Some(e.scheme),
                user_agent: e.user_agent,
                is_default: e.is_default,
                template: e.subscribe_template.unwrap_or_default(),
                output_format: e.output_format,
                download_link: dl,
                created_at: e.created_at,
                updated_at: e.updated_at,
            }
        })
        .collect();

    let total = list.len() as i64;
    Ok(GetSubscribeApplicationListResponse { total, list })
}
