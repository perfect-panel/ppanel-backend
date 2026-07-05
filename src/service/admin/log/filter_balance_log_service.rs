use crate::config::Config;
use crate::model::dto::log::{
    BalanceLog, FilterBalanceLogRequest, FilterBalanceLogResponse,
    FilterEmailLogResponse, FilterMobileLogResponse, FilterTrafficLogDetailsRequest,
    FilterTrafficLogDetailsResponse, FilterSubscribeTrafficRequest, FilterSubscribeTrafficResponse,
    GetMessageLogListRequest, GetMessageLogListResponse, LogSetting,
};
use crate::model::entity::log::{Balance, LogType};
use crate::repository::log::LogRepo;
use result::code_error::CodeError;
use result::error_code;
use serde::{Deserialize, Serialize};

pub async fn filter_balance_log(
    repo: &dyn LogRepo,
    req: FilterBalanceLogRequest,
) -> anyhow::Result<FilterBalanceLogResponse> {
    let page = req.params.page.max(1) as i64;
    let size = req.params.size.max(1) as i64;
    let (rows, total) = repo
        .filter_logs(
            page,
            size,
            Some(LogType::BALANCE.0),
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
        .filter_map(|row| {
            let content: Balance = serde_json::from_str(&row.content).ok()?;
            Some(BalanceLog {
                type_: content.type_ as u16,
                user_id: row.object_id,
                amount: content.amount,
                order_no: content.order_no,
                balance: content.balance,
                timestamp: content.timestamp,
            })
        })
        .collect();
    Ok(FilterBalanceLogResponse { total, list })
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FilterEmailMobileLogRequest {
    pub page: i32,
    pub size: i32,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub search: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub date: Option<String>,
}

pub async fn filter_email_log(_repo: &dyn LogRepo, _req: FilterEmailMobileLogRequest) -> anyhow::Result<FilterEmailLogResponse> {
    Ok(FilterEmailLogResponse { total: 0, list: vec![] })
}

pub async fn filter_mobile_log(_repo: &dyn LogRepo, _req: FilterEmailMobileLogRequest) -> anyhow::Result<FilterMobileLogResponse> {
    Ok(FilterMobileLogResponse { total: 0, list: vec![] })
}

pub async fn filter_traffic_log_details(_repo: &dyn LogRepo, _req: FilterTrafficLogDetailsRequest) -> anyhow::Result<FilterTrafficLogDetailsResponse> {
    Ok(FilterTrafficLogDetailsResponse { total: 0, list: vec![] })
}

pub async fn filter_user_subscribe_traffic_log(_repo: &dyn LogRepo, _req: FilterSubscribeTrafficRequest) -> anyhow::Result<FilterSubscribeTrafficResponse> {
    Ok(FilterSubscribeTrafficResponse { total: 0, list: vec![] })
}

pub async fn get_log_setting(_config: &Config) -> anyhow::Result<LogSetting> {
    Ok(LogSetting { auto_clear: Some(false), clear_days: 30 })
}

pub async fn update_log_setting(_config: &Config, _req: LogSetting) -> anyhow::Result<()> {
    Ok(())
}

pub async fn get_message_log_list(_repo: &dyn LogRepo, _req: GetMessageLogListRequest) -> anyhow::Result<GetMessageLogListResponse> {
    Ok(GetMessageLogListResponse { total: 0, list: vec![] })
}
