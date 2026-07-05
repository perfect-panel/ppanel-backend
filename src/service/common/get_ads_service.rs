use crate::model::dto::ads::{Ads, GetAdsResponse};
use crate::repository::Repositories;
use anyhow::anyhow;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_ads(repos: &Repositories) -> anyhow::Result<GetAdsResponse> {
    let (_total, items) = repos
        .ads
        .get_list_by_page(1, 200, Some(1), None)
        .await
        .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;

    let list: Vec<Ads> = items
        .into_iter()
        .map(|a| Ads {
            id: a.id as i32,
            title: a.title,
            type_: a.type_,
            content: a.content,
            description: a.description,
            target_url: a.target_url,
            start_time: a.start_time,
            end_time: a.end_time,
            status: a.status,
            created_at: a.created_at,
            updated_at: a.updated_at,
        })
        .collect();

    Ok(GetAdsResponse { list })
}
