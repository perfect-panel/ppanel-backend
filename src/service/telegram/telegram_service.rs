//! Telegram bot orchestration service.
//!
//! Port of `server/internal/logic/telegram/telegramLogic.go`.

use std::sync::Arc;

use anyhow::anyhow;

use crate::config::Config;
use crate::model::entity::user::AuthMethods;
use crate::repository::Repositories;
use crate::service::telegram::bot::{parse_command, parse_update, BotCommand, SendMessage};
use crate::service::telegram::template;

pub struct TelegramService {
    pub repos: Arc<Repositories>,
    pub config: Arc<Config>,
}

impl TelegramService {
    pub fn new(repos: Arc<Repositories>, config: Arc<Config>) -> Self {
        Self { repos, config }
    }

    /// Process a raw Telegram Update payload.
    pub async fn handle_update(&self, body: &[u8]) -> Result<Option<SendMessage>, anyhow::Error> {
        let update = parse_update(body)
            .map_err(|e| anyhow!("parse telegram update: {e}"))?;

        let message = match update.message {
            Some(m) => m,
            None => return Ok(None),
        };

        let text = match &message.text {
            Some(t) => t.clone(),
            None => return Ok(None),
        };

        let tg_user = match &message.from {
            Some(u) => u,
            None => return Ok(None),
        };

        let chat_id = message.chat.id;
        let tg_id = tg_user.id;
        let bot_name = self.config.telegram.bot_name.clone();

        let reply = match parse_command(&text) {
            BotCommand::Start => {
                SendMessage::markdown(chat_id, template::welcome(&bot_name))
            }

            BotCommand::Bind { token } => {
                if token.is_empty() {
                    SendMessage::markdown(chat_id, template::bind_failed("token required"))
                } else {
                    match self.handle_bind(tg_id, &token).await {
                        Ok(email) => SendMessage::markdown(chat_id, template::bind_success(&email)),
                        Err(e) => SendMessage::markdown(chat_id, template::bind_failed(&e.to_string())),
                    }
                }
            }

            BotCommand::Traffic => {
                match self.handle_traffic(tg_id).await {
                    Ok(msg) => SendMessage::markdown(chat_id, msg),
                    Err(e) => SendMessage::markdown(chat_id, template::error_msg(&e.to_string())),
                }
            }

            BotCommand::Unknown(_) => return Ok(None),
        };

        Ok(Some(reply))
    }

    /// Bind a Telegram account to a ppanel account via subscribe token.
    async fn handle_bind(&self, tg_id: i64, token: &str) -> Result<String, anyhow::Error> {
        let user_subscribe = self
            .repos
            .user
            .find_one_subscribe_by_token(token)
            .await
            .map_err(|e| anyhow!("invalid token: {e}"))?;

        let user = self
            .repos
            .user
            .find_one_user(user_subscribe.user_id)
            .await
            .map_err(|e| anyhow!("user not found: {e}"))?;

        let auth = AuthMethods {
            id: 0,
            user_id: user.id,
            auth_type: "telegram".into(),
            auth_identifier: tg_id.to_string(),
            verified: true,
            created_at: 0,
            updated_at: 0,
        };
        self.repos
            .user
            .upsert_user_auth_method(&auth)
            .await
            .map_err(|e| anyhow!("upsert auth: {e}"))?;

        Ok(user.refer_code)
    }

    /// Return traffic info string for a bound Telegram user.
    async fn handle_traffic(&self, tg_id: i64) -> Result<String, anyhow::Error> {
        let auth = self
            .repos
            .user
            .find_auth_method_by_open_id("telegram", &tg_id.to_string())
            .await
            .map_err(|e| anyhow!("auth lookup: {e}"))?;

        let auth = match auth {
            Some(a) => a,
            None => return Ok(template::not_bound()),
        };

        let user = self
            .repos
            .user
            .find_one_user(auth.user_id)
            .await
            .map_err(|e| anyhow!("user not found: {e}"))?;

        let subscribes = self
            .repos
            .user
            .query_user_subscribe(user.id, &[1, 2])
            .await
            .map_err(|e| anyhow!("query subscribe: {e}"))?;

        if subscribes.is_empty() {
            return Ok(template::no_subscription());
        }

        let sub = &subscribes[0];
        let plan = self
            .repos
            .subscribe
            .find_one(sub.subscribe_id)
            .await
            .map_err(|e| anyhow!("find plan: {e}"))?;

        Ok(template::traffic_info(
            &user.refer_code,
            sub.upload,
            sub.download,
            plan.traffic,
            sub.expire_time,
        ))
    }

    /// Send a message via Telegram Bot API.
    pub async fn send_message(&self, msg: &SendMessage) -> Result<(), anyhow::Error> {
        let token = &self.config.telegram.bot_token;
        if token.is_empty() {
            return Ok(());
        }
        let url = format!("https://api.telegram.org/bot{token}/sendMessage");
        let client = reqwest::Client::new();
        let resp = client
            .post(&url)
            .json(msg)
            .send()
            .await
            .map_err(|e| anyhow!("telegram send: {e}"))?;

        if !resp.status().is_success() {
            let body = resp.text().await.unwrap_or_default();
            return Err(anyhow!("telegram API error: {body}"));
        }
        Ok(())
    }
}
