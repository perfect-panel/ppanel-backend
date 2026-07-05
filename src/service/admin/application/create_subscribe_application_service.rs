use chrono::Utc;

use crate::model::dto::{DownloadLink, SubscribeApplication};
use crate::model::dto::CreateSubscribeApplicationRequest;
use crate::model::entity::client::SubscribeApplication as SubscribeApplicationEntity;
use crate::repository::client::ClientRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn create_subscribe_application(
    repo: &dyn ClientRepo,
    req: CreateSubscribeApplicationRequest,
) -> Result<SubscribeApplication, anyhow::Error> {
    let link_json = serde_json::to_string(&req.download_link)
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::ERROR,
            &format!("failed to marshal download link: {}", e),
        )))?;

    let now = Utc::now().timestamp_millis();
    let entity = SubscribeApplicationEntity {
        id: 0,
        name: req.name,
        icon: req.icon,
        description: req.description,
        scheme: req.scheme.unwrap_or_default(),
        user_agent: req.user_agent,
        is_default: req.is_default,
        subscribe_template: Some(req.template),
        output_format: req.output_format,
        download_link: link_json,
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

    let dl: Option<DownloadLink> = if result.download_link.is_empty() {
        None
    } else {
        serde_json::from_str(&result.download_link).ok()
    };

    Ok(SubscribeApplication {
        id: result.id,
        name: result.name,
        description: result.description,
        icon: result.icon,
        scheme: Some(result.scheme),
        user_agent: result.user_agent,
        is_default: result.is_default,
        template: result.subscribe_template.unwrap_or_default(),
        output_format: result.output_format,
        download_link: dl,
        created_at: result.created_at,
        updated_at: result.updated_at,
    })
}
