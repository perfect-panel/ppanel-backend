use crate::model::dto::log::{
    GetMessageLogListRequest, GetMessageLogListResponse, MessageLog,
};
use crate::model::entity::log::{LogType, Message, SystemLog};
use crate::repository::log::LogRepo;
use result::code_error::CodeError;
use result::error_code;

/// Unified listing for EMAIL_MESSAGE (10) and MOBILE_MESSAGE (11) logs.
const MESSAGE_LOG_TYPES: [i16; 2] = [LogType::EMAIL_MESSAGE.0, LogType::MOBILE_MESSAGE.0];

pub async fn get_message_log_list(
    repo: &dyn LogRepo,
    req: GetMessageLogListRequest,
) -> Result<GetMessageLogListResponse, anyhow::Error> {
    // The request carries an `u8` type discriminator (0 = all, otherwise the
    // numeric LogType).  Map it to the matching i16 if it's one of the known
    // message constants; otherwise fall back to scanning both types.
    let type_filter: Option<i16> = match req.type_ {
        10 => Some(LogType::EMAIL_MESSAGE.0),
        11 => Some(LogType::MOBILE_MESSAGE.0),
        _ => None,
    };

    let mut list: Vec<MessageLog> = Vec::new();
    let mut total: i64 = 0;

    let types_to_scan: Vec<i16> = match type_filter {
        Some(t) => vec![t],
        None => MESSAGE_LOG_TYPES.to_vec(),
    };

    for type_ in types_to_scan {
        let (rows, page_total) = repo
            .filter_logs(
                req.page as i64,
                req.size as i64,
                Some(type_),
                None,
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

        total += page_total;
        list.extend(rows.into_iter().map(message_log_from_row));
    }

    Ok(GetMessageLogListResponse { total, list })
}

fn message_log_from_row(row: SystemLog) -> MessageLog {
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
