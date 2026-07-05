use crate::model::dto::{Ads, GetAdsListRequest, GetAdsListResponse};
use crate::repository::ads::AdsRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_ads_list(
    repo: &dyn AdsRepo,
    req: GetAdsListRequest,
) -> Result<GetAdsListResponse, anyhow::Error> {
    let (total, items) = repo
        .get_list_by_page(
            req.page as i64,
            req.size as i64,
            req.status,
            req.search.as_deref(),
        )
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;

    let list = items
        .into_iter()
        .map(|e| Ads {
            id: e.id as i32,
            title: e.title,
            type_: e.type_,
            content: e.content,
            description: e.description,
            target_url: e.target_url,
            start_time: e.start_time,
            end_time: e.end_time,
            status: e.status,
            created_at: e.created_at,
            updated_at: e.updated_at,
        })
        .collect();

    Ok(GetAdsListResponse { total, list })
}
