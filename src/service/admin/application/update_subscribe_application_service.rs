use chrono::Utc;

use crate::model::dto::SubscribeApplication;
use crate::model::dto::UpdateSubscribeApplicationRequest;
use crate::repository::client::ClientRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn update_subscribe_application(
    repo: &dyn ClientRepo,
    req: UpdateSubscribeApplicationRequest,
) -> Result<SubscribeApplication, anyhow::Error> {
    let mut entity = repo
        .find_one(req.id)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;

    entity.name = req.name;
    entity.icon = req.icon;
    entity.description = req.description;
    entity.scheme = req.scheme.unwrap_or_default();
    entity.user_agent = req.user_agent;
    entity.is_default = req.is_default;
    entity.subscribe_template = Some(req.template);
    entity.output_format = req.output_format;
    if let Some(ref dl) = req.download_link {
        let link_json = serde_json::to_string(dl)
            .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::ERROR,
                &format!("failed to marshal download link: {}", e),
            )))?;
        entity.download_link = link_json;
    }
    entity.updated_at = Utc::now().timestamp_millis();

    let result = repo
        .update(&entity)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_UPDATE_ERROR,
            &e.to_string(),
        )))?;

    let dl = if result.download_link.is_empty() {
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
