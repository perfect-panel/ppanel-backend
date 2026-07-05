use crate::model::dto::log::{
    FilterTrafficLogDetailsRequest, FilterTrafficLogDetailsResponse, TrafficLogDetails,
};
use crate::model::entity::log::{LogType, Traffic};
use crate::repository::log::LogRepo;
use result::code_error::CodeError;
use result::error_code;

/// Aggregated traffic details across SUBSCRIBE / SUBSCRIBE_TRAFFIC / SERVER_TRAFFIC.
const TRAFFIC_LOG_TYPES: [i16; 3] = [
    LogType::SUBSCRIBE.0,
    LogType::SUBSCRIBE_TRAFFIC.0,
    LogType::SERVER_TRAFFIC.0,
];

pub async fn filter_traffic_log_details(
    repo: &dyn LogRepo,
    req: FilterTrafficLogDetailsRequest,
) -> Result<FilterTrafficLogDetailsResponse, anyhow::Error> {
    // Without a single type discriminator, the unified `filter_logs` call is
    // scoped to the broadest relevant bucket.  Callers wanting strict per-type
    // filtering should use the dedicated per-type services instead.
    let object_id = req.server_id.or(req.user_id);
    let mut list: Vec<TrafficLogDetails> = Vec::new();
    let mut total: i64 = 0;

    for type_ in TRAFFIC_LOG_TYPES.iter() {
        let (rows, page_total) = repo
            .filter_logs(
                req.params.page as i64,
                req.params.size as i64,
                Some(*type_),
                req.params.date.as_deref(),
                object_id,
                req.params.search.as_deref(),
            )
            .await
            .map_err(|e| {
                anyhow::Error::new(CodeError::new_err_code_msg(
                    error_code::DATABASE_QUERY_ERROR,
                    &e.to_string(),
                ))
            })?;

        total += page_total;
        for row in rows {
            let content: Traffic = serde_json::from_str(&row.content).unwrap_or(Traffic {
                download: 0,
                upload: 0,
            });
            list.push(TrafficLogDetails {
                id: row.id,
                server_id: req.server_id.unwrap_or(0),
                user_id: req.user_id.unwrap_or(row.object_id),
                subscribe_id: req.subscribe_id.unwrap_or(0),
                download: content.download,
                upload: content.upload,
                timestamp: row.created_at,
            });
        }
    }

    Ok(FilterTrafficLogDetailsResponse { total, list })
}
