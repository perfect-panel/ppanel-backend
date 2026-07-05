use crate::model::dto::log::{
    FilterGiftLogRequest, FilterGiftLogResponse, GiftLog,
};
use crate::model::entity::log::{Gift, LogType};
use crate::repository::log::LogRepo;
use result::code_error::CodeError;
use result::error_code;

pub async fn filter_gift_log(
    repo: &dyn LogRepo,
    req: FilterGiftLogRequest,
) -> Result<FilterGiftLogResponse, anyhow::Error> {
    let (rows, total) = repo
        .filter_logs(
            req.params.page as i64,
            req.params.size as i64,
            Some(LogType::GIFT.0),
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
            let content: Gift = serde_json::from_str(&row.content).unwrap_or(Gift {
                type_: 0,
                order_no: String::new(),
                subscribe_id: 0,
                amount: 0,
                balance: 0,
                remark: None,
                timestamp: row.created_at,
            });
            GiftLog {
                type_: content.type_ as u16,
                user_id: row.object_id,
                order_no: content.order_no,
                subscribe_id: content.subscribe_id,
                amount: content.amount,
                balance: content.balance,
                remark: content.remark,
                timestamp: content.timestamp,
            }
        })
        .collect();

    Ok(FilterGiftLogResponse { total, list })
}
