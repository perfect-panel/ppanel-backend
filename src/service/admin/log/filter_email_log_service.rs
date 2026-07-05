use crate::model::dto::log::{
    FilterEmailLogResponse, MessageLog,
};
use crate::model::entity::log::{LogType, Message, SystemLog};
use crate::repository::log::LogRepo;
use result::code_error::CodeError;
use result::error_code;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterEmailLogRequest {
    pub page: i32,
    pub size: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

pub async fn filter_email_log(
    repo: &dyn LogRepo,
    req: FilterEmailLogRequest,
) -> Result<FilterEmailLogResponse, anyhow::Error> {
    let (rows, total) = repo
        .filter_logs(
            req.page as i64,
            req.size as i64,
            Some(LogType::EMAIL_MESSAGE.0),
            req.date.as_deref(),
            None,
            req.search.as_deref(),
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
        .map(message_log_from_system_log)
        .collect();

    Ok(FilterEmailLogResponse { total, list })
}

pub fn message_log_from_system_log(row: SystemLog) -> MessageLog {
    let msg: Message = serde_json::from_str(&row.content).unwrap_or(Message {
        to: String::new(),
        subject: None,
        content: serde_json::Value::Null,
        platform: String::new(),
        template: String::new(),
        status: 0,
    });
    let content_json = serde_json::to_value(&msg).unwrap_or(serde_json::Value::Null);
    let subject = msg.subject.unwrap_or_default();
    MessageLog {
        id: row.id,
        type_: row.type_ as u8,
        platform: msg.platform,
        to: msg.to,
        subject,
        content: content_json,
        status: msg.status as u8,
        created_at: row.created_at,
    }
}
