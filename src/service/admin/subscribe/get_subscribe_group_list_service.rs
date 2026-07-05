use crate::model::dto::subscribe::{
    GetSubscribeGroupListResponse, QuerySubscribeGroupListResponse, SubscribeGroup,
};
use crate::model::entity::subscribe::Group;
use crate::repository::subscribe::SubscribeRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn get_subscribe_group_list(
    repo: &dyn SubscribeRepo,
) -> Result<GetSubscribeGroupListResponse, anyhow::Error> {
    let (total, groups) = repo
        .query_group_list()
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;
    let list = groups.into_iter().map(group_to_dto).collect();
    Ok(GetSubscribeGroupListResponse { list, total })
}

/// Backwards-compatible alias used by older callers.
pub async fn query_subscribe_group_list(
    repo: &dyn SubscribeRepo,
) -> Result<QuerySubscribeGroupListResponse, anyhow::Error> {
    let resp = get_subscribe_group_list(repo).await?;
    Ok(QuerySubscribeGroupListResponse {
        list: resp.list,
        total: resp.total,
    })
}

fn group_to_dto(g: Group) -> SubscribeGroup {
    SubscribeGroup {
        id: g.id,
        name: g.name,
        description: g.description.unwrap_or_default(),
        created_at: g.created_at,
        updated_at: g.updated_at,
    }
}
