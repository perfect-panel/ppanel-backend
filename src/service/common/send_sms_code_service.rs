use std::sync::Arc;

use anyhow::anyhow;
use rand::Rng;

use crate::cache::Cache;
use crate::config::cache_key::{AUTH_CODE_TELEPHONE_CACHE_KEY, SEND_COUNT_LIMIT_KEY_PREFIX, SEND_INTERVAL_KEY_PREFIX};
use crate::config::Config;
use crate::model::dto::auth::{SendCodeResponse, SendSmsCodeRequest};
use crate::queue::client::QueueClient;
use crate::queue::types::FORTHWITH_SEND_SMS;
use crate::repository::Repositories;
use result::code_error::CodeError;
use result::error_code;

pub struct SendSmsCodeService {
    repos: Arc<Repositories>,
    _config: Arc<Config>,
    cache: Arc<Cache>,
    queue: QueueClient,
}

impl SendSmsCodeService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>, cache: Arc<Cache>, queue: QueueClient) -> Self {
        Self {
            repos,
            _config: config,
            cache,
            queue,
        }
    }

    pub async fn send_code(
        &self,
        req: SendSmsCodeRequest,
    ) -> Result<SendCodeResponse, anyhow::Error> {
        let phone = format!("+{}{}", req.telephone_area_code, req.telephone);
        let cache_key = format!("{}:{}", AUTH_CODE_TELEPHONE_CACHE_KEY, phone);

        // Rate limit: interval check
        let interval_key = format!("{}{}", SEND_INTERVAL_KEY_PREFIX, phone);
        let last_send = self.cache.get(&interval_key).await.unwrap_or(None);
        if last_send.is_some() {
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::TOO_MANY_REQUESTS,
                "Please wait before requesting another code",
            )));
        }

        // Daily count limit
        let count_key = format!("{}{}", SEND_COUNT_LIMIT_KEY_PREFIX, phone);
        let daily_count: i64 = self.cache.get_int(&count_key).await.unwrap_or(None).unwrap_or(0);
        if daily_count >= 15 {
            return Err(anyhow!(CodeError::new_err_code_msg(
                error_code::TODAY_SEND_COUNT_EXCEEDS_LIMIT,
                "Daily send limit reached",
            )));
        }

        // Validate user state based on type
        let existing = self.repos.user.find_auth_method_by_open_id("mobile", &phone).await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::DATABASE_QUERY_ERROR, e.to_string())))?;
        match req.type_ {
            1 => {
                if existing.is_some() {
                    return Err(anyhow!(CodeError::new_err_code(error_code::USER_EXIST)));
                }
            }
            2 => {
                if existing.is_none() {
                    return Err(anyhow!(CodeError::new_err_code(error_code::USER_NOT_EXIST)));
                }
            }
            _ => {}
        }

        // Generate 6-digit code
        let code: String = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(6)
            .map(char::from)
            .collect();

        let payload = serde_json::json!({
            "code": code,
            "lastAt": chrono::Utc::now().timestamp_millis(),
        });

        // Store in Redis with 300s TTL
        self.cache
            .set_ex(&cache_key, &payload.to_string(), 300)
            .await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        self.cache
            .set_ex(&interval_key, "1", 60)
            .await
            .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;

        if daily_count == 0 {
            self.cache
                .set_ex(&count_key, "1", 86400)
                .await
                .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;
        } else {
            self.cache
                .incr(&count_key)
                .await
                .map_err(|e| anyhow!(CodeError::new_err_code_msg(error_code::ERROR, e.to_string())))?;
        }

        let sms_payload = serde_json::json!({
            "area_code": req.telephone_area_code,
            "telephone": req.telephone,
            "code": code,
        });
        if let Err(e) = self.queue.enqueue_json(FORTHWITH_SEND_SMS, &sms_payload).await {
            tracing::error!(telephone = %req.telephone, "failed to enqueue send-sms task: {e}");
        }

        Ok(SendCodeResponse {
            code: Some(code),
            status: true,
        })
    }
}
