//! Raw Telegram Update JSON parsing and command dispatch.
//!
//! No external bot crate — parses `serde_json::Value` directly.

use serde::{Deserialize, Serialize};

/// Minimal Telegram Update (only fields we use).
#[derive(Debug, Deserialize)]
pub struct Update {
    pub update_id: i64,
    pub message: Option<Message>,
    pub callback_query: Option<CallbackQuery>,
}

#[derive(Debug, Deserialize)]
pub struct Message {
    pub message_id: i64,
    pub from: Option<User>,
    pub chat: Chat,
    pub text: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct CallbackQuery {
    pub id: String,
    pub from: User,
    pub message: Option<Message>,
    pub data: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub id: i64,
    pub first_name: String,
    pub username: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Chat {
    pub id: i64,
    #[serde(rename = "type")]
    pub chat_type: String,
}

/// Parsed bot command.
#[derive(Debug, PartialEq)]
pub enum BotCommand {
    Start,
    Bind { token: String },
    Traffic,
    Unknown(String),
}

/// Parse a `/command arg` text into a [`BotCommand`].
pub fn parse_command(text: &str) -> BotCommand {
    // Strip bot-mention suffix (e.g. `/start@MyBot`).
    let text = text.trim();
    let cmd_part = text.splitn(2, '@').next().unwrap_or(text);
    let mut parts = cmd_part.splitn(2, ' ');
    let cmd = parts.next().unwrap_or("").to_lowercase();
    let arg = parts.next().unwrap_or("").trim().to_string();

    match cmd.as_str() {
        "/start" => BotCommand::Start,
        "/bind" => BotCommand::Bind { token: arg },
        "/traffic" => BotCommand::Traffic,
        other => BotCommand::Unknown(other.to_string()),
    }
}

/// Outgoing Telegram sendMessage request body.
#[derive(Debug, Serialize)]
pub struct SendMessage {
    pub chat_id: i64,
    pub text: String,
    pub parse_mode: &'static str,
}

impl SendMessage {
    pub fn markdown(chat_id: i64, text: impl Into<String>) -> Self {
        Self {
            chat_id,
            text: text.into(),
            parse_mode: "Markdown",
        }
    }
}

/// Parse a Telegram Update from raw JSON bytes.
pub fn parse_update(body: &[u8]) -> Result<Update, serde_json::Error> {
    serde_json::from_slice(body)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_start() {
        assert_eq!(parse_command("/start"), BotCommand::Start);
    }

    #[test]
    fn test_parse_bind() {
        assert_eq!(
            parse_command("/bind abc123"),
            BotCommand::Bind { token: "abc123".into() }
        );
    }

    #[test]
    fn test_parse_bot_mention() {
        assert_eq!(parse_command("/start@MyBot"), BotCommand::Start);
    }

    #[test]
    fn test_parse_traffic() {
        assert_eq!(parse_command("/traffic"), BotCommand::Traffic);
    }

    #[test]
    fn test_parse_unknown() {
        matches!(parse_command("/help"), BotCommand::Unknown(_));
    }
}
