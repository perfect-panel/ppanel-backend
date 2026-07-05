use crate::model::dto::server::{
    FilterServerListRequest, FilterServerListResponse, Server as ServerDto,
};
use crate::model::entity::node::Server;
use crate::repository::node::{NodeRepo, ServerFilter};
use result::code_error::CodeError;
use result::error_code;

pub async fn filter_server_list(
    repo: &dyn NodeRepo,
    req: FilterServerListRequest,
) -> Result<FilterServerListResponse, anyhow::Error> {
    let page = req.page.max(1) as i64;
    let size = req.size.max(1) as i64;
    let filter = ServerFilter {
        page,
        size,
        search: req.search.clone(),
        ..Default::default()
    };
    let (total, items) = repo
        .filter_server_list(&filter)
        .await
        .map_err(|e| anyhow::Error::new(CodeError::new_err_code_msg(
            error_code::DATABASE_QUERY_ERROR,
            &e.to_string(),
        )))?;
    let list = items.into_iter().map(server_to_dto).collect();
    Ok(FilterServerListResponse { total, list })
}

fn server_to_dto(s: Server) -> ServerDto {
    let now = chrono::Utc::now().timestamp_millis();
    let last = s.last_reported_at.unwrap_or(0);
    ServerDto {
        id: s.id,
        name: s.name,
        country: s.country,
        city: s.city,
        address: s.address,
        sort: s.sort,
        protocols: Vec::new(), // TODO: decode `s.protocols` (JSON string) into Vec<Protocol>
        last_reported_at: last,
        status: crate::model::dto::server::ServerStatus {
            cpu: 0.0,
            mem: 0.0,
            disk: 0.0,
            protocol: String::new(),
            online: Vec::new(),
            status: if last > now.saturating_sub(60_000) {
                "online".to_string()
            } else {
                "offline".to_string()
            },
        },
        created_at: s.created_at,
        updated_at: s.updated_at,
    }
}
