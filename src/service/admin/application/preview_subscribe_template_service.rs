use crate::model::dto::{PreviewSubscribeTemplateRequest, PreviewSubscribeTemplateResponse};
use crate::repository::client::ClientRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn preview_subscribe_template(
    repo: &dyn ClientRepo,
    req: PreviewSubscribeTemplateRequest,
) -> Result<PreviewSubscribeTemplateResponse, anyhow::Error> {
    let app = repo
        .find_one(req.id)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;

    let template = app.subscribe_template.unwrap_or_default();

    Ok(PreviewSubscribeTemplateResponse { template })
}
