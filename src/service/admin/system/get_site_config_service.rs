use result::code_error::CodeError;
use result::error_code;

use crate::model::dto::SiteConfig;
use crate::repository::Repositories;

/// Read site configuration from the `system` table (category = "site").
pub async fn get_site_config(
    repos: &Repositories,
) -> Result<SiteConfig, anyhow::Error> {
    let rows = repos
        .system
        .get_site_config()
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                e.to_string(),
            ))
        })?;
    let mut resp = SiteConfig {
        host: String::new(),
        site_name: String::new(),
        site_desc: String::new(),
        site_logo: String::new(),
        keywords: String::new(),
        custom_html: String::new(),
        custom_data: String::new(),
    };
    for row in rows {
        match row.key.as_str() {
            "host" => resp.host = row.value,
            "site_name" => resp.site_name = row.value,
            "site_desc" => resp.site_desc = row.value,
            "site_logo" => resp.site_logo = row.value,
            "keywords" => resp.keywords = row.value,
            "custom_html" => resp.custom_html = row.value,
            "custom_data" => resp.custom_data = row.value,
            _ => {}
        }
    }
    Ok(resp)
}
