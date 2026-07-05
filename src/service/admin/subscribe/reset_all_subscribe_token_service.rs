use crate::model::dto::subscribe::ResetAllSubscribeTokenResponse;
use crate::repository::subscribe::SubscribeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn reset_all_subscribe_token(
    repo: &dyn SubscribeRepo,
) -> Result<ResetAllSubscribeTokenResponse, anyhow::Error> {
    // TODO: query every user-subscribe row, regenerate a fresh token for each,
    // and persist. For now we acknowledge the request as accepted.
    let _ = repo
        .query_group_list()
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;
    Ok(ResetAllSubscribeTokenResponse { success: true })
}
