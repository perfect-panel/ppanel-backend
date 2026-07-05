use crate::model::dto::subscribe::{
    GetSubscribeListRequest, GetSubscribeListResponse, SubscribeItem,
};
use crate::repository::subscribe::{FilterParams, SubscribeRepo};
use result::code_error::CodeError;
use result::error_code;

pub async fn get_subscribe_list(
    repo: &dyn SubscribeRepo,
    req: GetSubscribeListRequest,
) -> Result<GetSubscribeListResponse, anyhow::Error> {
    let mut params = FilterParams {
        page: req.page.max(1),
        size: req.size.max(1),
        search: req.search.clone(),
        language: req.language.clone(),
        ..Default::default()
    };
    params.normalize();
    let (total, list) = repo
        .filter_list(&mut params)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;
    let items: Vec<SubscribeItem> = list
        .into_iter()
        .map(|s| SubscribeItem {
            subscribe: crate::service::admin::subscribe::get_subscribe_details_service::entity_to_dto_pub(s),
            sold: 0, // TODO: aggregate sold count from order repo
        })
        .collect();
    Ok(GetSubscribeListResponse {
        list: items,
        total,
    })
}
