use result::code_error::CodeError;
use result::error_code;

use crate::model::dto::InviteConfig;
use crate::repository::Repositories;

/// Read invite configuration from the `system` table (category = "invite").
pub async fn get_invite_config(
    repos: &Repositories,
) -> Result<InviteConfig, anyhow::Error> {
    let rows = repos
        .system
        .get_invite_config()
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string(),
            ))
        })?;
    let mut resp = InviteConfig {
        forced_invite: false,
        referral_percentage: 0,
        only_first_purchase: false,
    };
    for row in rows {
        match row.key.as_str() {
            "forced_invite" => resp.forced_invite = row.value == "true",
            "referral_percentage" => {
                if let Ok(v) = row.value.parse() {
                    resp.referral_percentage = v;
                }
            }
            "only_first_purchase" => resp.only_first_purchase = row.value == "true",
            _ => {}
        }
    }
    Ok(resp)
}
