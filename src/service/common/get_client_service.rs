use crate::model::dto::subscribe::{GetSubscribeClientResponse, SubscribeClient};
use crate::repository::Repositories;
use anyhow::anyhow;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_client(repos: &Repositories) -> anyhow::Result<GetSubscribeClientResponse> {
    let items = repos
        .client
        .list()
        .await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

    let list: Vec<SubscribeClient> = items
        .into_iter()
        .map(|item| {
            let download_link = if item.download_link.is_empty() {
                None
            } else {
                serde_json::from_str(&item.download_link).ok()
            };
            SubscribeClient {
                id: item.id,
                name: item.name,
                description: item.description,
                icon: item.icon,
                scheme: Some(item.scheme),
                is_default: item.is_default,
                download_link,
            }
        })
        .collect();

    let total = list.len() as i64;
    Ok(GetSubscribeClientResponse { total, list })
}
