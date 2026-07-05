use crate::model::dto::log::{
    FilterSubscribeTrafficRequest, FilterSubscribeTrafficResponse, UserSubscribeTrafficLog,
};
use crate::model::entity::log::{LogType, UserTraffic};
use crate::repository::log::LogRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn filter_user_subscribe_traffic_log(
    repo: &dyn LogRepo,
    req: FilterSubscribeTrafficRequest,
) -> Result<FilterSubscribeTrafficResponse, anyhow::Error> {
    let object_id = req.user_subscribe_id.or(req.user_id);
    let (rows, total) = repo
        .filter_logs(
            req.params.page as i64,
            req.params.size as i64,
            Some(LogType::SUBSCRIBE_TRAFFIC.0),
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

    let list = rows
        .into_iter()
        .map(|row| {
            let content: UserTraffic = serde_json::from_str(&row.content).unwrap_or(UserTraffic {
                subscribe_id: row.object_id,
                user_id: 0,
                upload: 0,
                download: 0,
                total: 0,
            });
            let date = row.date.clone().unwrap_or_default();
            UserSubscribeTrafficLog {
                subscribe_id: content.subscribe_id,
                user_id: content.user_id,
                upload: content.upload,
                download: content.download,
                total: content.total,
                date,
                details: true,
            }
        })
        .collect();

    Ok(FilterSubscribeTrafficResponse { total, list })
}
