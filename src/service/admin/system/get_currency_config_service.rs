use result::code_error::CodeError;
use result::error_code;

use crate::model::dto::CurrencyConfig;
use crate::repository::Repositories;

/// Read currency configuration from the `system` table (category = "currency").
pub async fn get_currency_config(
    repos: &Repositories,
) -> Result<CurrencyConfig, anyhow::Error> {
    let rows = repos
        .system
        .get_currency_config()
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string(),
            ))
        })?;
    let mut resp = CurrencyConfig {
        access_key: String::new(),
        currency_unit: String::new(),
        currency_symbol: String::new(),
    };
    for row in rows {
        match row.key.as_str() {
            "access_key" => resp.access_key = row.value,
            "currency_unit" => resp.currency_unit = row.value,
            "currency_symbol" => resp.currency_symbol = row.value,
            _ => {}
        }
    }
    Ok(resp)
}
