use crate::repository::node::{NodeFilter, NodeRepo};
use result::code_error::CodeError;
use result::error_code;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct FilterNodeListRequest {
    pub page: i64,
    pub size: i64,
    #[serde(default)]
    pub node_ids: Vec<i64>,
    #[serde(default)]
    pub server_ids: Vec<i64>,
    #[serde(default)]
    pub tags: Vec<String>,
    #[serde(default)]
    pub search: Option<String>,
    #[serde(default)]
    pub protocol: Option<String>,
    #[serde(default)]
    pub enabled: Option<bool>,
}

#[derive(Debug, Clone, serde::Serialize)]
pub struct FilterNodeListResponse {
    pub total: i64,
    pub list: Vec<crate::model::entity::node::Node>,
}

pub async fn filter_node_list(
    repo: &dyn NodeRepo,
    req: FilterNodeListRequest,
) -> Result<FilterNodeListResponse, anyhow::Error> {
    let page = req.page.max(1);
    let size = req.size.max(1);
    let filter = NodeFilter {
        page,
        size,
        node_ids: req.node_ids,
        server_ids: req.server_ids,
        tags: req.tags,
        search: req.search,
        protocol: req.protocol,
        enabled: req.enabled,
    };
    let (total, list) = repo
        .filter_node_list(&filter, false)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;
    Ok(FilterNodeListResponse { total, list })
}
