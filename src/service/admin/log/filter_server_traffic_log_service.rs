use crate::model::dto::log::{
    FilterServerTrafficLogRequest, FilterServerTrafficLogResponse, ServerTrafficLog,
};
use crate::model::entity::log::{LogType, ServerTraffic};
use crate::repository::log::LogRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn filter_server_traffic_log(
    repo: &dyn LogRepo,
    req: FilterServerTrafficLogRequest,
) -> Result<FilterServerTrafficLogResponse, anyhow::Error> {
    let (rows, total) = repo
        .filter_logs(
            req.params.page as i64,
            req.params.size as i64,
            Some(LogType::SERVER_TRAFFIC.0),
            req.params.date.as_deref(),
            req.server_id,
            req.params.search.as_deref(),
        )
        .await
        .map_err(|e| {
            anyhow::Error::new(CodeError::new_err_code_msg(
                error_code::DATABASE_QUERY_ERROR,
                &e.to_string(),
            ))
        })?;

    let list = rows
        .into_iter()
        .map(|row| {
            let content: ServerTraffic = serde_json::from_str(&row.content).unwrap_or(ServerTraffic {
                server_id: row.object_id,
                upload: 0,
                download: 0,
                total: 0,
            });
            let date = row.date.clone().unwrap_or_default();
            ServerTrafficLog {
                server_id: content.server_id,
                upload: content.upload,
                download: content.download,
                total: content.total,
                date,
                details: true,
            }
        })
        .collect();

    Ok(FilterServerTrafficLogResponse { total, list })
}
