//! Shared helper for persisting a single `system` table row by `(category, key)`.

use result::code_error::CodeError;
use result::error_code;

use crate::repository::Repositories;

/// Update the `value` field of `system` row identified by `(category, key)`.
///
/// Mirrors the Go `updateConfigFields` helper — one row per call, no
/// transaction. Callers wrap multiple writes in their own transaction if
/// atomicity across rows is required.
pub async fn persist_config(
    repos: &Repositories,
    category: &str,
    key: &str,
    value: &str,
) -> Result<(), anyhow::Error> {
    repos
        .system
        .update_value_by_category_key(category, key, value)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_UPDATE_ERROR,
                e.to_string(),
            ))
        })?;
    Ok(())
}

/// Read every row in a category as raw `System` entities.
pub async fn fetch_category(
    repos: &Repositories,
    category: &str,
) -> Result<Vec<crate::model::entity::system::System>, anyhow::Error> {
    repos
        .system
        .get_by_category(category)
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string(),
            ))
        })
}
