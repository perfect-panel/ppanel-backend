use crate::model::dto::user::VersionResponse;

const VERSION: &str = env!("CARGO_PKG_VERSION");

pub async fn get_version() -> anyhow::Result<VersionResponse> {
    Ok(VersionResponse {
        version: VERSION.to_string(),
    })
}
