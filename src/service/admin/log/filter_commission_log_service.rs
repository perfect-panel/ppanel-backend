use crate::model::dto::log::{
    CommissionLog, FilterCommissionLogRequest, FilterCommissionLogResponse,
};
use crate::model::entity::log::{Commission, LogType};
use crate::repository::log::LogRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn filter_commission_log(
    repo: &dyn LogRepo,
    req: FilterCommissionLogRequest,
) -> Result<FilterCommissionLogResponse, anyhow::Error> {
    let (rows, total) = repo
        .filter_logs(
            req.params.page as i64,
            req.params.size as i64,
            Some(LogType::COMMISSION.0),
            req.params.date.as_deref(),
            req.user_id,
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
            let content: Commission = serde_json::from_str(&row.content).unwrap_or(Commission {
                type_: 0,
                amount: 0,
                order_no: String::new(),
                timestamp: row.created_at,
            });
            CommissionLog {
                type_: content.type_ as u16,
                user_id: row.object_id,
                amount: content.amount,
                order_no: content.order_no,
                timestamp: content.timestamp,
            }
        })
        .collect();

    Ok(FilterCommissionLogResponse { total, list })
}
