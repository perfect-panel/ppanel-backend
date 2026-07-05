use crate::model::dto::marketing::{
    GetPreSendEmailCountRequest, GetPreSendEmailCountResponse,
};
use crate::repository::user::{EmailRecipientFilter, UserRepo};
use result::code_error::CodeError;
use result::error_code;

pub async fn get_pre_send_email_count(
    repo: &dyn UserRepo,
    req: GetPreSendEmailCountRequest,
) -> Result<GetPreSendEmailCountResponse, anyhow::Error> {
    let filter = EmailRecipientFilter {
        scope: req.scope as i16,
        register_start_time: req.register_start_time.unwrap_or(0),
        register_end_time: req.register_end_time.unwrap_or(0),
    };
    let count = repo.count_email_recipients(&filter).await.map_err(|e| {
        anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            e.to_string(),
        ))
    })?;
    Ok(GetPreSendEmailCountResponse { count })
}
